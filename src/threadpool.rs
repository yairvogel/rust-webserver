use std::{thread, sync::{mpsc::{channel, Receiver, Sender}, Mutex, Arc}};

type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    Job(Job),
    Terminate
}

type AsyncMutex<T> = Arc<Mutex<T>>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Message>>
}


#[derive(Debug)]
pub struct PoolCreationError;

impl ThreadPool {
    pub fn new(threads: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(threads > 0);

        let (sender, reciever) = channel::<Message>();
        let reciever: AsyncMutex<Receiver<Message>> = Arc::new(Mutex::new(reciever));
        let workers: Vec<Worker> = (0..threads)
            .map(|_| Worker::new(Arc::clone(&reciever)))
            .collect();

        Ok(ThreadPool { workers, sender: Some(sender) })
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static 
    {
        if let Some(sender) = &self.sender {
            sender.send(Message::Job(Box::new(f))).unwrap()
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..self.workers.len() {
            if let Some(sender) = &self.sender {
                sender.send(Message::Terminate).unwrap()
            }
        }
        for worker in &mut self.workers {
            if let Some(handle) = worker.0.take() {
                handle.join().unwrap()
            }
        }
    }
}

struct Worker(Option<thread::JoinHandle<()>>);

impl Worker {
    fn new(reciever: AsyncMutex<Receiver<Message>>) -> Worker {
        let thread = thread::spawn(move || loop {
                let reciever = reciever.lock().unwrap();
                match reciever.recv().unwrap() {
                    Message::Job(job) => job(),
                    Message::Terminate => break,
                }
            });
        Worker(Some(thread))
    }
}