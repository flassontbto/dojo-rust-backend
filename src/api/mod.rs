use std::{collections::HashMap, sync::Arc};

use rocket::{data::ToByteUnit, http::RawStr, tokio::sync::RwLock, Response};
use rocket::{http::Status, State};

use self::auth::User;

pub mod auth;

#[rocket::get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/<binid>")]
pub async fn get_bin<'a>(binid: &'a RawStr, storage: State<'a, BinStorage>) -> Option<String> {
    let storage = storage.as_ref().read().await;
    storage.get(binid.as_str()).map(|bin| bin.clone())
}

#[rocket::post("/me/bins", data = "<bin>")]
pub async fn create_bin<'a>(
    _user: User,
    bin: rocket::data::Data,
    storage: State<'a, BinStorage>,
) -> rocket::response::Result<'a> {
    let uuid = uuid::Uuid::new_v4().to_simple().to_string();
    let mut storage = storage.as_ref().write().await;
    storage.insert(
        uuid.clone(),
        bin.open(2u32.mebibytes())
            .stream_to_string()
            .await
            .map_err(|_| Status::PayloadTooLarge)?,
    );
    Response::build()
        .raw_header(
            rocket::http::hyper::header::LOCATION.as_str(),
            rocket::uri!(get_bin: binid = uuid).to_string(),
        )
        .ok()
}

pub struct BinStorage(Arc<RwLock<HashMap<String, String>>>);

impl Default for BinStorage {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(HashMap::new())))
    }
}

impl AsRef<Arc<RwLock<HashMap<String, String>>>> for BinStorage {
    fn as_ref(&self) -> &Arc<RwLock<HashMap<String, String>>> {
        &self.0
    }
}
