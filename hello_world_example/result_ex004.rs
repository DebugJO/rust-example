fn parse_number(number: &str) -> Result<i32, std::num::ParseIntError> {
    number.parse()
}

fn main() {
    let mut result_vec: Vec<Result<i32, std::num::ParseIntError>> = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("one"));
    result_vec.push(parse_number("7"));

    for index in 0..result_vec.iter().count() {
        if let Some(number) = result_vec.get(index) {
            println!("{:?}", number.as_ref().unwrap_or(&0));
        }
    }
}
