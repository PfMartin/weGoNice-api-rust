use crate::{database::database::DB, models::user_model::User};

use rocket::{get, http::Status, serde::json::Json, State};

#[get("/users")]
pub async fn get_all_users(db: &State<DB>) -> Result<Json<Vec<User>>, Status> {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            println!("{}", err);
            Err(Status::InternalServerError)
        }
    }
}
