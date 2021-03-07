use structopt::StructOpt;
use std::net::SocketAddr;
use my_multi_kvs::{KvsResult, Client};



const DEFAULT_VALUE : &str = "127.0.0.1:8080";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(StructOpt,Debug)]
#[structopt(name = "opt")]
struct Opt{
    #[structopt(subcommand)]
    command:Command
}

#[derive(StructOpt,Debug)]
enum Command{
    #[structopt(name = "set",about = "set the key/value")]
    Set{
        #[structopt(name = "key")]
        key:String,
        #[structopt(name = "value")]
        value:String,
        #[structopt(
            name = "addr",
            long,
            raw(value_name = "ADDRESS_FORMAT"),
            raw(default_value = "DEFAULT_VALUE"),
            parse(try_from_str)
        )]
        addr:SocketAddr
    },
    #[structopt(name = "get")]
    Get{
        #[structopt(name = "key")]
        key :String,
        #[structopt(
        name = "addr",
        long,
        raw(value_name = "ADDRESS_FORMAT"),
        raw(default_value = "DEFAULT_VALUE"),
        parse(try_from_str)
        )]
        addr:SocketAddr
    },
    #[structopt(name = "remove")]
    Remove{
        #[structopt(name = "key")]
        key:String,
        #[structopt(
        name = "addr",
        long,
        raw(value_name = "ADDRESS_FORMAT"),
        raw(default_value = "DEFAULT_VALUE"),
        parse(try_from_str)
        )]
        addr:SocketAddr
    }
}

fn main() -> KvsResult<()>{
    let opt = Opt::from_args();
    match opt.command{
        Command::Set {key,value,addr} =>{
            let mut client = Client::connect(addr)?;
            client.set(key,value)?;
        }
        Command::Get {key,addr} =>{
            let mut client = Client::connect(addr)?;
            client.get(key)?;
        }
        Command::Remove {key,addr} =>{
            let mut client = Client::connect(addr)?;
            client.remove(key)?;
        }
        _ =>{
            panic!("the client don't have this command")
        }
    }
    Ok(())
}