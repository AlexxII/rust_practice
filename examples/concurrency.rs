//multi-thread counter

use std::sync::{Arc, Mutex, mpsc};
use std::{result, thread};

fn main() {
    // shared_counter();
    channel();
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
