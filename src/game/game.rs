use actix_web::{web, HttpResponse, Responder};

pub async fn create_game() -> impl Responder {
    // TODO: Implement game creation
    HttpResponse::Ok().body("Game created")
}

pub async fn edit_game() -> impl Responder {
    // TODO: Implement game editing
    HttpResponse::Ok().body("Game edited")
}

pub async fn delete_game() -> impl Responder {
    // TODO: Implement game deletion
    HttpResponse::Ok().body("Game deleted")
}

pub async fn view_scores() -> impl Responder {
    // TODO: Implement view scores
    HttpResponse::Ok().body("View scores")
}
