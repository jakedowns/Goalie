use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::SqliteConnection;
use diesel::prelude::*;

// user_operations web resource /users route post to create_user
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use crate::db::DbConn;
use diesel::QueryResult;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/create", web::post().to(create_user_route))
}

pub async fn create_user_route(
    pool: web::Data<DbConn>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get a connection from the pool");

    use bcrypt::{hash, DEFAULT_COST};

    let hashed_password = hash(&user_data.password_hash, DEFAULT_COST)
        .expect("Failed to hash the password using bcrypt");

    let new_user = NewUser {
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        password_hash: hashed_password,
    };

    match create_user(&mut conn, new_user) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


pub fn create_user(conn: &mut SqliteConnection, user: NewUser) -> QueryResult<User> {
    use self::users::dsl::*;

    diesel::insert_into(users)
        .values(&user)
        .execute(conn)?;

    users.order(users::id.desc()).first(conn)
}
