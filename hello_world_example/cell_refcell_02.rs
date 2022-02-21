use std::cell::RefCell;

fn main() {
    let my_cell = RefCell::new(String::from("가나닭"));
    println!("{:?}", my_cell);

    let blocking_reference = my_cell.borrow_mut();
    if let Ok(s) = String::from_utf8(blocking_reference.as_bytes().to_vec()) {
        println!("{}", s);
    }

    match my_cell.try_borrow_mut() {
        Ok(mut r) => *r = String::from("마바삵"),
        Err(e) => println!("Error : {}", e),
    }
    println!("{:?}", my_cell);
}

// RefCell { value: "가나닭" }
// 가나닭
// Error : already borrowed
// RefCell { value: <borrowed> }
