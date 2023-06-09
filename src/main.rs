use rocket::{get, http::Status, launch, routes, serde::json::Json};

#[get("/")]
fn setup() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Setup is done")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![setup])
}
