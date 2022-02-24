use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    history: Rc<String>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let city = City { name: "시티".to_string(), population: 1234, history: Rc::new("가나닭".to_string()) };

    let city_data = CityData { names: vec![city.name], histories: vec![Rc::clone(&city.history)] };

    println!("history : {}", city.history);

    println!("city_data : {:?}", city_data);

    println!("owners : {}", Rc::strong_count(&city.history));
}
      
// history : 가나닭
// city_data : CityData { names: ["시티"], histories: ["가나닭"] }
// owners : 2
