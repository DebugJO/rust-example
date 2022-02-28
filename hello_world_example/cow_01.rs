use std::borrow::Cow;

// TODO:Check: fn my_module<'a>(input: u8) -> Cow<'a, str> {
fn my_module(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {remainder}").into(),
    }
}

fn fake_api_request() -> String {
    "홍길동".to_string()
}

struct Name<'a> {
    name: Cow<'a, str>,
}

impl<'a> Name<'a> {
    pub fn new<S>(input: S) -> Name<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Name { name: input.into() }
    }
}

fn main() {
    for number in 0..10 {
        match my_module(number) {
            Cow::Borrowed(message) => println!("{} Borrowed : {}", number, message),
            Cow::Owned(message) => println!("{} Owned : {}", number, message),
        }
    }

    // TODO:Check: https://velog.io/@pandawithcat/Rust-Smart-Pointer-Cow
    let first_name = Name::new("가나닭");
    let second_name = Name::new(fake_api_request());
    println!("{}", first_name.name);
    println!("{}", second_name.name);
}
