use net_kvs::{KvsResult, MockConnector, Communicate, Connect, KvsError};
use structopt::StructOpt;
use std::net::SocketAddr;
use std::io;
use std::io::{Read};
use std::str::Split;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(StructOpt,Debug)]
struct Opt{
    #[structopt(subcommand)]
    command:Command,
}

#[derive(StructOpt, Debug)]
enum Command{
    #[structopt(name= "connect",about = "connect with the server")]
    Connect{
        #[structopt(
        long,
        help = "Sets the server address",
        raw(value_name = "ADDRESS_FORMAT"),
        raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
        parse(try_from_str)
        )]
    addr:SocketAddr
    }
}

fn main() ->KvsResult<()>{
    let opt = Opt::from_args();
    println!("{:?}",opt);
    match opt.command{
        Command::Connect {addr} =>{
            let mut input = String::new(); 
            let mut reader = io::stdin();
            let connector = MockConnector::connect(addr).unwrap();

            while (reader.read_line(&mut input).unwrap()) != 0 {
                let mut input_iter = input.split(" ");
                let command = input_iter.next().unwrap_or("can't find a command");

                match command {
                    "get" =>{
                        let key = input_iter.next().ok_or_else(|| KvsError::ParseLackArgs)?;
                        let value = connector.get(key.to_owned())?;
                    },
                    "set" =>{
                        let key = input_iter.next().ok_or_else(|| KvsError::ParseLackArgs)?;
                        let value = input_iter.next().ok_or_else(|| KvsError::ParseLackArgs)?;
                        connector.set(key.to_owned(),value.to_owned());
                    },
                    "remove" =>{
                        let key = input_iter.next().ok_or_else(|| KvsError::ParseLackArgs)?;
                        connector.remove(key.to_owned());
                    },
                    "exit" =>{
                        return Ok(());
                    },
                    _  =>{
                        println!("can't parse this command");
                    }
                }

                input.clear();
            }
        },
        _ =>{
            println!("please connect to your server first.");
        }
    };
    Ok(())
}