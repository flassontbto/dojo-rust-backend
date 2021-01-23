use rocket::http::CookieJar;
use std::convert::Infallible;

pub const USER_COOKIE_NAME: &'static str = "user";

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
