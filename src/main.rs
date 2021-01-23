#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod db;
mod models;
mod queries;
mod schema;

use api::*;
use schema::*;

#[rocket::launch]
fn start() -> rocket::Rocket {
    rocket::ignite()
        .manage({
            use api::auth::*;
            UserTable::new(vec![UserLoginInfo::new("flo".into(), "toto".into())])
        })
        .manage(api::BinStorage::default())
        .mount("/auth", rocket::routes![auth::login])
        .mount("/", rocket::routes![index, create_bin, get_bin])
}
