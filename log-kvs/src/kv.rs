use std::path::{PathBuf, Path};
use std::collections::{HashMap, BTreeMap};
use std::fs::{File, OpenOptions};
use std::io::{Write, Seek, BufReader, Read, BufWriter, SeekFrom};
use crate::error::{KvResult, KvsError};
use crate::command::Command;
use std::{io, fs};
use failure::_core::ops::Range;
use std::ffi::OsStr;
use serde_json::Deserializer;

const COMPACTION_THRESHOLD:u64 = 1024 * 1024;

pub struct KvStore{
    // 数据库日志对应的路径位置
    path:PathBuf,
    // 对应每个日志记录的序号以及在文件中所在的位置
    readers:HashMap<u64,BufReaderWithPos<File>>,
    // 对应该日志文件的写操作指针，每次都会指向文件末尾
    writer:BufWriterWithPos<File>,
    // 当前文件指针所对应的序号
    current_gen:u64,
    // 指令的对应存放位置
    index:BTreeMap<String,CommandPos>,
    // 对应的需要删减的大小
    uncompacted:u64
}

/// 含有对应记录的起始位置等信息
struct BufReaderWithPos<R:Read + Seek>{
    reader:BufReader<R>,
    pos:u64
}

struct BufWriterWithPos<W:Write + Seek>{
    writer:BufWriter<W>,
    pos:u64
}

/// 执行指令的位置
struct CommandPos{
    gen:u64,
    pos:u64,
    len:u64
}

impl KvStore {
    pub fn open(path:impl Into<PathBuf>) -> KvResult<Self>{
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let gen_list = sorted_gen_list(path.as_path())?;
        let mut uncompacted = 0 as u64;

        for &gen in & gen_list{
            let mut reader =
                BufReaderWithPos::new(File::open(log_path(path.as_path(),gen))?)?;
            uncompacted += load(gen,&mut reader,&mut index)?;
            readers.insert(gen,reader);
        }

        let current_gen = gen_list.last().unwrap_or(&0) + 1;
        let writer =new_log_file(&path,current_gen,&mut readers)?;

        Ok(KvStore{
            path,
            readers,
            writer,
            current_gen,
            index,
            uncompacted
        })
    }


    pub fn set(&mut self,key:String,value:String) -> KvResult<()>{
        let command = Command::set(key,value);
        let pos = self.writer.pos;
        // 将对应的value数据持久化为json格式放入数据流中
        serde_json::to_writer(&mut self.writer,&command)?;
        self.writer.flush();

        if let Command::Set {key,..} =command{
            if let Some(old_cmd) = self
                .index
                .insert(key,(self.current_gen,pos..self.writer.pos).into())
            {
                self.uncompacted += old_cmd.len
            }
        }

        if self.uncompacted > COMPACTION_THRESHOLD{
            self.compact();
        }
        Ok(())
    }

    pub fn get(&mut self, key:String) -> KvResult<Option<String>>{
        // 如果在索引中找到了对应的Key
        if let Some(cmd_pos) =self.index.get(&key){
            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("can't find log reader");
            reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            let com_reader = reader.take(cmd_pos.len);
            if let Command::Set {key,..} =serde_json::from_reader(com_reader)?{
                Ok(Some(key))
            }else{
                Err(KvsError::UnexpectedCommandType)
            }
        }else {
            Ok(None)
        }
    }

    pub fn remove(&mut self,key:String) -> KvResult<()>{
        if self.index.contains_key(&key){
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer,&cmd)?;
            self.writer.flush()?;
            if let Command::Remove {key} = cmd{
                let old_cmd = self.index.remove(&key).expect("key not found");
                self.uncompacted += old_cmd.len;
            }
            Ok(())
        }else{
            Err(KvsError::KeyNotFound)
        }

    }

    fn compact(&mut self) -> KvResult<()>{
        unimplemented!()
    }
}

fn new_log_file(
    path:&Path,
    gen : u64,
    readers : &mut HashMap<u64,BufReaderWithPos<File>>
) -> KvResult<BufWriterWithPos<File>>{
    let path =log_path(&path,gen);
    let writer = BufWriterWithPos::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;
    readers.insert(gen,BufReaderWithPos::new(File::open(&path)?)?);
    Ok(writer)
}

fn sorted_gen_list(path:&Path) -> KvResult<Vec<u64>>{
    let mut gen_list :Vec<u64> =fs::read_dir(&path)?
        .flat_map(|res| -> KvResult<_> {Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some(".log".as_ref()))
        .flat_map(|path|{
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    gen_list.sort_unstable();
    Ok(gen_list)
}

fn log_path(dir:&Path,gen:u64) -> PathBuf{
    dir.join(format!("{}.log",gen))
}

fn load(gen:u64,
        reader:&mut BufReaderWithPos<File>,
        index:&mut BTreeMap<String,CommandPos>) -> KvResult<u64>{
    let mut pos  = reader.seek(SeekFrom::Start(0))?;
    let mut stream =Deserializer::from_reader(reader).into_iter::<Command>();
    let mut uncompacted = 0 as u64;
    while let Some(cmd) = stream.next(){
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set {key,..} =>{
                if let Some(old_cmd) = index.insert(key,(gen,pos..new_pos).into()){
                    uncompacted += old_cmd.len;
                }
            },
            Command::Remove {key} =>{
                if let Some(old_cmd) = index.remove(&key){
                    uncompacted += old_cmd.len;
                }
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    };
    Ok(uncompacted)
}

impl <R: Read + Seek> BufReaderWithPos<R>{
    fn new(mut inner:R) -> KvResult<Self>{
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos{
            reader: BufReader::new(inner),
            pos
        })
    }
}

impl <R: Read + Seek> Read for BufReaderWithPos<R>{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl <R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

// 对自定义的写类进行封装，添加方法
impl <W: Write + Seek> BufWriterWithPos<W>{
    fn new(mut inner:W) -> KvResult<Self>{
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos{ writer: BufWriter::new(inner), pos })
    }
}

impl <W:Write + Seek> Write for BufWriterWithPos<W>{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) ->io::Result<()> {
        self.writer.flush()
    }
}

impl From<(u64,Range<u64>)> for CommandPos{
    fn from((gen,range):(u64, Range<u64>)) -> Self {
        CommandPos{
            gen,
            pos: range.start,
            len: range.end - range.start
        }
    }
}