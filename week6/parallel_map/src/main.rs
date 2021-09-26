extern crate num_cpus;

use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec = Vec::with_capacity(input_vec.len());
    output_vec.resize_with(input_vec.len(), Default::default);
    
    let (send_to_func, func_receiver) = crossbeam::channel::unbounded();
    let (send_to_output, output_receiver) = crossbeam::channel::unbounded();

    let mut threads = Vec::new();
    for _ in 0..num_cpus::get() {
        let receiver = func_receiver.clone();
        let sender = send_to_output.clone();
        threads.push(thread::spawn(move || {
            while let Ok((idx, next_num)) = receiver.recv() {
                sender.send((idx, f(next_num))).expect("Tried writing to channel, but there are no receivers!");
            }
        }));
    }

    let len = input_vec.len();
    for i in 0..len {
        send_to_func
            .send((len - i - 1, input_vec.pop().unwrap()))
            .expect("Tried writing to channel, but there are no receivers!");
    }
    
    drop(send_to_func);
    drop(send_to_output);
    
    while let Ok((idx, out)) = output_receiver.recv() {
        output_vec[idx] = out;
    }

    for thread in threads {
        thread.join().expect("Panic occurred in thread");
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
