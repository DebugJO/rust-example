fn fun1(s: &mut String) {
    s.push_str(", World");
}

fn fun2(s: String) -> String {
    let result = s + &", World".to_string();
    result
}

fn fun3(s: &str) -> &'static str {
    let result = s.to_owned() + ", World";
    Box::leak(result.into_boxed_str())
}

fn main() {
    let mut a = "Hello".to_string();
    fun1(&mut a);
    println!("{}", a);

    let b = "Hello".to_string();
    let result = fun2(b);
    println!("{}", result);

    let c = "Hello";
    let result = fun3(c);
    println!("{}", result);
}
