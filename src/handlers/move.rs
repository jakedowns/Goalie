//use diesel::prelude::*;
use std::sync::Mutex;
use diesel::RunQueryDsl;
use actix_web::{web, HttpResponse, Result, Scope};
use serde::Deserialize;
use log;
use actix_web::error::Error as reject;

use crate::db::DbConn;
use crate::models::{NewMove, Move, MoveInput};
use crate::schema::moves;
use crate::schema::moves::dsl::*;

#[derive(Deserialize)]
pub struct SubmitMoveParams {
    input: MoveInput,
}

pub async fn submit_move(params: web::Json<SubmitMoveParams>, db: web::Data<DbConn>) -> Result<HttpResponse> {    // Debug log what we are attempting to insert
    log::debug!("Attempting to insert: {:?}", params.input);

    let new_move = NewMove {
        player_id: params.input.player_id,
        round_id: params.input.round_id,
        game_id: params.input.game_id,
        points_id: params.input.points_id,
        times_id: params.input.times_id,
        move_type: params.input.move_type,
    };

    let _move = diesel::insert_into(moves::table)
        .values(&new_move)
        .get_result::<Move>()
        .map_err(|e| reject::from)?;

    Ok(HttpResponse::Ok().json(_move))
}



pub fn submit_move_scope() -> Scope {
    web::scope("/move")
        .route("/submit", web::post().to(submit_move))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::db::DbConn;
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::PgConnection;
    use serde_json::json;

    #[actix_rt::test]
    async fn test_submit_move() {
        let (app, db) = setup_for_test().await;


        // Create a test request with sample data
        let move_input = MoveInput {
            player_id: 1,
            game_id: 1,
            move_type: 1
        };
        let params = SubmitMoveParams { input: move_input };
        let req = test::TestRequest::post()
            .uri("/move/submit")
            .set_json(&params)
            .to_request();

        // Send the request and check the response
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        // Check if the response contains the expected data
        let result: Move = test::read_body_json(resp).await;
        assert_eq!(result.player_id, params.input.player_id);
        assert_eq!(result.game_id, params.input.game_id);
        assert_eq!(result.move_type, params.input.move_type);
    }
}