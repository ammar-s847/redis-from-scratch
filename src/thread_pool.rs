use std::{
    thread,
    deque::VecDeque,
    sync::{Arc, Mutex, Condvar},
};

pub mod thread_pool;

#[derive(Debug)]
struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    queue: Vec<()>,
    mutex: Mutex<VecDeque<()>>,
    not_empty: Condvar,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            threads.push(thread::spawn(|| {
                println!("Hello from thread");
            }));
        }

        ThreadPool {
            threads,
            queue: Vec::new(),
        }
    }
}
