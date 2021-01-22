use db::establish_connection;
use diesel::{result::Error, RunQueryDsl, SqliteConnection};
use models::{NewBin, NewLike, NewUser};
use queries::{
    bin_queries::create_bin,
    like_queries::create_like,
    user_queries::{create_user, delete_user, get_users, update_user},
};
use schema::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod queries;
mod schema;

pub fn clean(conn: &SqliteConnection) -> Result<(), Error> {
    use crate::bins::dsl::*;
    use crate::likes::dsl::*;
    use crate::users::dsl::*;

    diesel::delete(likes).execute(conn)?;
    diesel::delete(bins).execute(conn)?;
    diesel::delete(users).execute(conn)?;
    Ok(())
}

pub fn test() -> Result<(), Error> {
    let conn = establish_connection();
    create_user(&conn, &NewUser { pseudo: "test" })?;
    create_user(&conn, &NewUser { pseudo: "test" })?;

    for user in get_users(&conn)? {
        println!("{}", user.pseudo);
        update_user(&conn, user.id, &(user.pseudo + "HAHA") as &str)?;
    }

    for user in get_users(&conn)? {
        println!("{}", user.pseudo);
        delete_user(&conn, user.id)?;
    }

    assert_eq!(get_users(&conn)?.len(), 0);

    let user = create_user(&conn, &NewUser { pseudo: "test" })?;
    let bin = create_bin(
        &conn,
        &NewBin {
            title: "THISISATEST",
            author_id: user.id,
            code: "let a = 1;",
        },
    )?;
    let user2 = create_user(&conn, &&NewUser { pseudo: "test2" })?;
    create_like(
        &conn,
        &NewLike {
            user_id: user2.id,
            bin_id: bin.id,
        },
    )?;

    clean(&conn).unwrap();
    Ok(())
}

fn main() {
    test().expect("HAHA CA A PLANTE, ET MAINTENANT TU LES AIMES MES ?");
}
