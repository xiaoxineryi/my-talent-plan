use std::net::SocketAddr;
use multi_net::{KvsResult, KvEngine};
use std::env::current_dir;
use std::fs;
use std::process::exit;
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Engine {
    kvs,
    sled
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Opt {
    #[structopt(
    long,
    help = "Sets the listening address",
    value_name = "IP:PORT",
    raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
    parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(
    long,
    help = "Sets the storage engine",
    value_name = "ENGINE-NAME",
    raw(possible_values = "&Engine::variants()")
    )]
    engine: Option<Engine>,
}






fn main() {
    let opt = Opt::from_args();
    let res = current_engine().and_then(move |curr_engine| {
        if opt.engine.is_none(){
            opt.engine = curr_engine;
        }
        if curr_engine.is_some() && opt.engine != curr_engine {
            println!("Wrong engine!");
            exit(1);
        }
        run(opt)
    });
    if let Err(e) = res {
        println!("{}", e);
        exit(1);
    }
}

fn run(opt:Opt) -> KvsResult<()>{
    let engine = opt.engine.unwrap_or(DEFAULT_ENGINE);
    println!("kvs-server {}", env!("CARGO_PKG_VERSION"));
    println!("Storage engine: {}", engine);
    println!("Listening on {}", opt.addr);

    // write engine to engine file
    fs::write(current_dir()?.join("engine"), format!("{}", engine))?;
    run_with_engine(kvs::open(current_dir()?)?, opt.addr)
}

fn run_with_engine<E: KvEngine>(engine: E, addr: SocketAddr) -> KvsResult<()> {
    let server = KvsServer::new(engine);
    server.run(addr)
}

fn current_engine() -> KvsResult<Option<Engine>>{
    let engine = current_dir()?.join("engine");
    if !engine.exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            println!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }
}
