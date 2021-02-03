#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

#[rocket_contrib::database("db")]
pub struct Database(diesel::SqliteConnection);

#[rocket::launch]
fn start() -> rocket::Rocket {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/", rocket::routes![index])
}

#[rocket::get("/")]
pub async fn index(database: Database) -> Result<String, ()> {
    use crate::models::User;
    use diesel::prelude::*;

    let users: Result<Vec<User>, _> = database
        .run(|c| crate::schema::users::table.get_results::<crate::models::User>(c))
        .await;
    users
        .map(|users| {
            users
                .iter()
                .map(|user| format!("{}\n", user.pseudo))
                .collect::<String>()
        })
        .map_err(|_| ())
}
