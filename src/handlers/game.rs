use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::models::{NewGameInput, NewGame, Game};
use crate::schema::games::dsl::*;
//use crate::schema::scores::dsl::*;
use crate::db::{establish_connection, establish_connection_and_execute};
//use diesel::QueryResult;
//use diesel::result::Error;

pub async fn create_game(new_game_input: web::Json<NewGameInput>) -> impl Responder {
    // Create a NewGame with a hard-coded creator_id
    let new_game = NewGame {
        name: new_game_input.name.clone(),
        creator_id: 1, // Replace with the actual creator_id
    };

    let result = establish_connection_and_execute(|connection| {
        diesel::insert_into(games)
            .values(&new_game)
            .execute(connection)
    });

    // get the id of the game that was was most recently created
    let game_id = establish_connection_and_execute(|connection| {
        games
            .select(diesel::dsl::max(id))
            .first::<Option<i32>>(connection)
    });

    match result {
        Ok(_) => HttpResponse::Ok().body("Game created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating game"),
    }
}

// pub async fn edit_game(game_id: web::Path<i32>, updated_game: web::Json<Game>) -> impl Responder {
//     let result = establish_connection_and_execute(|connection| {
//         diesel::update(games.find(game_id.into_inner()))
//             .set(&updated_game.into_inner())
//             .execute(connection)
//     });

//     match result {
//         Ok(_) => HttpResponse::Ok().body("Game edited"),
//         Err(_) => HttpResponse::InternalServerError().body("Error editing game"),
//     }
// }

// pub async fn delete_game(game_id: web::Path<i32>) -> impl Responder {
//     let result = establish_connection_and_execute(|connection| {
//         diesel::delete(games.find(game_id.into_inner()))
//             .execute(connection)
//     });

//     match result {
//         Ok(_) => HttpResponse::Ok().body("Game deleted"),
//         Err(_) => HttpResponse::InternalServerError().body("Error deleting game"),
//     }
// }

// pub async fn view_scores(game_id: web::Path<i32>) -> impl Responder {
//     let connection = establish_connection();
//     let result = scores
//         .filter(game_id.eq(game_id.into_inner()))
//         .load::<Score>(&connection);

//     match result {
//         Ok(scores) => HttpResponse::Ok().json(scores),
//         Err(_) => HttpResponse::InternalServerError().body("Error fetching scores"),
//     }
// }

pub async fn list_games() -> impl Responder {
    let mut connection = establish_connection();
    let result = games.load::<Game>(&mut connection);

    match result {
        Ok(game_list) => HttpResponse::Ok().json(game_list),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching games"),
    }
}



// Export game_scope route definition for /game/add mapped to create_game
pub fn game_scope() -> actix_web::Scope {
    web::scope("/game")
        .route("/list", web::get().to(list_games))
        .route("/add", web::post().to(create_game))
        // .route("/edit/{id}", web::post().to(edit_game))
        // .route("/delete/{id}", web::post().to(delete_game))
        // .route("/{id}/scores/", web::get().to(view_scores))
}

