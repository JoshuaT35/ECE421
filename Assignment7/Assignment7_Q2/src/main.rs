use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    // store thread handles
    let mut main_thread_handles = Vec::with_capacity(10);

    let sample_data: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 81, 107]));

    for i in 0..10 {
        // clone Arc pointer
        let sample_data_clone = Arc::clone(&sample_data);
        let thread_handle = thread::spawn(move || {
            {
                let mut sample_data_lock = sample_data_clone.lock().unwrap();
                sample_data_lock[0] += i;
            }
        });

        // store handle
        main_thread_handles.push(thread_handle);
    }

    thread::sleep(Duration::from_millis(50));

    // main waits
    for handle in main_thread_handles {
        handle.join().unwrap();
    }
}
