pub mod handler;
pub mod model;

use serde::{Deserialize, Serialize};
use mongodb::bson;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertableUser {
    pub name: String,
    pub role: Option<String>,
}

impl InsertableUser {
    fn from_user(user: User) -> InsertableUser {
        InsertableUser {
            name: user.name,
            role: user.role,
        }
    }
}