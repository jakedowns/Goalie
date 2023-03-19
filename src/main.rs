use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpServer};
mod auth;
mod game;
mod admin;
mod score;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    HttpServer::new(|| {
        App::new()
            // Add a basic home route that serves static/index.html
            .service(actix_files::Files::new("/", "static").index_file("index.html"))
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
            // Add score routes to the server
            .service(score::submit_score)
            .service(score::get_score)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
