use diesel::prelude::*;
use warp::{http::StatusCode, reject, Rejection, Reply};

use crate::db::DbConn;
use crate::models::score::{Score, ScoreInput};

pub async fn submit_score(input: ScoreInput, db: DbConn) -> Result<impl Reply, Rejection> {
    let score = diesel::insert_into(scores::table)
        .values(&input)
        .get_result::<Score>(&*db)
        .map_err(|e| reject::custom(e))?;

    Ok(warp::reply::json(&score))
}

pub async fn get_scores(db: DbConn) -> Result<impl Reply, Rejection> {
    let scores = scores::table.load::<Score>(&*db).map_err(|e| reject::custom(e))?;

    Ok(warp::reply::json(&scores))
}
