use serde::{Deserialize, Serialize};
/// Struct representing a command.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command{
    Set {key:String,value:String},
    Remove{ key:String },
}

impl Command{
    pub fn set(key:String,value:String) -> Self{
        Command::Set { key, value }
    }

    pub fn remove(key:String) -> Self{
        Command::Remove {key}
    }
}