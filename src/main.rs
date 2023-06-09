mod api;
mod models;
mod repository;

use api::user_api::get_all_users;

use repository::mongo_repo::MongoRepo;
use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![get_all_users])
}
