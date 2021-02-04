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
    tracing_subscriber::fmt::init();
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/", rocket::routes![index])
        .mount("/", rocket::routes![user])
        .mount("/", rocket::routes![create])
}

#[tracing::instrument(skip(database))]
#[rocket::get("/")]
pub async fn index(_addr: std::net::SocketAddr, database: Database) -> Result<Json<Vec<User>>, ()> {
    tracing::debug!("GET /");
    database
        .run(|c| get_users(c))
        .await
        .map(Json)
        .map_err(|_| ())
}

#[tracing::instrument(skip(database))]
#[rocket::get("/<userid>")]
pub async fn user(userid: i32, database: Database) -> Result<Json<Option<User>>, ()> {
    database
        .run(move |c| get_user(c, userid))
        .await
        .map(Json)
        .map_err(|_| ())
}

#[derive(serde::Deserialize, Debug)]
pub struct NewUserDto {
    pseudo: String
}

#[tracing::instrument(skip(database))]
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