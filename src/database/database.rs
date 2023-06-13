use dotenv::dotenv;
use std::env;
use tokio_stream::StreamExt;

use crate::models::user_model::User;
use crate::utils::types::CustomError;
use mongodb::{options::ClientOptions, Client, Collection, Database};

pub struct DB {
    database: Database,
}

impl DB {
    pub async fn new() -> Result<DB, CustomError> {
        dotenv().ok();
        let uri = compute_uri()?;
        let mut client_options = ClientOptions::parse(&uri).await?;

        client_options.app_name = Some(String::from("WeGoNice API"));

        let db_name = env::var("DATABASE_NAME")?;
        let database = Client::with_options(client_options)?.database(&db_name);

        Ok(DB { database })
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, CustomError> {
        let coll: Collection<User> = self.database.collection("users");

        let mut users = vec![];
        let mut cursor = coll.find(None, None).await?;
        while let Some(result) = cursor.try_next().await? {
            users.push(result);
        }

        Ok(users)
    }
}

fn compute_uri() -> Result<String, CustomError> {
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
