use std::env::current_dir;
use log_kvs::{KvStore, KvResult};

#[test]
fn m() -> KvResult<()>{
    let path = ".\\tests";
    let mut kvStore = KvStore::open(path)?;

    kvStore.set("1".to_owned(),"2".to_owned());
    kvStore.set("3".to_owned(),"4".to_owned());
    let value = kvStore.get("1".to_owned())?;
    assert_eq!(value.unwrap(),"2".to_owned());
    Ok(())
}