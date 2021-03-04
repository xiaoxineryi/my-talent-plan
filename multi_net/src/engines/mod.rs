mod kvs;

use crate::KvsResult;

/// Trait for a key value storage engine.
pub trait KvEngine {
    fn get(&mut self,key:String) -> KvsResult<Option<String>>;
    fn set(&mut self,key:String,value:String) -> KvsResult<()>;
    fn remove(&mut self,key:String) -> KvsResult<()>;
}


