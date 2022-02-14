fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    if check_error(5).is_ok() {
        println!("It's okay, guys!");
    } else {
        println!("It's an error, guys!");
    }

    match check_error(4) {
        Ok(..) => println!("It's okay, guys!"),
        Err(..) => println!("It's an error, guys!")
    }
}
