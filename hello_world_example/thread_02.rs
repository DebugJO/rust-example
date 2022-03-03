use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    {
        let p = data.clone();
        thread::spawn(move || {
            for _ in 0..4 {
                *p.lock().unwrap() += 1;
                println!("{:?}", p.lock().unwrap());
            }
        })
        .join().unwrap();
    }

    println!("Result : {}", &data.lock().unwrap());
}
