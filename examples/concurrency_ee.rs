use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    fn new(workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let workers = (0..workers)
            .map(|_| {
                let receiver = Arc::clone(&receiver);
                thread::spawn(move || {
                    loop {
                        let job = receiver.lock().unwrap().recv();
                        match job {
                            Ok(job) => job(),
                            Err(_) => break,
                        }
                    }
                })
            })
            .collect();
        Self {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(job)).unwrap();
    }

    fn shutdown() {}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();

        for worker in self.workers.drain(..) {
            worker.join().unwrap()
        }
    }
}

fn main() {
    let pool = ThreadPool::new(4);

    for i in 0..10 {
        println!("sending {i}");
        pool.execute(move || {
            std::thread::sleep(std::time::Duration::from_millis(400));
        });
        println!("sent {i}");
    }
}
