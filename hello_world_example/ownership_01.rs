// [1] --------------------------------------------------------------------
fn say_hi(name: String) {
    println!("Hi {}", name);
}

fn main() {
    let my_name = String::from("홍길동");

    say_hi(my_name);

    println!("{}", my_name); // Error
}

// [2] --------------------------------------------------------------------
fn say_hi(name: &String) {
    println!("Hi {}", name);
}

fn main() {
    let my_name = String::from("홍길동");

    say_hi(&my_name);

    println!("{}", my_name); // 정상
}

// [3] --------------------------------------------------------------------
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn say_hi(name: &str) {
    print_type_of(&name);
    print_type_of(&name.to_string());
    println!("Hi {} - {}", name, name.type_name());
    println!("Hi {} - {}", name.to_string(), name.to_string().type_name());
}

fn main() {
    // let my_name: String = String::from("홍길동");
    let my_name: &str = "홍길동";

    say_hi(&my_name);

    println!("{}", my_name); // 정상
}

// &str
// alloc::string::String
// Hi 홍길동 - &str
// Hi 홍길동 - alloc::string::String
// 홍길동
