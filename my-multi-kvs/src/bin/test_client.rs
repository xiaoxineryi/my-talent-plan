use std::net::{TcpListener, TcpStream};
use structopt::StructOpt;
use std::io::{Write, Read, repeat, BufWriter, BufReader};
use std::borrow::Borrow;


#[derive(StructOpt,Debug)]
#[structopt(name = "opt")]
struct Opt{
    #[structopt(subcommand)]
    command:Command
}


#[derive(StructOpt,Debug)]
enum Command{
    #[structopt(name = "hello")]
    Hello,
    #[structopt(name = "bye")]
    Bye,
}
fn main() {
    let opt = Opt::from_args();
    let mut tcpStream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut content;
    match opt.command{
        Command::Hello =>{
            content = "hello".as_bytes();
        }
        Command::Bye => {
            content = "bye".as_bytes();
        }
        _ => {
            content = "error".as_bytes();
        }
    }
    let mut writer = BufWriter::new(&tcpStream);
    let mut reader = BufReader::new(&tcpStream);
    writer.write(content).unwrap();
    writer.flush().unwrap();
    println!("send a message");
    let mut resp = [0 as u8;1024];

    reader.read(&mut resp).unwrap();

    let s = String::from_utf8(Vec::from(resp)).unwrap();
    println!("{}",s);
}