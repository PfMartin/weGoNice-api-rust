use crate::{models::user_model::User, repository::mongo_repo::MongoRepo};
use rocket::{get, http::Status, serde::json::Json, State};

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}
