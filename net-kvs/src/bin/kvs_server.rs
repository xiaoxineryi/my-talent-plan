use std::net::{TcpListener, TcpStream};
use failure::_core::mem::transmute_copy;
use std::io::{BufWriter, BufReader};
use net_kvs::KvsResult;
use serde_json::{Serializer,Deserializer};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum GetResponse {
    Ok(Option<String>),
    Err(String),
}


fn main() {
    let tcpListener = TcpListener::bind("127.0.0.1:4040")?;
    for stream in tcpListener.incoming(){
        match stream {
            Ok(tcpStream) =>{
                deal(tcpStream);
            },
            _ =>{

            }
        }
    }
}

fn deal(tcp_stream:TcpStream) -> KvsResult<()>{

    let peer_addr = tcp_stream.peer_addr()?;

    let mut writer = BufWriter::new(&tcp_stream);
    let reader = BufReader::new(&tcp_stream);

    let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

    macro_rules! send_resp {
        ($resp:expr) => {{
            let resp = $resp;
            serde_json::to_writer(&mut writer, &resp)?;
            writer.flush()?;
            debug!("Response sent to {}: {:?}", peer_addr, resp);
        };};
    }

    for req in req_reader{
        let req = req?;
        debug!("Receive request from {}: {:?}", peer_addr, req);

        match req{
            Request::Get { key } => send_resp!(GetResponse::Ok(Some("get the key 1"))),
            Request::Set { key,value } => send_resp!(GetResponse::Ok(Some("set the key"))),
            Request::Remove { key } => send_resp!(GetResponse::Ok(Some("remove the key")))
        }
    }

    Ok(())
}