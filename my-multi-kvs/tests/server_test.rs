use my_multi_kvs::{KvsEngine, Server};
use std::net::SocketAddr;
use std::str::FromStr;

#[test]
fn test_server(){
    let engine = KvsEngine::open("./tests/data").unwrap();
    let mut server = Server::new(engine, 5);
    let addr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    server.bind(addr);
}