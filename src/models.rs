use super::schema::*;
use serde::Serialize;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
}

#[derive(Queryable, Serialize)]
pub struct Bin {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub code: String,
}

#[derive(Queryable)]
pub struct Like {
    pub user_id: i32,
    pub bin_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub pseudo: &'a str,
}

#[derive(Insertable)]
#[table_name = "bins"]
pub struct NewBin {
    pub title: String,
    pub author_id: i32,
    pub code: String,
}

#[derive(Insertable)]
#[table_name = "likes"]
pub struct NewLike {
    pub user_id: i32,
    pub bin_id: i32,
}
