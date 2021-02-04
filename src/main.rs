use models::NewUser;
use rocket_contrib::json::Json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod user_queries;

use crate::models::User;
use schema::*;
use crate::user_queries::{get_users, create_user, get_user};

#[rocket_contrib::database("db")]
pub struct Database(diesel::SqliteConnection);

#[rocket::launch]
fn start() -> rocket::Rocket {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/", rocket::routes![index])
        .mount("/", rocket::routes![user])
        .mount("/", rocket::routes![create])
}

#[rocket::get("/")]
pub async fn index(database: Database) -> Result<Json<Vec<User>>, ()> {
    database
        .run(|c| get_users(c))
        .await
        .map(Json)
        .map_err(|_| ())
}

#[rocket::get("/<userid>")]
pub async fn user(userid: i32, database: Database) -> Result<Json<Option<User>>, ()> {
    database
        .run(move |c| get_user(c, userid))
        .await
        .map(Json)
        .map_err(|_| ())
}

#[derive(serde::Deserialize)]
pub struct NewUserDto {
    pseudo: String
}

#[rocket::post("/", data="<dto>", format="json")]
pub async fn create(dto: Json<NewUserDto>, database: Database) -> Result<Json<User>, ()> {
    let insert = NewUser {
        pseudo: dto.pseudo.clone()
    };
    database
        .run(move |c| create_user(c, &insert))
        .await
        .map(Json)
        .map_err(|_| ())
}