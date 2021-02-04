#[derive(diesel::Queryable, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
}