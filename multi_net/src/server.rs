use crate::engines::KvEngine;
use std::net::{ToSocketAddrs, TcpStream, TcpListener};
use crate::KvsResult;
use std::io::{BufReader, BufWriter, Write};
use serde_json::Deserializer;
use crate::common::{Request, GetResponse, SetResponse, RemoveResponse};

struct Server<E: KvEngine>{
    engine:E
}

impl<E: KvEngine> Server<E>{
    pub fn new(engine:E) -> Self{
        Server{
            engine
        }
    }
    fn run<A:ToSocketAddrs>(&mut self, addr:A) -> KvsResult<()>{
        let listener = TcpListener::bind(addr)?;
        for tcpStream in listener.incoming(){
            match tcpStream{
                Ok(stream) =>{
                    if let Err(e) =self.serve(stream){
                        println!("Error on serving client : {:?}",e);
                    }
                },
                Err(e) =>{
                    println!("Connection Error : {:?}",e);
                }
            }
        };
        Ok(())
    }

    fn serve(&mut self,tcp:TcpStream) -> KvsResult<()>{
        let peer_addr = tcp.peer_addr()?;
        let reader =BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);

        let req_reader =Deserializer::from_reader(reader).into_iter::<Request>();

        macro_rules! send_resp {
            ($resp:expr) => {{
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?;
                println!("Response sent to {}: {:?}", peer_addr, resp);
            };};
        }
        
        for req in req_reader{
            let req = req?;
            match req{
                Request::Get { key } => send_resp!(match self.engine.get(key) {
                    Ok(value) => GetResponse::Ok(value),
                    Err(e) => GetResponse::Err(format!("{}", e)),
                }),
                Request::Set { key,value} => send_resp!(match self.engine.set(key,value){
                    Ok(()) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(format!("{}",e)),
                }),
                Request::Remove { key} => send_resp!(match self.engine.remove(key) {
                    Ok(()) => RemoveResponse::Ok(()),
                    Err(e) => RemoveResponse::Err(format!("{}",e)),
                })
            }
        };
        Ok(())
    }
}