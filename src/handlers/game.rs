use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{NewGame, Game, Score};
use crate::schema::games::dsl::*;
use crate::schema::scores::dsl::*;
use crate::db::establish_connection;
use diesel::QueryResult;

fn establish_connection_and_execute<T>(operation: T) -> QueryResult<usize>
where
    T: FnOnce(&PgConnection) -> QueryResult<usize>,
{
    let connection = establish_connection();
    operation(&connection)
}

pub async fn create_game(new_game: web::Json<NewGame>) -> impl Responder {
    let result = establish_connection_and_execute(|connection| {
        diesel::insert_into(games)
            .values(&new_game.into_inner())
            .execute(connection)
    });

    match result {
        Ok(_) => HttpResponse::Ok().body("Game created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating game"),
    }
}

pub async fn edit_game(game_id: web::Path<i32>, updated_game: web::Json<Game>) -> impl Responder {
    let result = establish_connection_and_execute(|connection| {
        diesel::update(games.find(game_id.into_inner()))
            .set(&updated_game.into_inner())
            .execute(connection)
    });

    match result {
        Ok(_) => HttpResponse::Ok().body("Game edited"),
        Err(_) => HttpResponse::InternalServerError().body("Error editing game"),
    }
}

pub async fn delete_game(game_id: web::Path<i32>) -> impl Responder {
    let result = establish_connection_and_execute(|connection| {
        diesel::delete(games.find(game_id.into_inner()))
            .execute(connection)
    });

    match result {
        Ok(_) => HttpResponse::Ok().body("Game deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting game"),
    }
}

pub async fn view_scores(game_id: web::Path<i32>) -> impl Responder {
    let connection = establish_connection();
    let result = scores
        .filter(game_id.eq(game_id.into_inner()))
        .load::<Score>(&connection);

    match result {
        Ok(scores) => HttpResponse::Ok().json(scores),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching scores"),
    }
}
