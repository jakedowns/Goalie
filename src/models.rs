use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{users, moves, games};
use diesel::Identifiable;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub verified: Option<bool>,
    pub remember_me: Option<bool>,
    pub verification_token: Option<String>,
    pub password_reset_token: Option<String>,
    pub last_logged_in: Option<chrono::NaiveDateTime>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

// Move struct
#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Move {
    pub id: i32,
    pub player_id: i32,
    pub round_id: Option<i32>,
    pub game_id: i32,
    pub points_id: Option<i32>,
    pub times_id: Option<i32>,
    pub move_type: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = moves)]
pub struct NewMove {
    pub player_id: i32,
    pub round_id: Option<i32>,
    pub game_id: i32,
    pub points_id: Option<i32>,
    pub times_id: Option<i32>,
    pub move_type: i32,
}

// MoveInput struct
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MoveInput {
    pub player_id: i32,
    pub round_id: Option<i32>,
    pub game_id: i32,
    pub points_id: Option<i32>,
    pub times_id: Option<i32>,
    pub move_type: i32,
}

// Game struct
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub creator_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[diesel(table_name = games)]
pub struct NewGame {
    pub name: String,
    pub creator_id: i32,
}

// NewGameInput struct
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct NewGameInput {
    pub name: String,
}