fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec1 = vec![1, 2, 3, 4, 5];
    let index1 = take_fifth(new_vec1);

    match index1 {
        Some(number) => println!("I got a number: {}", number),
        None => println!("There was nothing inside"),
    }

    let new_vec2 = vec![1, 2];
    let index2 = take_fifth(new_vec2);

    if index2.is_some() {
        println!("I got a number: {}", index2.unwrap());
    } else {
        println!("There was nothing inside");
    }
}
