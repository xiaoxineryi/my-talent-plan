use crate::KvsResult;

pub trait Engine:Clone + 'static + Send{
    fn get(&self,key:String) -> KvsResult<Option<String>>;

    fn set(&mut self,key:String,value:String) -> KvsResult<()>;

    fn remove(&self,key:String) ->KvsResult<()>;
}