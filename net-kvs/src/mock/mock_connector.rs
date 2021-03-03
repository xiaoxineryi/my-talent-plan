use crate::interface::{Connect, Communicate};
use crate::KvsResult;
use std::net::SocketAddr;

pub struct MockConnector {
    addr:SocketAddr
}

impl MockConnector {
    fn new(addr:SocketAddr) -> MockConnector{
        MockConnector{ addr }
    }
}

impl Communicate for MockConnector{
    fn set(&self, key: String, value: String) -> KvsResult<()> {
        println!("insert,key={},value={}",key,value);
        Ok(())
    }
    fn get(&self, key: String) -> KvsResult<Option<String>> {
        println!("get None");
        Ok(None)
    }
    fn remove(&self, key: String) -> KvsResult<()> {
        println!("removing");
        Ok(())
    }
}
impl Connect for MockConnector{
    fn connect(addr: SocketAddr) -> KvsResult<Box<dyn Communicate>> {
        println!("the client is try connect with the address:{}",addr);
        Ok(Box::new(MockConnector::new(addr) ))
    }
}