use std::net::{SocketAddr, ToSocketAddrs, TcpStream};
use crate::{KvsResult, KvsError};
use std::io::{BufWriter, BufReader, Write};
use serde_json::Deserializer;
use serde_json::de::IoRead;
use crate::common::{Request, GetResponse, SetResponse, RemoveResponse};
use serde::Deserialize;

pub struct Client{
    writer:BufWriter<TcpStream>,
    reader:Deserializer<IoRead<BufReader<TcpStream>>>
}

impl Client {
    /// the client connect to the server
    pub fn connect<A:ToSocketAddrs>(addr:A) -> KvsResult<Self>{
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(Client{
            writer: BufWriter::new(tcp_writer),
            reader:Deserializer::from_reader(BufReader::new(tcp_reader))
        })
    }
    /// the client send a request to the server to get some value and deal the response of the server
    pub fn get(&mut self,key:String) -> KvsResult<Option<String>>{
        serde_json::to_writer(&mut self.writer,&Request::Get {key});
        self.writer.flush()?;

        let resp = GetResponse::deserialize(&mut self.reader)?;

        match resp{
            GetResponse::Ok(value) => {Ok(value)}
            GetResponse::Err(msg) => {
                Err(KvsError::StringError(msg))
            }
        }
    }
    /// the client send a request to set the key/value and deal with the response
    pub fn set(&mut self,key:String,value:String) -> KvsResult<()>{
        serde_json::to_writer(&mut self.writer,&Request::Set {key,value});
        self.writer.flush()?;

        let resp = SetResponse::deserialize(&mut self.reader)?;

        match resp {
            SetResponse::Ok(_) => {
                Ok(())
            }
            SetResponse::Err(msg) => {
                Err(KvsError::StringError(msg))
            }
        }
    }
    /// remove the key
    pub fn remove(&mut self,key:String) -> KvsResult<()>{
        serde_json::to_writer(&mut self.writer,&Request::Remove {key})?;
        self.writer.flush()?;

        let resp = RemoveResponse::deserialize(&mut self.reader)?;
        match resp{
            RemoveResponse::Ok(_) => {
                Ok(())
            }
            RemoveResponse::Err(msg) => {
                Err(KvsError::StringError(msg))
            }
        }
    }
}