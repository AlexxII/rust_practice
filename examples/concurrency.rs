use std::sync::mpsc::Sender;
//multi-thread counter
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
// use threadpool::ThreadPool;

fn main() {
    // shared_counter();
    // channel();
    thread_pool();
}

fn shared_counter() {
    let counter = Arc::new(Mutex::new(0));
    thread::scope(|s| {
        for _ in 0..10 {
            let c = counter.clone();
            s.spawn(move || {
                let mut temp = 0;
                for _ in 0..1000 {
                    temp += 1;
                }
                let mut lock = c.lock().unwrap();
                *lock += temp;
            });
        }
    });
    println!("{}", *counter.lock().unwrap());
}

fn channel() {
    let (prod, cons) = mpsc::channel();

    let mut result = 0;
    thread::spawn(move || {
        for i in 1..=1000 {
            let _ = prod.send(i);
        }
        drop(prod);
    });

    while let Ok(msg) = cons.recv() {
        result += msg;
    }
    println!("Sum - {}", result);
}

struct Worker {
    handle: JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..workers)
            .map(|_| {
                let receiver = Arc::clone(&receiver);
                let handle = thread::spawn(move || {
                    loop {
                        let job = {
                            let lock = receiver.lock().unwrap();
                            lock.recv()
                        };

                        match job {
                            Ok(job) => job(),
                            Err(_) => break,
                        }
                    }
                });
                Worker { handle }
            })
            .collect();
        Self { workers, sender: Some(sender) }
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();

        for worker in self.workers.drain(..) {
            worker.handle.join().unwrap();
        }
    }
}

fn thread_pool() {
    let pool = ThreadPool::new(4);
    for i in 0..8 {
        pool.execute(move || {
            println!("task {i} started");
            std::thread::sleep(std::time::Duration::from_millis(500));
            println!("task {i} done");
        });
    }
}
