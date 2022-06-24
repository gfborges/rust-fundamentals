use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc,
};
use std::{
    sync::Mutex,
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    Do(Job),
    Stop
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let receiver = match receiver.lock() {
                Ok(lock) => lock,
                Err(e) => {
                    println!("(W-{id}) error getting lock: {e}");
                    continue;
                },
            };

            let message = match receiver.recv() {
                Ok(msg) => msg,
                Err(e) =>  {
                    println!("(W-{id}) error receiving message: {e}");
                    continue;
                },
            };

            match message {
                Message::Do(job) => job(),
                Message::Stop => return,
            }
        });

        Self {
            id,
            handle: Some(handle),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);
        let (sender, reciever) = mpsc::channel::<Message>();
        let arc_receiver = Arc::new(Mutex::new(reciever));
        for i in 0..size {
            let arc_receiver = Arc::clone(&arc_receiver);
            threads.push(Worker::new(i, arc_receiver));
        }

        Self {
            workers: threads,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Message::Do(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Stop).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle
                    .join()
                    .expect(format!("error on joining worker {}", worker.id).as_str());
            }
        }
    }
}
