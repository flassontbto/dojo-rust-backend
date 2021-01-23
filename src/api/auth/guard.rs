use rocket::http::Status;

use crate::{db::Database, queries::user_queries::get_user};
pub use crate::models::User;

pub const USER_COOKIE_NAME: &'static str = "user";

#[rocket::async_trait]
impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for User {
    type Error = ();

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        use rocket::request::Outcome;
        let jar = request
            .cookies()
            .get(USER_COOKIE_NAME)
            .map(|cookie| cookie.value().parse::<i32>())
            .transpose();
        match jar {
            Ok(Some(user_id)) => {
                let connection: Database = rocket::try_outcome!(request.guard().await);
                match connection.run(move |c| get_user(c, user_id)).await {
                    Ok(Some(user)) => Outcome::Success(user),
                    Ok(None) => Outcome::Failure((Status::BadRequest, ())),
                    Err(_) => Outcome::Failure((Status::InternalServerError, ())),
                }
            }
            Ok(None) => Outcome::Forward(()),
            Err(_) => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}
