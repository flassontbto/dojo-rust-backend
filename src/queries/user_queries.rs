use diesel::prelude::*;
use diesel::{insert_into, RunQueryDsl};

use crate::models::{NewUser, User};
use crate::schema::users;
use crate::users::dsl::*;

use super::helpers::last_inserted_id;

pub fn create_user<'a>(conn: &SqliteConnection, user_pseudo: &'a str) -> User {
    let new_user = NewUser {
        pseudo: user_pseudo,
    };

    insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new user");

    return get_user(conn, last_inserted_id(conn));
}

pub fn get_users(conn: &SqliteConnection) -> Vec<User> {
    return users.get_results(conn).expect("Error loading users");
}

pub fn get_user(conn: &SqliteConnection, user_id: i32) -> User {
    return users
        .find(user_id)
        .first(conn)
        .expect("Unable to retrieve user");
}

pub fn update_user(conn: &SqliteConnection, user_id: i32, user_pseudo:  &str) -> User {
    diesel::update(users.find(user_id))
    .set(pseudo.eq(user_pseudo))
    .execute(conn)
    .expect("Error updating user");

    return get_user(conn, last_inserted_id(conn));
}

pub fn delete_user(conn: &SqliteConnection, user_id: i32) -> usize {
    return diesel::delete(users.find(user_id))
        .execute(conn)
        .expect("Error deleting user");
}
