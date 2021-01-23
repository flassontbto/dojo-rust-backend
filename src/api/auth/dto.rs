#[derive(rocket::request::FromForm, Hash, PartialEq, Eq)]
pub struct LoginDto {
    name: String,
}

impl LoginDto {
    pub fn get_name<'a>(&'a self) -> &'a str {
        &self.name
    }
}
