#[derive(Debug)]
struct Book {
    name: &'static str,
}

#[derive(Debug)]
struct MyBook<'a> {
    name: &'a str,
}

#[derive(Debug)]
struct Adventurer<'a> {
    name: &'a str,
    hit_porints: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) -> i32 {
        self.hit_porints -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_porints);
        self.hit_porints as i32
    }
}

fn main() {
    // [1] Book
    let my_book = Book { name: "my book" };
    println!("{:?}\n{}", my_book, my_book.name);

    // [2] MyBook
    let my_book_title = "my_book_title".to_string();
    let my_book = MyBook { name: &my_book_title };
    println!("{:?}\n{}", my_book, my_book.name);

    // [3] Adventurer
    let mut adventurer = Adventurer { name: "홍길동", hit_porints: 100 };
    println!("{} : {}", adventurer.name, adventurer.hit_porints);
    let last = adventurer.take_damage();
    println!("{} : {}", adventurer.name, last);
}
