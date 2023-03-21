use diesel::prelude::*;
// use std::sync::Mutex;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use actix_web::{App, web, HttpResponse, Result, Scope};
use serde::{Serialize,Deserialize};
use log;
use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use r2d2_sqlite::SqliteConnectionManager;

use crate::db::establish_connection;

//use crate::errors::{CustomError};
use crate::db::DbConn;

use crate::models::{NewMove, Move, MoveInput};
use crate::schema::moves;
use crate::schema::moves::dsl::*;

#[derive(Deserialize, Serialize)]
pub struct SubmitMoveParams {
    input: MoveInput,
}

pub async fn submit_move(params: web::Json<SubmitMoveParams>, db: web::Data<DbConn>) -> Result<HttpResponse> {

    // Create a new move from the input
    let new_move = NewMove {
        player_id: params.input.player_id,
        game_id: params.input.game_id,
        round_id: params.input.round_id,
        points_id: params.input.points_id,
        times_id: params.input.times_id,
        move_type: params.input.move_type,
    };

    // Get a connection from the connection pool
    // let conn: PooledConnection<SqliteConnectionManager> = db.get_ref().get().map_err(|e| {
    //     actix_web::error::ErrorInternalServerError(format!("Error getting connection: {}", e))
    // })?;

    let mut conn = establish_connection();


    // Insert the new move
    diesel::insert_into(moves::table)
        .values(&new_move)
        .execute(&mut conn)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Error inserting move: {}", e))
        })?;

    // Fetch the last inserted move
    let _move = moves
        .order(moves::id.desc())
        .first::<Move>(&mut conn)
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Error fetching last inserted move: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(_move))
}





pub fn submit_move_scope() -> Scope {
    web::scope("/move")
        .route("/submit", web::post().to(submit_move))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test};
    use actix_web::web::Data;
    use diesel::r2d2::{ConnectionManager, Pool};

    #[actix_rt::test]
    async fn test_submit_move() -> Result<(), actix_web::Error> {
        // Create a test server
        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(
                    Pool::builder()
                        .build(ConnectionManager::<SqliteConnection>::new("sqlite::memory:"))
                        .unwrap(),
                ))
                .service(submit_move_scope()),
        ).await;


        // Create a test request with sample data
        let move_input = MoveInput {
            player_id: 1,
            game_id: 1,
            round_id: Some(1),
            points_id: Some(1),
            times_id: Some(1),
            move_type: 1,
        };
        let params = SubmitMoveParams { input: move_input };
        let req = test::TestRequest::post()
            .uri("/move/submit")
            .set_json(&params)
            .to_request();

        // Send the request and check the response
        let resp = test::call_service(&mut app, req).await;
        let resp_status = resp.status();
        // Store the body of the response in a variable
        println!("Response status: {:?}", resp_status);
        assert!(resp_status.is_success());
        
        let resp_body = test::read_body(resp).await;
        println!("Response body: {:?}", resp_body);
        
        // Deserialize the resp_body into a Move struct
        let result: Move = serde_json::from_slice(&resp_body).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Error deserializing response body: {}", e))
        })?;


        // Check if the response contains the expected data
        assert_eq!(result.player_id, params.input.player_id);
        assert_eq!(result.game_id, params.input.game_id);
        assert_eq!(result.move_type, params.input.move_type);

        Ok(())
    }
}