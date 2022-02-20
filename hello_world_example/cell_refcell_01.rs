use std::cell::RefCell;

#[allow(unused)]
#[derive(Debug)]
struct User {
    id: u32,
    year: u32,
    name: String,
    active: RefCell<bool>,
}

fn main() {
    let user = User { id: 1, year: 2022, name: "홍길동".to_string(), active: RefCell::new(true) };

    println!("{:?}", user);

    let mut my_ref = user.active.borrow_mut();

    println!("{:?}", user);
    
    *my_ref = false;
    
    println!("{:?}", user);

    drop(my_ref);

    println!("{:?}", user);
}

// User { id: 1, year: 2022, name: "홍길동", active: RefCell { value: true } }
// User { id: 1, year: 2022, name: "홍길동", active: RefCell { value: <borrowed> } }
// User { id: 1, year: 2022, name: "홍길동", active: RefCell { value: <borrowed> } }
// User { id: 1, year: 2022, name: "홍길동", active: RefCell { value: false } }
