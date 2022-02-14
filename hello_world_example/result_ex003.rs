fn parse_number(number: &str) -> Result<i32, std::num::ParseIntError> {
    number.parse()
}

fn main() {
    let mut result_vec: Vec<Result<i32, std::num::ParseIntError>> = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("one"));
    result_vec.push(parse_number("7"));

    for number in result_vec {
        if number.is_ok() {
            println!("Ok: {:?}", number.unwrap())
        } else {
            println!("Err: {:?}", number.unwrap_err().kind())
        }
    }
}
