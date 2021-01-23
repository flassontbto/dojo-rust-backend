use rocket::{
    http::{Cookie, CookieJar, Status},
    request::Form,
};

mod dto;
mod guard;
mod storage;

pub use self::dto::UserLoginInfo;
pub use self::guard::User;
pub use self::storage::UserTable;

#[rocket::post("/login", data = "<info>")]
pub fn login(
    info: Form<UserLoginInfo>,
    users: rocket::State<UserTable>,
    cookies: &CookieJar,
) -> Status {
    if users.inner().contains(&info) {
        cookies.add(Cookie::new(
            self::guard::USER_COOKIE_NAME,
            info.0.get_name().to_owned(),
        ));
        Status::Ok
    } else {
        Status::Unauthorized
    }
}
