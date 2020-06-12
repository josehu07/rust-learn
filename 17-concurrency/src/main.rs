use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
// More on `atomic`, `Barrier`, `MutexGuard`, `RwLock`...


fn main() -> thread::Result<()> {
    
    // Simple multithreading.
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from thread {:?}", thread::current().id());
    });
    println!("Hello from thread {:?}", thread::current().name());
    handle.join()?;

    // To pass values between threads, often we need `move`, because if main thread exits early,
    //   a reference from the child thread will become invalid.
    let v = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("There is a vector with length {}.", v.len());
    });
    handle.join()?;

    // Message passing by channels (here demonstrated also using threads).
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = 7;
        tx.send(val).unwrap();  // `send()` from `Sender` will never block (asynchronous send),
                                //   but `send()` from `SyncSender` (create by `sync_channel()`)
                                //   is synchronous.
        // `val` is moved here.
    });
    println!("Got: {}!", rx.recv().unwrap());   // `recv()` blocks, while `try_recv()` does not.

    // Cloning multiple senders.
    let (tx0, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx0);
    thread::spawn(move || { tx0.send(33).unwrap(); });
    thread::spawn(move || { tx1.send(77).unwrap(); });
    for received in rx {
        println!("Got: {}!!", received);
    }

    // Using mutex protection by sharing it across threads safely with `Arc` (Atomic Rc).
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            if let Ok(mut num) = counter.lock() {
                *num += 1;
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *counter.lock().unwrap());

    Ok(())
}


// NOTICE: Must have enough knowledge on CA / OS / Concurrency before using these features.
