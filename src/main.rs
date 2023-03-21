use dotenv::dotenv;
// use std::env;
use actix_web::{HttpServer, App};

mod errors;
mod db;
mod handlers;
mod models;
mod schema;

#[cfg(test)]
mod tests;

use crate::handlers::game;
use crate::handlers::game_move;
use crate::handlers::user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/", "static").index_file("index.html"))
            .service(game::game_scope())
            .service(game_move::submit_move_scope())
            .service(user::user_scope())
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}

