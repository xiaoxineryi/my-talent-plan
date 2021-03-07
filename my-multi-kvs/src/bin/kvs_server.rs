use structopt::StructOpt;
use std::net::{SocketAddr, TcpListener, TcpStream};
use my_multi_kvs::{KvsResult, ThreadPool, Server, KvsEngine};
use std::sync::Arc;
use std::path::Path;

const ADDRESS_FORMAT: &str = "IP:PORT";
const DEFAULT_ADDRESS: &str = "127.0.0.1:8080";


#[derive(StructOpt, Debug)]
#[structopt(name = "opt")]
struct Opt {
    #[structopt(subcommand)]
    command: Command
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "bind", about = "listen to a IP:PORT to get the request")]
    Bind {
        #[structopt(
        long,
        name = "addr",
        raw(value_name = "ADDRESS_FORMAT"),
        raw(default_value = "DEFAULT_ADDRESS"),
        parse(try_from_str)
        )]
        addr: SocketAddr,
        #[structopt(
        long,
        name = "num",
        parse(try_from_str)
        )]
        number: u32,
        #[structopt(
        name = "location",
        long
        )]
        location:String
    }
}

fn main() -> KvsResult<()> {
    let opt = Opt::from_args();
    match opt.command {
        Command::Bind { addr, number,location } => {
            let engine = KvsEngine::open(location)?;
            let mut server = Server::new(engine, number);
            server.bind(addr);
        }
        _ => {
            panic!("the server don't have this command");
        }
    }
    Ok(())
}

