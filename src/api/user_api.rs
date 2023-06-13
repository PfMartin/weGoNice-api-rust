use crate::{database::users_client::UsersClient, models::user_model::User};
use rocket::{get, http::Status, serde::json::Json, State};

#[get("/users")]
pub async fn get_all_users(db: &State<UsersClient>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}
