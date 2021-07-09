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

pub struct ThreadPool
{
    threads: Vec<Worker>,
    sender: Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
type SafeJobReceiver = Arc<Mutex<Receiver<Job>>>;

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
                let mut threads = Vec::with_capacity(size);

                let (sender, receiver) = mpsc::channel();
                let receiver: SafeJobReceiver = Arc::new(Mutex::new(receiver));

                for id in 0..size
                {
                    threads.push(Worker::new(id, Arc::clone(&receiver)));
                }

                Ok(ThreadPool
                { 
                    threads: threads,
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

        self.sender.send(job).unwrap();
    }
}

struct Worker
{
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker
{
    fn new(id: usize, receiver: SafeJobReceiver) -> Worker
    {
        Worker
        {
            id: id,
            handle: thread::spawn(move ||
            {
                loop
                {
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {} got a job; executing...", id);

                    job();
                }
            }),
        }
    }
}