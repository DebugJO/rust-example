use std::cell::Cell; // not tread safe, set, get

fn main() {
    let my_cell = Cell::new(String::from("Hello World"));

    my_cell.set(String::from("가나닭"));

    let result = my_cell.into_inner();

    println!("{}", result);
}
