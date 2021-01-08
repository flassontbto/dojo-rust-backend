use rocket::{
    http::{Cookie, CookieJar, Status},
    request::Form,
};
use std::convert::Infallible;

const USER_COOKIE_NAME: &'static str = "user";

#[rocket::post("/login", data = "<info>")]
pub fn login(
    info: Form<UserLoginInfo>,
    users: rocket::State<UserTable>,
    cookies: &CookieJar,
) -> Status {
    if users.inner().contains(&info) {
        cookies.add(Cookie::new(USER_COOKIE_NAME, info.0.name));
        Status::Ok
    } else {
        Status::Unauthorized
    }
}

#[derive(rocket::request::FromForm, Hash, PartialEq, Eq)]
pub struct UserLoginInfo {
    name: String,
    password: String,
}

impl UserLoginInfo {
    pub fn new(name: String, password: String) -> Self {
        Self { name, password }
    }
}

pub struct UserTable(std::collections::HashSet<UserLoginInfo>);

impl UserTable {
    pub fn new(table: Vec<UserLoginInfo>) -> Self {
        Self(table.into_iter().collect())
    }

    pub fn contains(&self, info: &UserLoginInfo) -> bool {
        self.0.contains(&info)
    }
}

pub struct User(String);

impl<'a> rocket::request::FromParam<'a> for User {
    type Error = Infallible;

    fn from_param(param: &'a rocket::http::RawStr) -> Result<Self, Self::Error> {
        Ok(User(param.to_string()))
    }
}

#[rocket::async_trait]
impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for User {
    type Error = Infallible;

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use rocket::request::Outcome;
        let jar = rocket::try_outcome!(request.guard::<&CookieJar>().await);
        match jar.get("user") {
            Some(user) => Outcome::Success(User(user.value().into())),
            None => Outcome::Forward(()),
        }
    }
}
