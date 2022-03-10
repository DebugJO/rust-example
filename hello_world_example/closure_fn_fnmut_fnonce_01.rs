fn fn_closure<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn fn_mut_closure<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn fn_once_closure<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let mut my_string = String::from("Hello");
    let mut s = my_string.clone();

    fn_closure(|| {
        println!("fn_closure : {}", my_string);
    });

    fn_mut_closure(|| {
        my_string.push_str(" World");
        println!("fn_mut_closure : {}", my_string);
    });

    fn_once_closure(|| {
        s.push_str(" 월드");
        println!("fn_once_closure : {}", s);
        drop(my_string);
        drop(s);
    });
}
  
// fn_closure : Hello
// fn_mut_closure : Hello World
// fn_once_closure : Hello  
