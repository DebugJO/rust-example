#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    let result: Rocket<Build> = rocket::build().mount("/", routes![index]);
    result
}
