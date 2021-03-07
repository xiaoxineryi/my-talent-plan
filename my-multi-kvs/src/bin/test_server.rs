use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufWriter, Read};
use my_multi_kvs::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let thread_pool = ThreadPool::new(4);
    for tcpStream in listener.incoming(){
        thread_pool.execute(||{handle_stream(tcpStream.unwrap())});
    }
}

fn handle_stream(tcpStream : TcpStream){
    let mut reader = BufReader::new(&tcpStream);
    let mut writer = BufWriter::new(&tcpStream);

    let mut buffer =[0 as u8;1024];

    reader.read(&mut buffer).unwrap();

    let mut content;
    println!("receive a message");
    if buffer.starts_with(b"hello"){
        content = "good".as_bytes();
    }else{
        thread::sleep(Duration::from_secs(5));
        content = "not good".as_bytes();
    }
    writer.write(content).unwrap();
    writer.flush().unwrap();
    println!("send a message");
}