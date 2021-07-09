use std::thread;
use std::thread::JoinHandle;
use std::string::ToString;
use std::sync::mpsc;
use std::sync::mpsc::{ Receiver, Sender };
use std::sync::{ Arc, Mutex };

pub enum ThreadPoolCreationError
{
    NullSize,
}

impl ToString for ThreadPoolCreationError
{
    fn to_string(&self) -> String
    {
        String::from("Tried to instantiate a thread pool with no worker (size = 0).")
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type SafeJobReceiver = Arc<Mutex<Receiver<Message>>>;

enum Message
{
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: Sender<Message>,
}


impl ThreadPool 
{
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
	pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolCreationError>
	{
        match size
        {
            0 => Err(ThreadPoolCreationError::NullSize),
            _ => {
                let mut workers = Vec::with_capacity(size);

                let (sender, receiver) = mpsc::channel();
                let receiver: SafeJobReceiver = Arc::new(Mutex::new(receiver));

                for id in 0..size
                {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                Ok(ThreadPool
                { 
                    workers: workers,
                    sender: sender,
                })
            },
        }
	}

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool
{
    fn drop(&mut self)
    {
        for _ in &mut self.workers
        {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers
        {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread_handle) = worker.handle.take()
            {
                thread_handle.join().unwrap();
            }
        }
    }
}

struct Worker
{
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker
{
    fn new(id: usize, receiver: SafeJobReceiver) -> Worker
    {
        Worker
        {
            id: id,
            handle: Some(thread::spawn(move ||
            {
                loop
                {
                    let message = receiver.lock().unwrap().recv().unwrap();

                    match message
                    {
                        Message::NewJob(job) =>
                        {
                            println!("Worker {} got a job; executing...", id);
                            job();
                        },
                        Message::Terminate =>
                        {
                            println!("Worker {} got a termination command; stopping...", id);
                            break;
                        },
                    }

                }
            })),
        }
    }
}