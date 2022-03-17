#[derive(Debug)]
struct User {
    name: String,
}

impl User {
    fn new(input: &str) -> Self {
        Self { name: input.to_string() }
    }
}

use std::borrow::Cow;

struct UserCow<'a> {
    name: Cow<'a, str>,
}

fn main() {
    let name_1 = "가나닭";
    let name_2 = "마바삵".to_string();

    let my_user1 = User::new(name_1);
    let my_user2 = User::new(&name_2);

    println!("String : {}, {}", my_user1.name, my_user2.name);

    let name_1 = "가나닭";
    let name_2 = "마바삵".to_string();

    let my_user1 = UserCow { name: name_1.into() };
    let my_user2 = UserCow { name: name_2.into() };

    println!("CowStr : {}, {}", my_user1.name, my_user2.name);
}


// String : 가나닭, 마바삵
// CowStr : 가나닭, 마바삵
