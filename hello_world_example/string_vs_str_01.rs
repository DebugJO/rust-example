// fn main() {
//     let s = "Jane Doe".to_string();
//     say_hello(&s);
// }

// fn say_hello(name: &str) {
//     println!("Hello {}!", name);
// }

// https://prev.rust-lang.org/ko-KR/faq.html#strings

fn main() {
    let c: &str = "World"; // immutable
    let s: String = c.to_string();
    let result = say_hello(&s);
    println!("{}", result);

    let mut my_str = "가나닭"; // to mutable
    println!("{}", my_str);
    my_str = &"마바삵";
    println!("{}", my_str);
}

fn say_hello(name: &str) -> String {
    let s: String = String::from("Hello");
    let c: char = 32 as char;
    s + &c.to_string() + &name.to_string()
}
