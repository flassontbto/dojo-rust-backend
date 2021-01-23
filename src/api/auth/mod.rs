use rocket::{
    http::{Cookie, CookieJar, Status},
    request::Form,
};

mod dto;
mod guard;

use crate::db::Database;

pub use self::dto::LoginDto;
pub use self::guard::User;

#[rocket::post("/login", data = "<info>")]
pub async fn login<'a>(info: Form<LoginDto>, connection: Database, cookies: &CookieJar<'a>) -> Status {
    let user = connection
        .run(move |c| crate::queries::user_queries::by_name(c, info.get_name()))
        .await;
    match user {
        Ok(Some(User { id, ..  })) => {
            cookies.add(Cookie::new(self::guard::USER_COOKIE_NAME,id.to_string()));
            Status::Ok
        }
        Ok(None) => Status::Unauthorized,
        Err(_) => Status::InternalServerError
    }
}
