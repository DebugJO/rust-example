use std::fmt::Display;

fn return_closure1() -> Box<dyn Fn(i32)> {
    Box::new(|x| println!("{}", x))
}

fn return_closure2() -> impl Fn(i32) {
    |x| println!("{}", x)
}

fn generic_fun<T: Display>(input: T) {
    println!("{}", input);
}

fn impl_fun(input: impl Display) {
    println!("{}", input);
}

fn main() {
    let my_number = 7;

    let my_closure = return_closure1();
    my_closure(my_number);

    let my_closure = return_closure2();
    my_closure(my_number);

    generic_fun::<u8>(8);
    impl_fun(8);
}

// 7
// 7
// 8
// 8
