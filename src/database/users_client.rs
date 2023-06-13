use dotenv::dotenv;
use rocket::futures::TryStreamExt;
use std::env;

use crate::models::user_model::User;
use crate::utils::types::CustomError;
use mongodb::{bson::extjson::de::Error, options::ClientOptions, Client, Collection};

pub struct UsersClient {
    col: Collection<User>,
}

impl UsersClient {
    pub async fn new() -> Result<UsersClient, CustomError> {
        let uri = compute_uri()?;
        let mut client_options = ClientOptions::parse(&uri).await?;

        client_options.app_name = Some(String::from("WeGoNice"));

        let user_db = Client::with_options(client_options)?.database("weGoNice");

        let col = user_db.collection("users");

        Ok(UsersClient { col })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self.col.find(None, None).await.expect("DIDN'T WORK");

        let users = vec![];

        Ok(users)
    }
}

fn compute_uri() -> Result<String, CustomError> {
    dotenv().ok();

    let db_username = env::var("DATABASE_USERNAME")?;
    let db_password = env::var("DATABASE_PASSWORD")?;
    let db_host = env::var("DATABASE_HOST")?;
    let db_port = env::var("DATABASE_PORT")?;
    let db_name = env::var("DATABASE_NAME")?;

    let uri = format!(
        "mongodb://{}:{}@{}:{}/?authSource={}",
        db_username, db_password, db_host, db_port, db_name
    );

    Ok(uri)
}
