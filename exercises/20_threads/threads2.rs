// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            // Option 1: Named Bindings, not so good, the lock is held until out of the scope
            // let mut status_guard = status_shared.lock().unwrap();
            // status_guard.jobs_done += 1;

            // Option 2: Temporaries, better, the lock is released as soon as possible
            // status_shared.lock().unwrap().jobs_done += 1;
            match status_shared.lock() {
                Ok(mut status) => status.jobs_done += 1,
                Err(e) => println!("Failed to lock: {:?}", e),
            }
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}
