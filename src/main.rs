mod api;
mod database;
mod models;
mod utils;

use api::user_api::get_all_users;

use database::database::DB;
use rocket::{launch, routes};

#[launch]
async fn rocket() -> _ {
    let db = match DB::new().await {
        Ok(d) => d,
        Err(err) => panic!("Couldn't connect to User db: {}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/", routes![get_all_users])
}
