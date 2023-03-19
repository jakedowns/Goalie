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

    fn add_scores(game_id: String, scores: Vec<NewScoreInput>) -> FieldResult<Vec<Score>> {
        let conn = establish_connection();
        let game: Game = game.filter(id.eq(&game_id)).first(&conn)?;
        let mut inserted_scores = vec![];
        for score_input in scores {
            let new_score = NewScore {
                game_id: game.id,
                player_id: score_input.player_id,
                score_value: score_input.score_value,
            };
            let score = diesel::insert_into(score)
                .values(&new_score)
                .get_result(&conn)?;
            inserted_scores.push(score);
        }
        Ok(inserted_scores)
    }
}

#[juniper::graphql_object]
impl Score {
    fn id(&self) -> &str {
        &self.id.to_string()
    }

    fn game_id(&self) -> &str {
        &self.game_id.to_string()
    }

    fn player_id(&self) -> &str {
        &self.player_id
    }

    fn score_value(&self) -> i32 {
        self.score_value
    }
}

#[derive(juniper::GraphQLInputObject)]
struct NewScoreInput {
    player_id: String,
    score_value: i32,
}
