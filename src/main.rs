mod api;

use api::*;

#[rocket::launch]
fn start() -> rocket::Rocket {
    rocket::ignite()
        .manage({
            use api::auth::*;
            UserTable::new(vec![UserLoginInfo::new("flo".into(), "toto".into())])
        })
        .manage(api::BinStorage::default())
        .mount("/auth", rocket::routes![auth::login])
        .mount("/", rocket::routes![index, create_bin, get_bin])
}
