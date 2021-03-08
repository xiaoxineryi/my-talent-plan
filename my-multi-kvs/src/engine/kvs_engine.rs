use crate::engine::engine::Engine;
use crate::{KvsResult, KvsError};
use std::path::{PathBuf, Path};
use std::sync::{Arc, RwLock};
use std::collections::{BTreeMap, HashMap};
use std::{fs, io};
use std::io::{BufWriter, BufReader, Seek, Read, Write, SeekFrom, BufRead};
use std::fs::{File, OpenOptions, read};
use std::borrow::Borrow;
use crate::common::Request;
use serde_json::Deserializer;
use std::time::Duration;

/// 多线程并发实现数据库读写操作
#[derive(Clone)]
pub struct KvsEngine{
    path:Arc<PathBuf>,
    index:Arc<RwLock<HashMap<String,CommandPos>>>,
    manager:Arc<KvsManager>
}

struct CommandPos{
    gen:u64,
    pos:u64,
    len:u64
}
impl CommandPos{
    pub fn new(gen:u64,pos:u64,len:u64) -> Self{
        Self{
            gen,
            pos,
            len
        }
    }
}

struct KvsManager{
    path :Arc<PathBuf>,
    manager:Arc<BTreeMap<u64, RwLock<File>>>,
    temp_page : u64
}
impl KvsManager{
    pub fn new(path:PathBuf,manager:BTreeMap<u64,RwLock<File>>,page_id:u64) -> KvsResult<Self>{
        Ok(Self{
            path: Arc::new(path),
            manager: Arc::new(manager),
            temp_page: page_id
        })
    }
    /// 初始时加载文件，更新对应的index索引
    pub fn load(&self, number:&u64, index: &mut HashMap<String, CommandPos>) -> KvsResult<()>{
        let file = self.manager.get(number).ok_or_else(|| KvsError::None)?.read().unwrap();
        let file = file.try_clone()?;
        let mut reader = BufReader::new(file);
        // 将文件指针移动至文件头
        let mut pos = reader.seek(SeekFrom::Start(0))?;

        // 解析遍历所有的记录，并且记录到index中
        let mut stream = Deserializer::from_reader(reader).into_iter::<Request>();
        while let Some(request) =stream.next(){
            let next_pos = stream.byte_offset();
            let request = request?;
            // 若是set则保存 若是remove则清除对应的index
            match request {
                Request::Set {key,..} =>{
                    index.insert(key,CommandPos::new(*number,pos,next_pos as u64 -pos));
                }
                Request::Remove {key} =>{
                    index.remove(&key);
                }
                _ => {panic!("解析得到未定义数据类型")}
            }
            pos = next_pos as u64;
        }
        Ok(())
    }

    /// 获取键值对
    pub fn get(&self,command_pos:&CommandPos) -> KvsResult<Option<String>>{
        // 使用命令位置获得对应的数据
        let file = self.manager.get(&command_pos.gen).unwrap().read().unwrap();
        let mut reader = BufReader::new(file.try_clone()?);
        // 指针移动到对应位置
        reader.seek(SeekFrom::Start(command_pos.pos))?;
        let mut reader = reader.take(command_pos.len);
        let buf = reader.fill_buf().unwrap();
        let e :Request= serde_json::from_reader(&mut reader)?;
        match e{
            Request::Set {key,value} =>{
                Ok(Some(value))
            }
            Request::Remove {key}=>{
                Ok(None)
            }
            _ =>{
                Ok(None)
            }
        }
    }

    /// 插入键值对，对对应的文件用写锁,返回插入后指令初始位置和长度
    pub fn set(&self,key:String,value:String) -> KvsResult<CommandPos>{
        // 加写锁，键值对写入文件
        let file = self.manager.get(&self.temp_page).ok_or_else(||KvsError::None)?.write().unwrap();
        let mut writer = BufWriter::new(file.try_clone()?);
        let pos= writer.seek(SeekFrom::End(0))?;
        let set = Request::Set{key,value};
        serde_json::to_writer(&mut writer,&set)?;
        let len = writer.buffer().len();
        writer.flush()?;

        Ok(CommandPos::new(self.temp_page,pos,len as u64))
    }
    /// 移除键值对，将记录插入文件
    pub fn remove(&self,key:String) -> KvsResult<CommandPos>{
        // 加写锁
        let file = self.manager.get(&self.temp_page).ok_or_else(|| KvsError::None)?.write().unwrap();
        let mut writer = BufWriter::new(file.try_clone()?);
        let pos = writer.seek(SeekFrom::End(0))?;
        let remove = Request::Remove {key };
        serde_json::to_writer(&mut writer,&remove)?;
        let len = writer.buffer().len();
        writer.flush()?;

        Ok(CommandPos::new(self.temp_page,pos,len as u64))
    }
}

impl KvsEngine{
    /// 检查所有已创建的文件，将之记录到读写器中，并且新建新的文件作为本次写入的文件。
    pub fn open(path: impl Into<PathBuf>) -> KvsResult<Self>{
        // 检查对应文件路径是否已经创建
        let path = path.into();
        fs::create_dir_all(&*path)?;

        // 检查有多少已创建的数据文件
        let files_number = get_files_number(&path)?;
        // 将已创建文件的文件指针保存
        let mut index:HashMap<String,CommandPos>= HashMap::new();
        let mut managers = BTreeMap::new();
        for &ind in files_number.as_slice(){
            let file_name = log_file_path(&path,ind);
            let file = RwLock::new(OpenOptions::new().read(true).write(true).open(file_name)?);
            managers.insert(ind,file);
        }
        // 创建新文件，并且加入读写器
        let last = files_number.last().unwrap_or(&0);
        let new = last + 1;
        create_new_file(&path,&mut managers,new);

        let mut manager = KvsManager::new(path.clone(), managers, new)?;

        // 遍历所有文件，得到所有的index
        for ind in files_number.as_slice(){
            manager.load(ind,&mut index);
        }

        Ok(
            Self{
                path:Arc::new(path),
                index:Arc::new(RwLock::new(index)),
                manager: Arc::new(manager)
            }
        )
    }
}


// 每次重启服务器时，新建一个文件存储本次所有记录
pub fn create_new_file(path:&PathBuf,manager:&mut BTreeMap<u64,RwLock<File>>,new:u64) -> KvsResult<()>{
    let path = path.join(format!("{}.log",new));
    let file = OpenOptions::new().write(true).
        create(true).read(true).open(path)?;
    manager.insert(new,RwLock::new(file));
    Ok(())
}

impl Engine for KvsEngine{
    fn get(&self, key: String) -> KvsResult<Option<String>> {
        // 先根据index查询得到对应的命令地址
        let e = self.index.read().unwrap();
        let pos = e.get(&key);
        if let Some(command) = pos{
            let value= self.manager.get(command)?;
            Ok(value)
        }else {
            Ok(None)
        }
    }
    /// 将键值对插入文件中，并且保存索引
    fn set(&mut self, key: String, value: String) -> KvsResult<()> {
        // 将数据先进行序列化
        let command_pos = self.manager.set(key.clone(),value)?;
        self.index.write().unwrap().insert(key,command_pos);
        Ok(())
    }

    fn remove(&self, key: String) -> KvsResult<()> {
        let command_pos = self.manager.remove(key.clone())?;
        /// 要是直接合成一句的话，write就直接执行完就释放了，所以线程睡眠看起来没有起到效果
        /// self.index.write().unwrap().insert(key,command_pos)?;
        /// std::thread::sleep(Duration::from_secs(5));
        let mut e =self.index.write().unwrap();
        std::thread::sleep(Duration::from_secs(5));
        e.insert(key,command_pos);
        Ok(())
    }
}

/// 获取目标目录下所有记录文件的尾号
fn get_files_number(path:&Path) -> KvsResult<Vec<u64>>{
    // 读取目录下所有文件
    let mut files_number:Vec<u64> = fs::read_dir(&path)?
        .flat_map(|f| -> KvsResult<_> {Ok(f?.path())})
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        // 检查后缀名
        .filter_map(|path|{
            path.file_name().
            and_then(|file_name| file_name.to_str())
            .map(|s| s.trim_end_matches(".log"))
                .map(|s| s.parse::<u64>().ok())
        })
        .flatten()
        .collect();

    files_number.sort_unstable();
    Ok(files_number)
}

/// 将路径拼接得到对应文件名
fn log_file_path(path: &Path,gen:u64) -> PathBuf{
    path.join(format!("{}.log",gen))
}