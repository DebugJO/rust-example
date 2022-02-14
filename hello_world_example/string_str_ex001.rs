struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Self {
        Self { first_name: first.to_string(), last_name: last.to_string() }
    }

    fn get_name_string(&self) -> String {
        return self.last_name.to_string() + &self.first_name.to_string();
    }

    fn get_name_str1(&self) -> &str {
        let a = self.last_name.to_string() + &self.first_name.to_string();
        return Box::leak(a.into_boxed_str());
    }

    fn get_name_str2(&self) -> &str {
        let b = self.last_name.to_string() + &self.first_name.to_string();
        return string_to_static_str(b);
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn run() {
    let p = Person::new("길동", "홍");
    println!("Person: {} / {} / {}", p.get_name_string(), p.get_name_str1(), p.get_name_str2());
}

fn main() {
    run();
}
