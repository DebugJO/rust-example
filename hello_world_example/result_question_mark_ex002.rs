use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    Ok(parsed_number)
}

fn main() {
    let mut result_number: Vec<i32> = Vec::new();
    let mut result_string: Vec<String> = Vec::new();

    for item in vec!["one", "2", "3"] {
        let parsed = parse_str(item);
        if let Ok(number) = parsed {
            result_number.push(number);
        } else if let Err(e) = parsed {
            result_string.push(item.to_string() + "#:$" + &e.to_string());
        }
    }

    println!("{:?}", result_number);
    println!("{:?}", result_string);

    for n in result_number {
        println!("{}", n);
    }

    for s in result_string {
        println!("{}", s.split("#:$").next().unwrap());
        println!("{}", s.split("#:$").fold("".to_string(), |_, b| b.to_string()));
    }
}

// [2, 3]
// ["one#:$invalid digit found in string"]
// 2
// 3
// one
// invalid digit found in string
