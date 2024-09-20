use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    // let mut data = vec![1, 2, 3];

    // for i in 0..3 {
    //     // Try to capture a mutable reference in multiple threads
    //     // This will fail to compile!
    //     thread::spawn(move || {
    //         data[i] += 1;
    //     });
    // }
    // // No data race can occur, this will not compile.

    // Using Mutex
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                data[i] += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);

    // Using RWLock
    let rw_data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let rw_handles: Vec<_> = (0..3)
        .map(|i| {
            let rw_data = Arc::clone(&rw_data);
            thread::spawn(move || {
                let mut rw_data = rw_data.write().unwrap();
                rw_data[i] += 1;
            })
        })
        .collect();

    for rw_handle in rw_handles {
        rw_handle.join().unwrap();
    }

    println!("{:?}", rw_data);
}
