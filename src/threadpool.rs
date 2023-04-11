use std::{thread, sync::{mpsc::{channel, Receiver, Sender}, Mutex, Arc}};

type Job = Box<dyn FnOnce() + Send + 'static>;
type AsyncMutex<T> = Arc<Mutex<T>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}


#[derive(Debug)]
pub struct PoolCreationError;

impl ThreadPool {
    pub fn new(threads: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(threads > 0);

        let (sender, reciever): (Sender<Job>, Receiver<Job>) = channel::<Job>();
        let reciever: AsyncMutex<Receiver<Job>> = Arc::new(Mutex::new(reciever));
        let workers: Vec<Worker> = (0..threads)
            .map(|_| Worker::new(Arc::clone(&reciever)))
            .collect();

        Ok(ThreadPool { workers, sender: Some(sender) })
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static 
    {
        if let Some(sender) = &self.sender {
            sender.send(Box::new(f)).unwrap()
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(handle) = worker.0.take() {
                handle.join().unwrap()
            }
        }
    }
}

struct Worker(Option<thread::JoinHandle<()>>);

impl Worker {
    fn new(reciever: AsyncMutex<Receiver<Job>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let reciever = reciever.lock().unwrap();
                let job = reciever.recv().unwrap();
                drop(reciever);
                job();
            }});
        Worker(Some(thread))
    }
}