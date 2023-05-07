use std::{
    error::Error,
    thread::{self, JoinHandle}, sync::{mpsc, Mutex, Arc},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    /// Building a [`ThreadPool`]
    /// with `size` of threads
    /// # Errors
    ///
    /// This function will return an error if `size` < 1.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError);
        }
        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let shared_receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&shared_receiver)));
        }
        Ok(ThreadPool { workers, sender })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        Self {
            id,
            thread: thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("{id} got a job!");

                job();
            }),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct PoolCreationError;
impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Size of threads can't be less than 1")
    }
}
impl Error for PoolCreationError {}

