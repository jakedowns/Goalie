use actix_web::{web, App, HttpServer};
mod auth;
mod game;
mod admin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(auth::register)
            .service(auth::login)
            .service(auth::forgot_password)
            .service(auth::reset_password)
            .service(auth::verify_account_email)
            .service(game::create_game)
            .service(game::edit_game)
            .service(game::delete_game)
            .service(game::view_scores)
            .service(admin::games_list)
            .service(admin::hide_user_from_scoreboards)
            .service(admin::reports)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
