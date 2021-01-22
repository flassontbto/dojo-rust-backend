use db::establish_connection;
use diesel::{RunQueryDsl, SqliteConnection, result::Error};
use queries::{bin_queries::create_bin, like_queries::create_like, user_queries::{create_user, delete_user, get_users, update_user}};
use schema::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod schema;
mod queries;

pub fn clean(conn: &SqliteConnection) -> Result<(), Error> {
    use crate::likes::dsl::*;
    use crate::users::dsl::*;
    use crate::bins::dsl::*;

    diesel::delete(likes).execute(conn)?;
    diesel::delete(bins).execute(conn)?;
    diesel::delete(users).execute(conn)?;
    Ok(())
}

pub fn test() {
    let conn = establish_connection();
    create_user(&conn, "test");
    create_user(&conn, "test2");

    for user in get_users(&conn) {
        println!("{}", user.pseudo);
        update_user(&conn, user.id, &(user.pseudo + "HAHA") as &str);
    }

    for user in get_users(&conn) {
        println!("{}", user.pseudo);
        delete_user(&conn, user.id);
    }

    assert_eq!(get_users(&conn).len(), 0);

    let user = create_user(&conn, "test");
    let bin = create_bin(&conn, "THISISATEST", user.id, "let a = 1;");
    let user2 = create_user(&conn, "test2");
    create_like(&conn, user2.id, bin.id);

    clean(&conn).unwrap();
}

fn main() {
    test();
}
