use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};


// Thread pool.
pub struct ThreadPool {
    workers : Vec<Worker>,
    sender : mpsc::Sender<Message>,
}

impl ThreadPool {

    // Initialize a thread pool with given size number of threads.
    pub fn new(size : usize) -> ThreadPool {

        // Size must be positive.
        assert!(size > 0);

        // Create a bounded-sized vector of workers.
        let mut workers = Vec::with_capacity(size);

        // Create the channel for passing requests to workers. The receiver is shared among
        // threads, therefore should be wrapped in a mutex lock and shared using atomic
        // reference counting.
        let (sender, receiver) = mpsc::channel::<Message>();
        let receiver = Arc::new(Mutex::new(receiver));

        // Initialize the workers.
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // Trigger a vacant worker to execute a closure.
    pub fn exec<F>(&self, func : F)
        where F : FnOnce() + Send + 'static
    {
        let job = Box::new(func);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {

    // Implement drop trait to enable graceful shutdown.
    fn drop(&mut self) {

        // Send terminate messages, one for each worker.
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // For each worker, take its thread handle and join it.
        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
                println!("ß Worker {} terminated.", worker.id);
            }
        }
    }
}


// Worker wrapping over a thread.
struct Worker {
    id : usize,
    handle : Option<thread::JoinHandle<()>>,    // None variant indicates going to shutdown.
}

impl Worker {

    // Initialize a worker by spawing a thread that loops infinitely to receive jobs.
    pub fn new(id : usize, receiver : Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        // Spawn the thread, get its handle.
        let handle = thread::spawn(move || {
            loop {

                // At any time, there should be one thread holding the mutex lock, waiting on
                // the receiver (hoping to get a new message), and other threads waiting on
                // the lock.
                let msg = receiver.lock().unwrap().recv().unwrap();

                // Match the type of the message reveived.
                match msg {
                    Message::NewJob(job) => {
                        println!("ß Worker {} got a new job.", id);
                        job.call_box();
                    },
                    Message::Terminate => break,
                }
            }
        });
        
        Worker { id, handle : Some(handle) }
    }
}


// Alias `Job` as the type of a box over a closure.
type Job = Box<FnBox + Send + 'static>;


// To enable calling the closure within threads, implement `FnBox` trait for it, which
// explicitly specifies a `call_box()` function to take over the ownership of the closure
// inside the box. Otherwise the compiler will complain.
trait FnBox {
    fn call_box(self : Box<Self>);
}

impl<F : FnOnce()> FnBox for F {
    fn call_box(self : Box<F>) {
        (*self)();
    }
}


// A message sent is either a new job or a terminate signal.
enum Message {
    NewJob(Job),
    Terminate,
}
