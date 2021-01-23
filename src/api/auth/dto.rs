#[derive(rocket::request::FromForm, Hash, PartialEq, Eq)]
pub struct UserLoginInfo {
    name: String,
    password: String,
}

impl UserLoginInfo {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        &self.name
    }
}
