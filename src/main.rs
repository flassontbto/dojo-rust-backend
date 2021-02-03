#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

#[rocket::launch]
fn start() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", rocket::routes![index])
}

#[rocket::get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
