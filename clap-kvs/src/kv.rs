use std::collections::HashMap;

/// Example:
///
/// ```rust
/// # use clap_kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(),"value".to_owned());
/// let value = store.get("key".to_owned());
/// assert_eq!(Some("value".to_owned()),value)
/// ```
///

#[derive(Default)]
pub struct KvStore{
    map:HashMap<String,String>
}

impl KvStore{
    pub fn new() -> Self{
        KvStore{
            map: HashMap::new()
        }
    }
    pub fn set(&mut self,key:String,val:String){
        self.map.insert(key,val);
    }
    pub fn get(&self,key:String) -> Option<String>{
        self.map.get(&key).cloned()
    }
    pub fn remove(&mut self,key:String){
        self.map.remove(&key);
    }
}