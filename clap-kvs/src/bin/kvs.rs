use clap::{App, Arg, SubCommand};

/// this method parse command line with clap
#[warn(dead_code)]
fn parse_with_clap(){
    let matches = App::new("clap-kvs")
        .version("0.1")
        .author("kaito")
        // this is mandatory
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .required(true)
            .help("the output of the kvStore"))
        // this is an optional command
        .arg(Arg::with_name("input")
            .short("i")
            .takes_value(true)
            .help("the input of the kvStore"))
        // this is a subcommand which can have its args
        .subcommand(SubCommand::with_name("set")
            .about("set a key/value to the store")
            .arg(Arg::with_name("KEY").help("A string key").required(true))
            .arg(Arg::with_name("VALUE").help("A string value").required(true)))
        .get_matches();
    match matches.subcommand(){
        ("set",Some(_matches))=>{
            let key = _matches.value_of("KEY").unwrap();
            let value = _matches.value_of("VALUE").unwrap();
            println!("the key is {}, the value is {}",key,value);

        },
        _ =>{
            eprintln!("the app don't have this command")
        }
    };
}


// parse the command line with structopt

use std::path::PathBuf;
use structopt::StructOpt;

#[warn(dead_code)]
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd:Command
}

#[warn(dead_code)]
#[derive(StructOpt,Debug)]
#[structopt(name = "command",about = "these are the commands of this application")]
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

#[warn(dead_code)]
fn parse_with_structopt(){
    let opt = Opt::from_args();
    println!("{:?}",opt);
    match opt.cmd {
        Command::Set{key,value} =>{
            println!("the key is {} and the value is {}",key,value);
        },
        _ =>{
            println!("nothing happen");
        }
    }
}


fn main(){
    parse_with_structopt();
}