use crate::models::score::{Score, NewScore};
use crate::models::game::Game;
use crate::schema::score::dsl::*;
use crate::schema::game::dsl::*;
use diesel::prelude::*;
use juniper::{FieldResult};

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn scores() -> FieldResult<Vec<Score>> {
        let conn = establish_connection();
        let scores = score.load::<Score>(&conn)?;
        Ok(scores)
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn create_score(game_id: String, player_id: String, score_value: i32) -> FieldResult<Score> {
        let conn = establish_connection();
        let game: Game = game.filter(id.eq(&game_id)).first(&conn)?;
        let new_score = NewScore {
            game_id: game.id,
            player_id: player_id,
            score_value: score_value,
        };
        let score = diesel::insert_into(score)
            .values(&new_score)
            .get_result(&conn)?;
        Ok(score)
    }
}
