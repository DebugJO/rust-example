fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_string()),
    }
}

fn main() {
    let mut result_vec = Vec::new(); // Vec<Result<i32, String>>

    for number in 2..=7 {
        result_vec.push(check_if_five(number));
    }

    println!("{:#?}", result_vec);
}
