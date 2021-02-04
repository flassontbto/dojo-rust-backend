use super::schema::*;

#[derive(diesel::Queryable, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub pseudo: String,
}