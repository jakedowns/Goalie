use diesel::prelude::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub verified_at: Option<chrono::NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub verification_token: Option<String>,
    pub last_logged_in: Option<chrono::NaiveDateTime>,
}
