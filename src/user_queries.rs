use diesel::{insert_into, RunQueryDsl, SqliteConnection};
use diesel::{prelude::*, result::Error};

use crate::models::{NewUser, User};
use crate::schema::users;
use crate::users::dsl::*;

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

pub fn last_inserted_id(conn: &SqliteConnection) -> i32 {
    return diesel::select(last_insert_rowid)
        .first(conn)
        .expect("Nothing lastly inserted");
}

pub fn create_user<'a>(conn: &SqliteConnection, new_user: &NewUser) -> Result<User, Error> {
    insert_into(users::table).values(new_user).execute(conn)?;

    Ok(get_user(conn, last_inserted_id(conn))?.unwrap())
}

pub fn get_users(conn: &SqliteConnection) -> Result<Vec<User>, Error> {
    Ok(users.get_results(conn)?)
}

pub fn get_user(conn: &SqliteConnection, user_id: i32) -> Result<Option<User>, Error> {
    users.find(user_id).first(conn).optional()
}

pub fn by_name(conn: &SqliteConnection, user_name: &str) -> Result<Option<User>, Error> {
    users.filter(pseudo.eq(user_name)).first(conn).optional()
}

pub fn update_user(
    conn: &SqliteConnection,
    user_id: i32,
    user_pseudo: &str,
) -> Result<User, Error> {
    diesel::update(users.find(user_id))
        .set(pseudo.eq(user_pseudo))
        .execute(conn)?;

    Ok(get_user(conn, last_inserted_id(conn))?.unwrap())
}

pub fn delete_user(conn: &SqliteConnection, user_id: i32) -> Result<usize, Error> {
    Ok(diesel::delete(users.find(user_id)).execute(conn)?)
}
