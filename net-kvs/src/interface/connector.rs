use crate::KvsResult;
use std::net::SocketAddr;

pub trait Communicate{
    fn set(&self,key:String,value:String) -> KvsResult<()>;

    fn get(&self,key:String) -> KvsResult<Option<String>>;

    fn remove(&self,key:String) -> KvsResult<()>;
}

pub trait Connect {
    fn connect(addr: SocketAddr) -> KvsResult<Box<dyn Communicate>>;
}
