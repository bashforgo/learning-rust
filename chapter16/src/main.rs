use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let mutex_a = Arc::new(Mutex::new(0));
    let mutex_b = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let mutex_a = Arc::clone(&mutex_a);
        let mutex_b = Arc::clone(&mutex_b);
        let handle = thread::spawn(move || {
            println!("thread a starting");
            let _a = mutex_a.lock().unwrap();
            thread::sleep(Duration::from_secs(1));
            let _b = mutex_b.lock().unwrap();
            println!("thread a done");
        });
        handles.push(handle);
    }

    {
        let mutex_a = Arc::clone(&mutex_a);
        let mutex_b = Arc::clone(&mutex_b);
        let handle = thread::spawn(move || {
            println!("thread b starting");
            let _b = mutex_b.lock().unwrap();
            thread::sleep(Duration::from_secs(1));
            let _a = mutex_a.lock().unwrap();
            println!("thread b done");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("done");
}
