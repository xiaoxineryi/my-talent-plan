use std::path::PathBuf;
use structopt::StructOpt;
use log_kvs::{KvStore, KvResult};

#[derive(Debug,StructOpt)]
#[structopt(name = "cmd",about = "the command of kvs",version = "0.1")]
struct Opt{
    #[structopt(short = "i",parse(from_os_str))]
    input : PathBuf,
    #[structopt(subcommand)]
    command:Command
}

#[derive(StructOpt,Debug)]
#[structopt(name = "command")]
enum Command{
    Set{
        key:String,
        value:String
    },
    Get{
        key:String
    },
    Remove{
        key:String
    }
}

fn main() -> KvResult<()>{
    let opt =Opt::from_args();
    println!("{:?}",opt);
    let path = opt.input;
    let mut kvStore = KvStore::open(path)?;
    match opt.command {
        Command::Set {key,value} =>{
            println!("save the key:{},value:{}",key,value);
            kvStore.set(key,value);
        },
        Command::Remove {key} =>{
            println!("remove the key :{}",key);
            kvStore.remove(key);
        },
        Command::Get {key} =>{
            let value = kvStore.get(key)?;
            println!("the value is {}",value.unwrap_or(" ".to_owned()));
        }
    }
    Ok(())
}