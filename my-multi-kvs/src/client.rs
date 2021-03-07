use std::net::{TcpStream, ToSocketAddrs};
use crate::KvsResult;
use crate::common::{Request, SetResponse, GetResponse, RemoveResponse};
use std::io::{BufReader, BufWriter, Write, Read};
use serde::{Deserialize};
use serde_json::Deserializer;
use serde_json::de::IoRead;

pub struct Client{
    reader:Deserializer<IoRead<BufReader<TcpStream>>>,
    writer:BufWriter<TcpStream>
}

impl Client{
    pub fn connect<A:ToSocketAddrs>(addr:A) -> KvsResult<Self>{
        let tcp_stream = TcpStream::connect(addr)?;
        let writer = BufWriter::new(tcp_stream.try_clone()?);
        let reader = Deserializer::from_reader(BufReader::new(tcp_stream));
        Ok(Self{
            reader,
            writer
        })
    }

    pub fn get(&mut self,key:String) -> KvsResult<()>{
        let key_clone = key.clone();
        let g:Request = Request::Get {key};
        // 写入申请，向服务器发送请求
        self.writer.write(serde_json::to_string(&g).unwrap().as_bytes())?;
        self.writer.flush().unwrap();
        // 读取服务器发送回来的相应
        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp{
            GetResponse::Ok(value) =>{
                if let Some(value) = value{
                    println!("the value of {} is {}",key_clone,value);
                }else{
                    println!("the key {} doesn't have a value",key_clone)
                }
            }
            GetResponse::Err(msg) =>{
                println!("this connection has an error : {}",msg);
            }
        }
        Ok(())
    }

    pub fn set(&mut self,key:String,value:String) -> KvsResult<()>{
        let s = Request::Set {key ,value};
        self.writer.write(serde_json::to_string(&s).unwrap().as_bytes())?;
        self.writer.flush()?;

        let resp = SetResponse::deserialize(&mut self.reader)?;

        match resp {
            SetResponse::Ok(()) =>{
                println!("the key/value has been saved");
            }
            SetResponse::Err(msg) => {
                println!("this connection has an error : {}",msg);
            }
        }
        Ok(())
    }

    pub fn remove(&mut self,key:String) -> KvsResult<()>{
        let r = Request::Remove {key};
        self.writer.write(serde_json::to_string(&r).unwrap().as_bytes())?;
        self.writer.flush()?;

        let resp = RemoveResponse::deserialize(&mut self.reader)?;

        match resp {
            RemoveResponse::Ok(()) =>{
                println!("the key/value has been removed");
            },
            RemoveResponse::Err(msg) =>{
                println!("this connection has an error : {}",msg);
            }
        }
        Ok(())
    }

}