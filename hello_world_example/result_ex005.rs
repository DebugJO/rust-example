fn main() {
    let item_vec = vec![vec!["홍길동", "홍길서", "홍길남", "홍길북", "10"], vec!["가나닭", "20", "30"]];

    for mut item in item_vec {
        while let Some(info) = item.pop() {
            if let Ok(number) = info.parse::<i32>() {
                println!("The number is: {}", number);
            }
        }
    }
}

// fn main() {
//     let item_vec = vec![vec!["홍길동", "10"], vec!["가나닭", "20", "30"]];

//     let mut result:Vec<i32> = Vec::new();
//     let mut result_error:Vec<String> = Vec::new();

//     for mut item in item_vec {
//         while let Some(info) = item.pop() {
//             if let Ok(number) = info.parse::<i32>() {
//                 result.push(number);
//                 println!("The number is: {}", number);
//             } else if let Err(e) = info.parse::<i32>() {
//                 result_error.push(e.to_string());
//                 println!("Error : {}", e);
//             }
//         }
//     }

//     result.sort();
//     println!("{:?} : {}", result, result.len());
//     println!("{:?} : {}", result_error, result_error.len());
// }
