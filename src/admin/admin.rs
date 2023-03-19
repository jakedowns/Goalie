use actix_web::{web, HttpResponse, Responder};

pub async fn games_list() -> impl Responder {
    // TODO: Implement games list
    HttpResponse::Ok().body("Games list")
}

pub async fn hide_user_from_scoreboards() -> impl Responder {
    // TODO: Implement hide user from scoreboards
    HttpResponse::Ok().body("User hidden from scoreboards")
}

pub async fn reports() -> impl Responder {
    // TODO: Implement reports
    HttpResponse::Ok().body("Reports")
}
