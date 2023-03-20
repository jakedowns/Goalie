use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use juniper::FieldResult;

use crate::models::{NewUser, User};
use crate::schema::users;

pub async fn register(new_user: NewUser, pool: web::Data<DbPool>) -> FieldResult<User> {
    let conn = pool.get()?;
    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&conn)?;
    Ok(user)
}

pub async fn login(email: String, password: String, pool: web::Data<DbPool>) -> FieldResult<User> {
    let conn = pool.get()?;
    let user = users::table
        .filter(users::email.eq(&email))
        .filter(users::password.eq(&password))
        .first(&conn)?;
    Ok(user)
}

pub async fn forgot_password(email: String, pool: web::Data<DbPool>) -> FieldResult<String> {
    // TODO: Implement forgot password
    Ok("Forgot password".to_string())
}

pub async fn reset_password(email: String, new_password: String, pool: web::Data<DbPool>) -> FieldResult<String> {
    let conn = pool.get()?;
    let _ = diesel::update(users::table.filter(users::email.eq(&email)))
        .set(users::password.eq(&new_password))
        .execute(&conn)?;
    Ok("Password reset".to_string())
}

pub async fn verify_account_email(email: String, pool: web::Data<DbPool>) -> FieldResult<String> {
    // TODO: Implement account email verification
    Ok("Account email verified".to_string())
}
