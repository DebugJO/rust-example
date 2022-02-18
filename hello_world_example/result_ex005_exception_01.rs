fn main() {
    let item_vec = vec![vec!["홍길동", "10"], vec!["가나닭", "20", "30"]];

    for mut item in item_vec {
        while let Some(info) = item.pop() {
            if let Ok(number) = info.parse::<i32>() {
                println!("The number is: {}", number);
            } else if let Err(e) = info.parse::<i32>() {
                println!("Error : {}", e);
            }
        }
    }
}
