use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

fn main() {
    actor();
    println!("");

    actor_channel();
    println!("");
    println!("");

    read_only();
    println!("");
    println!("");    

    locking();
    println!("");

    print_from_thread();
    println!("");

    return_from_thread();
    println!("");

    mutex_locking();
    println!("");
}

fn actor() {
    let mut handles: Vec<thread::JoinHandle<i32>> = Vec::new();

    for i in 0..10 {
        // returns a handle that can be waited on to retrieve the result
        let handle = thread::spawn(move || { // Move local variables into the closure
            i
        });
        handles.push(handle);
    }

    let vec = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", vec);
}

fn actor_channel() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let thread_tx = tx.clone(); // Transmit channel for this thread
        thread::spawn(move || { // Move local variables into the closure
            thread_tx.send(i).unwrap();
            thread_tx.send(i).unwrap();
        });
        // i and thread_tx are now owned by the closure.
    }

    for _ in 0..20 {
        print!("{:?} ", rx.recv().unwrap());
    }
}

fn read_only() {
    // Atomically Reference Counted ~= "&"
    let vec = Arc::new(vec![10,20,33,44,50,100,125,300,322,42]);
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let thread_tx = tx.clone();
        let thread_vec = vec.clone(); // Reference Count ++
        thread::spawn(move || {
            thread_tx.send(thread_vec[i]).unwrap();
        }); // Reference Count --
    }

    for _ in 0..10 {
        print!("{:?} ", rx.recv().unwrap());
    }
} // Reference Count --

fn locking() {
    let arc_data = Arc::new(Mutex::new(0u32));
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (thread_mutex, thread_tx) = (arc_data.clone(), tx.clone());
        thread::spawn(move || {
            let mut unlocked_data = thread_mutex.lock().unwrap();
            *unlocked_data += 1;
            thread_tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
    println!("{:?}", *arc_data.lock().unwrap());
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
    // Tricky type: Arc (Atomically Reference Counted) is "Send", Mutex protects shared mutability
    let arc_data = Arc::new(Mutex::new(vec![1u32, 2, 3]));
    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new(); // threads return empty "()"

    for i in 0..3 {
        let thread_arc_ref = arc_data.clone(); // Clone Arc to get a handle to the mutex
        let handle = thread::spawn(move || {
            let mut unlocked_vec = thread_arc_ref.lock().unwrap(); // Auto deref to Mutex
            unlocked_vec[i] += 1; // MutexGuard<Vec<u32>> 
        });
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>();
    println!("data: {:?}", *arc_data.lock().unwrap());
}