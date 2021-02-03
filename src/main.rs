#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod models;
mod schema;

fn main() {
    let connection = establish_connection();
    let all_users: Vec<models::User> = schema::users::table
        .get_results(&connection)
        .expect("Query failed");
    for user in all_users {
        println!("Utilisateur {}", user.pseudo);
    }
}

fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
