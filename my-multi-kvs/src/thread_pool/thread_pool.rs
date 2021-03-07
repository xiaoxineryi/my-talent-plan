
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::borrow::Borrow;

pub struct ThreadPool{
    sender:Sender<Job>,
    workers: Vec<Worker>
}

struct Worker{
    id:u32,
    thread:JoinHandle<()>
}

impl Worker {
    pub fn new(id:u32,receiver:Arc<Mutex<Receiver<Job>>>) ->Self{
        let thread = thread::spawn(move ||{
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("the thread_pool:{} has get the job",id);
                job();
            }

        });
        Worker{
            id,
           thread
        }
    }
}

type Job = Box<dyn FnOnce() + 'static + Send>;

impl ThreadPool{
    pub fn new(number:u32) -> Self{
        assert!(number > 0);
        let mut workers = Vec::new();
        let (sender,receiver) = channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..number {
            workers.push(Worker::new(i,receiver.clone()))
        };
        Self{
            sender,
            workers
        }
    }

    pub fn execute<F>(&self,f:F)
    where F: FnOnce()+'static + Send {
        self.sender.send(Box::new(f)).unwrap();
        println!("have send the job");
    }
}