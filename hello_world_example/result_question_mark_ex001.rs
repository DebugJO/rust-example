use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?; // return Error
    Ok(parsed_number)
}

fn main() {
    for item in vec!["one", "2", "3"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }

    println!("-----------------");

    let my_vec = vec!["one", "2", "3"];
    let result = my_vec.iter().filter_map(|&s| parse_str(s).ok()).collect::<Vec<_>>();
    for item in result {
        println!("{:?}", item);
    }
}
