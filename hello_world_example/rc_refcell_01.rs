use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct DataContainer {
    data: Rc<RefCell<String>>,
}

fn main() {
    let my_data = Rc::new(RefCell::new("홍길동".to_string()));

    let con1 = DataContainer { data: Rc::clone(&my_data) };

    let con2 = DataContainer { data:Rc::clone(&my_data) };

    for _ in 0..10 {
        *con1.data.borrow_mut() = String::from("가나닭");
        *con2.data.borrow_mut() = String::from("마바삵");
    }

    let a = con1.data.borrow_mut().to_string();

    *con2.data.borrow_mut() = String::from("가나닭");

    println!("{}, {}", a, &con2.data.borrow_mut().to_string());
    // 마바삵, 가나닭
}
