use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn main() {
    print_from_thread();

    return_from_thread();

    mutex_locking();

    mutex_locking_channels();
}

fn print_from_thread() {
    //Send and Sync
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}

fn return_from_thread() {
    //Send and Sync
    let handle = thread::spawn(|| {
        "Returned from a thread!"
    });

    let val = handle.join().unwrap();

    println!("{}", val);
}

fn mutex_locking() {
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

fn mutex_locking_channels() {
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