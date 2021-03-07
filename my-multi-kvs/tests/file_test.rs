

use my_multi_kvs::{KvsEngine, Request};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Read, Seek, SeekFrom};
use std::fs;

#[test]
fn get_files_number_test(){
    KvsEngine::open("./tests/test");
}

#[test]
fn test_file_seek(){
    let path = "./tests/1.txt".to_string();
    let file = File::create(path.clone()).unwrap();
    let mut writer = BufWriter::new(file);
    let buf = "hello world".as_bytes();
    writer.write(buf).unwrap();
    writer.flush().unwrap();
}

#[test]
fn test_file_write(){
    let path = "./tests/1.txt".to_string();
    let mut file = OpenOptions::new().write(true).read(true).open(path).unwrap();
    file.seek(SeekFrom::End(0));
    let mut writer = BufWriter::new(
        file
    );

    let buf = "not bad".as_bytes();
    writer.write(buf).unwrap();
    let s = writer.buffer().len();
    println!("{}",s);
    writer.flush().unwrap();
}
#[test]
fn test_get_len(){
    let key = "123".to_string();
    let value= "123".to_string();
    let set = Request::Set {key,value};
    let path = "./tests/1.txt".to_string();
    let mut file = OpenOptions::new().write(true).read(true).open(path).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer,&set).unwrap();
    let len = writer.buffer().len();
    println!("the len is {}",len);
    writer.flush().unwrap();
}