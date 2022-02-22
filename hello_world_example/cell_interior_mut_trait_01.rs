use std::cell::Cell;

#[derive(Debug)]
struct User {
    id: u32,
    times_used: Cell<u32>,
}

trait SuperCoolTrait {
    fn cool_function(&self);
}

impl SuperCoolTrait for User {
    fn cool_function(&self) {
        println!("Now using cool_function : {:02}", self.times_used.get());
        let times_used = self.times_used.get();
        self.times_used.set(times_used + 1);
    }
}

fn main() {
    let user = User { id: 1234, times_used: Cell::new(0) };

    for _ in 0..20 {
        user.cool_function();
    }

    println!("id:{}, times_used:{}", user.id, user.times_used.get());
}

// Now using cool_function : 00
// Now using cool_function : 01
// Now using cool_function : 02
// Now using cool_function : 03
// Now using cool_function : 04
// Now using cool_function : 05
// Now using cool_function : 06
// Now using cool_function : 07
// Now using cool_function : 08
// Now using cool_function : 09
// Now using cool_function : 10
// Now using cool_function : 11
// Now using cool_function : 12
// Now using cool_function : 13
// Now using cool_function : 14
// Now using cool_function : 15
// Now using cool_function : 16
// Now using cool_function : 17
// Now using cool_function : 18
// Now using cool_function : 19
// id:1234, times_used:20
