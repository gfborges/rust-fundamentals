use std::{thread::{JoinHandle, self}, sync::Mutex, };
use std::sync::{mpsc::{self, Receiver, Sender}, Arc};

type Job = Box<dyn FnOnce() + Send + 'static>;


pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Sender<Job>
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            }
        });

        Self { id, handle }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);
        let (sender, reciever) = mpsc::channel();
        let arc_receiver = Arc::new(Mutex::new(reciever));
        for i in  0..size {
            let arc_receiver = Arc::clone(&arc_receiver);
            threads.push(Worker::new(i, arc_receiver));
        }

        Self { threads, sender }

    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
