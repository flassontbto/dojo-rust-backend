use diesel::{insert_into, RunQueryDsl};
use diesel::{prelude::*, result::Error};

use crate::likes::dsl::*;
use crate::models::{Like, NewLike};
use crate::schema::likes;

pub fn create_like<'a>(conn: &SqliteConnection, new_like: &NewLike) -> Result<Like, Error> {
    insert_into(likes::table).values(new_like).execute(conn)?;

    Ok(get_like(conn, new_like.user_id, new_like.bin_id)?.unwrap())
}

pub fn get_likes(conn: &SqliteConnection) -> Result<Vec<Like>, Error> {
    Ok(likes.get_results(conn)?)
}

pub fn get_likes_for_user(conn: &SqliteConnection, id: i32) -> Result<Vec<Like>, Error> {
    Ok(likes.filter(user_id.eq(id)).get_results(conn)?)
}

pub fn get_likes_for_bin(conn: &SqliteConnection, id: i32) -> Result<Vec<Like>, Error> {
    Ok(likes.filter(bin_id.eq(id)).get_results(conn)?)
}

pub fn get_like(
    conn: &SqliteConnection,
    like_user_id: i32,
    like_bin_id: i32,
) -> Result<Option<Like>, Error> {
    likes
        .find((like_user_id, like_bin_id))
        .first(conn)
        .optional()
}

pub fn delete_like(
    conn: &SqliteConnection,
    like_user_id: i32,
    like_bin_id: i32,
) -> Result<usize, Error> {
    Ok(diesel::delete(likes.find((like_user_id, like_bin_id))).execute(conn)?)
}
