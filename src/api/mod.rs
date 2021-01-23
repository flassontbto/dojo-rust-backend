use std::{collections::HashMap, sync::Arc};

use rocket::http::Status;
use rocket::{tokio::sync::RwLock, Response};
use rocket_contrib::json::Json;

use crate::models::{Bin, NewBin};

use self::auth::User;

pub mod auth;

#[rocket::get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/<binid>")]
pub async fn get_bin<'a>(
    binid: i32,
    connection: crate::db::Database,
) -> Result<Option<Json<Bin>>, ()> {
    connection
        .run(move |c| crate::queries::bin_queries::get_bin(c, binid))
        .await
        .map(|maybe_bin| maybe_bin.map(Json))
        .map_err(|_| ())
}

#[derive(serde::Deserialize)]
pub struct NewBinDto {
    title: String,
    code: String,
}

#[rocket::post("/me/bins", data = "<dto>", format = "json")]
pub async fn create_bin<'a>(
    user: User,
    dto: Json<NewBinDto>,
    connection: crate::db::Database,
) -> rocket::response::Result<'a> {
    let insert = NewBin {
        author_id: user.id,
        title: dto.title.clone(),
        code: dto.code.clone(),
    };
    let bin = connection
        .run(move |c| crate::queries::bin_queries::create_bin(c, &insert))
        .await
        .map_err(|_| Status::InternalServerError)?;
    Response::build()
        .raw_header(
            rocket::http::hyper::header::LOCATION.as_str(),
            rocket::uri!(get_bin: binid = bin.id).to_string(),
        )
        .ok()
}
