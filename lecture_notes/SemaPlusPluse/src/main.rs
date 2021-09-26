extern crate rand;
use std::sync::{Arc, Condvar, Mutex};

use rand::Rng;
use std::collections::VecDeque;
use std::{thread, time};

fn rand_sleep() {
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0..30)));
}

#[derive(Clone)]
pub struct SemaPlusPlus<T> {
    queue_and_cv: Arc<(Mutex<VecDeque<T>>, Condvar)>,
}

impl<T> SemaPlusPlus<T> {
    pub fn new() -> Self {
        SemaPlusPlus {
            queue_and_cv: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    // Enqueues -- Like semaphore.signal()
    pub fn send(&self, message: T) {
        let mut queue = self.queue_and_cv.0.lock().unwrap();
        queue.push_back(message);
        self.queue_and_cv.1.notify_all();
    }

    // Dequeues -- Like semaphore.wait()
    pub fn recv(&self) -> T {
        let mut queue = self
            .queue_and_cv
            .1
            .wait_while(self.queue_and_cv.0.lock().unwrap(), |x| x.is_empty())
            .unwrap();
        queue.pop_front().unwrap()
    }
}

const NUM_THREADS: usize = 12;
fn main() {
    // Inspired by this example https://doc.rust-lang.org/stable/rust-by-example/std_misc/channels.html
    let sem: SemaPlusPlus<String> = SemaPlusPlus::new();
    let mut handles = Vec::new();
    for i in 0..NUM_THREADS {
        let sem_clone = sem.clone();
        let handle = thread::spawn(move || {
            rand_sleep();
            sem_clone.send(format!("thread {} just finished!", i))
        });
        handles.push(handle);
    }
    for _ in 0..NUM_THREADS {
        println!("{}", sem.recv())
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
