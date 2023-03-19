use warp::{self, Filter};

use crate::handlers::score::{submit_score, get_scores};
use crate::models::score::ScoreInput;

pub fn score_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let score = warp::post()
        .and(warp::path!("score"))
        .and(warp::body::json())
        .and_then(submit_score);

    let scores = warp::get()
        .and(warp::path!("scores"))
        .and_then(get_scores);

    score.or(scores)
}
