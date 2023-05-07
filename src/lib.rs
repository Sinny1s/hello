use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
};

/// # ThreadPool
/// structure managing threads
/// 
/// use `ThreadPool::build` to create one
/// 
/// and `ThreadPool::execute` to give job
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }
    /// Executing your function by worker in another thread!
    ///
    /// # Panics
    ///
    /// I guess its not panic cause i don't let you to my field
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap_or_else(|e| eprintln!("receiver disappeared! {}", e));
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = Some(thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    println!("Worker {}: executing!!!", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {}: sender disappeared!!! Shutting down...", id);
                    break;
                }
            }
        }));
        Self {
            id,
            thread,
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

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            drop(self.sender.take());
            println!("ThreadPool: shutting down worker {}~", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}
