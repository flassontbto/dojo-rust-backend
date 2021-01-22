use diesel::prelude::*;
use diesel::{insert_into, RunQueryDsl};

use crate::likes::dsl::*;
use crate::models::{Like, NewLike};
use crate::schema::likes;

pub fn create_like<'a>(conn: &SqliteConnection, like_user_id: i32, like_bin_id: i32) -> Like {
    let new_like = NewLike {
        user_id: like_user_id,
        bin_id: like_bin_id,
    };

    insert_into(likes::table)
        .values(&new_like)
        .execute(conn)
        .expect("Error saving new like");

    return get_like(conn, like_user_id, like_bin_id);
}

pub fn get_likes(conn: &SqliteConnection) -> Vec<Like> {
    return likes.get_results(conn).expect("Error loading likes");
}

pub fn get_likes_for_user(conn: &SqliteConnection, id: i32) -> Vec<Like> {
    return likes
        .filter(user_id.eq(id))
        .get_results(conn)
        .expect("Error loading likes");
}

pub fn get_likes_for_bin(conn: &SqliteConnection, id: i32) -> Vec<Like> {
    return likes
        .filter(bin_id.eq(id))
        .get_results(conn)
        .expect("Error loading likes");
}

pub fn get_like(conn: &SqliteConnection, like_user_id: i32, like_bin_id: i32) -> Like {
    return likes
        .find((like_user_id, like_bin_id))
        .first(conn)
        .expect("Unable to retrieve bin");
}

pub fn delete_like(conn: &SqliteConnection, like_user_id: i32, like_bin_id: i32) -> usize {
    return diesel::delete(likes.find((like_user_id, like_bin_id)))
        .execute(conn)
        .expect("Error deleting user");
}