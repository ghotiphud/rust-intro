#![feature(core_intrinsics)]
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn main() {
    print_from_thread();

    return_from_thread();

    mutex_locking();

    mutex_locking_with_channel();
}

fn print_from_thread() {
    // spawn returns a JoinHandle which will wait until thread completion.
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    // we wait here for the thread to complete.
    handle.join().unwrap();
}

fn return_from_thread() {
    // JoinHandle<&str> here 
    let handle = thread::spawn(|| {
        "Returned from a thread!"
    });

    // wait for thread and retrieve the result
    let val = handle.join().unwrap();
    println!("{}", val);
}

fn mutex_locking() {
    // Tricky type: Arc is "Send", Mutex protects shared mutability
    let arc_data = Arc::new(Mutex::new(vec![1u32, 2, 3]));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

    for i in 0..3 {
        let mutex_data = arc_data.clone();
        let handle = thread::spawn(move || {
            let mut unlocked_data = mutex_data.lock().unwrap();
            unlocked_data[i] += 1;
        });
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();
    println!("data: {:?}", *arc_data.lock().unwrap());
}

fn mutex_locking_with_channel() {
    let arc_data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (mutex_data, thread_tx) = (arc_data.clone(), tx.clone());
        thread::spawn(move || {
            let mut unlocked_data = mutex_data.lock().unwrap();
            *unlocked_data += 1;
            thread_tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
    println!("{:?}", *arc_data.lock().unwrap());
}



fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}