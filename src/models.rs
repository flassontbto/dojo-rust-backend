#[derive(diesel::Queryable)]
pub struct User {
    pub id: i32,
    pub pseudo: String,
}