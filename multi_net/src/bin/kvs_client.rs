use structopt::StructOpt;
use std::net::SocketAddr;
use multi_net::{KvsResult,Client};



/// the socketAddr format and the default value
const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

/// parse the command line

#[derive(StructOpt,Debug)]
#[structopt(name = "kvs-client")]
struct Opt{
    #[structopt(subcommand)]
    command:Command
}

#[derive(StructOpt,Debug)]
enum Command{
    #[structopt(name = "get",about = "get the value of the key from server")]
    Get{
        #[structopt(name = "KEY")]
        key:String,
        #[structopt(
            long,
            name = "ADDR",
            raw(value_name="ADDRESS_FORMAT"),
            raw(default_value="DEFAULT_LISTENING_ADDRESS"),
            parse(try_from_str)
        )]
        addr:SocketAddr
    },
    #[structopt(name = "set",about = "save the key/value into the server ")]
    Set{
        #[structopt(name = "KEY")]
        key:String,
        #[structopt(name = "VALUE")]
        value:String,
        #[structopt(
            long,
            name = "ADDR",
            raw(value_name="ADDRESS_FORMAT"),
            raw(default_value="DEFAULT_LISTENING_ADDRESS"),
            parse(try_from_str)
        )]
        addr:SocketAddr
    },
    #[structopt(name ="remove", about = "remove the key from the server")]
    Remove{
        #[structopt(name = "KEY")]
        key:String,
        #[structopt(
            long,
            name = "ADDR",
            raw(value_name="ADDRESS_FORMAT"),
            raw(default_value="DEFAULT_LISTENING_ADDRESS"),
            parse(try_from_str)
        )]
        addr:SocketAddr
    }
}

fn main() ->KvsResult<()> {
    let opt = Opt::from_args();
    println!("{:?}",opt);
    run(opt);
    Ok(())
}

fn run(opt: Opt) -> KvsResult<()>{
    match opt.command{
        Command::Get { key,addr} => {
            let mut client = Client::connect(addr)?;
            if let Some(value) = client.get(key)?{
                println!("the value is : {}",value);
            }else {
                println!("the database don't have the key");
            }
        },
        Command::Set { key,value, addr } => {
            let mut client = Client::connect(addr)?;
            if let () = client.set(key,value)?{
                println!("the key and value have been saved");
            }
        },
        Command::Remove { key,addr} => {
            let mut client = Client::connect(addr)?;
            if let () = client.remove(key)?{
                println!("the key has been removed");
            }
        }
    };
    Ok(())
}