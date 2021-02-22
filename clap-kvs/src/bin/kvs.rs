use clap::{App, Arg, SubCommand};

fn main(){
    let matches = App::new("clap-kvs")
        .version("0.1")
        .author("kaito")
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .help("the output of the kvStore"))
        .arg(Arg::with_name("input")
            .short("i")
            .takes_value(true)
            .help("the input of the kvStore"))
        .subcommand(SubCommand::with_name("set")
            .about("set a key/value to the store")
            .arg(Arg::with_name("KEY").help("A string key").required(true))
            .arg(Arg::with_name("VALUE").help("A string value").required(true)))
        .get_matches();
    match matches.subcommand(){
        ("set",Some(_matches))=>{
            println!("{:?}",_matches);
            let key = _matches.value_of("KEY").unwrap();
            let value = _matches.value_of("VALUE").unwrap();
            println!("the key is {}, the value is {}",key,value);

        },
        _ =>{
            eprintln!("the app don't have this command")
        }
    };
}