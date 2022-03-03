use std::thread;

fn main() {
    let mut join_vec = vec![];

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("printing : {}", i);
        });
        join_vec.push(handle);
    }

    for handle in join_vec {
        handle.join().unwrap();
    }
}
