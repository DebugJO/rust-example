use std::sync::{Arc, Mutex};
use std::thread;

trait CoolTrait {
    fn cool_function(&self);
}

#[derive(Debug)]
struct MyStruct {
    data: Arc<Mutex<u8>>,
}

impl CoolTrait for MyStruct {
    fn cool_function(&self) {
        *self.data.lock().unwrap() += 1;
    }
}

fn main() {
    let my_struct = MyStruct { data: Arc::new(Mutex::new(0)) };

    let mut join_vec = vec![];

    for _ in 0..10 {
        let clone = Arc::clone(&my_struct.data);
        let join_handle = thread::spawn(move || {
            *clone.lock().unwrap() += 1;
            println!(" owners : {}", Arc::strong_count(&clone));
        });
        join_vec.push(join_handle);
    }

    for handle in join_vec {
        handle.join().unwrap();
    }

    println!("{:?}", my_struct);
}
