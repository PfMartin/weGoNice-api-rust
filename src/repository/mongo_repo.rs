use dotenv::dotenv;
use std::env;

use crate::models::user_model::User;
use mongodb::{
    bson::extjson::de::Error,
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("DATABASE_URI") {
            Ok(var) => var,
            Err(_) => String::from("Error loading uri from environment"),
        };

        let db_name = match env::var("DATABASE_NAME") {
            Ok(var) => var,
            Err(_) => String::from("Error loading database name from environment"),
        };

        let user_name = match env::var("DATABASE_USERNAME") {
            Ok(var) => var,
            Err(_) => String::from("Error loading database user name from environment"),
        };

        let password = match env::var("DATABASE_PASSWORD") {
            Ok(var) => var,
            Err(_) => String::from("Error loading database password from environment"),
        };

        let client = Client::with_uri_str(uri).expect("Failed to create MongoDB client");

        let db = client.database(&db_name);

        let col = db.collection("User");

        MongoRepo { col }
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .col
            .find(None, None)
            .expect("Error getting list of users");

        let users = cursors.map(|doc| doc.unwrap()).collect();

        Ok(users)
    }
}
