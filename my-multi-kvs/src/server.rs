use crate::engine::Engine;
use crate::{KvsResult, ThreadPool};
use std::net::{TcpStream, ToSocketAddrs, TcpListener};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use crate::common::{Request, GetResponse, SetResponse, RemoveResponse};
use serde::Deserialize;
use std::time::Duration;

pub struct Server<E:Engine>{
    engine : E,
    thread_pool : ThreadPool
}

impl<E:Engine> Server<E>{
    pub fn new(engine : E,number:u32) -> Self{
        let thread_pool = ThreadPool::new(number);
        Self{
            engine,
            thread_pool
        }
    }

    pub fn bind<A : ToSocketAddrs>(&mut self,addr:A) -> KvsResult<()>{
        let tcpListener = TcpListener::bind(addr)?;
        println!("i have connected");
        for tcpStream in tcpListener.incoming(){
            let tcpStream = tcpStream?;
            let mut engine = self.engine.clone();
            self.thread_pool.execute(move ||{
                serve(&mut engine,tcpStream);
            })
        };
        Ok(())
    }
}

fn serve<E:Engine>(engine:&mut E,tcp_stream:TcpStream) ->KvsResult<()>{
    let mut reader = Deserializer::from_reader(BufReader::new(tcp_stream.try_clone()?));
    let mut writer = BufWriter::new(tcp_stream);

    macro_rules! send_resp {
        ($resp:expr) => {{
            let resp = $resp;
            writer.write(serde_json::to_string(&resp).unwrap().as_bytes())?;
            writer.flush()?;
        };};
    }
    let request = Request::deserialize(&mut reader)?;
    match request{
        Request::Get {key} =>{
            send_resp!(match engine.get(key){
                Ok(value) => GetResponse::Ok(value),
                Err(msg) => GetResponse::Err(format!("{}",msg))
            })
        }
        Request::Set {key,value} =>{
            send_resp!(match engine.set(key,value) {
                Ok(()) => SetResponse::Ok(()),
                Err(msg) => SetResponse::Err(format!("{}",msg))
             })
        }
        Request::Remove {key} =>{
            std::thread::sleep(Duration::from_secs(5));
            send_resp!(match engine.remove(key){
                Ok(()) => RemoveResponse::Ok(()),
                Err(msg) => RemoveResponse::Err(format!("{}",msg))
            })
        }
    };
    Ok(())
}