use warp::{Filter, get, Reply};
use std::convert::Infallible;
use sqlx::{Pool, Sqlite};
use crate::api::with_db;

pub fn auth_filters(db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    check_token(db.clone())
}

fn check_token(db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("auth" / "token" / String)
        .and(with_db(db))
        .and(get())
        .and_then(check_token_handler)
}

async fn check_token_handler(token: String, db: Pool<Sqlite>) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&token))
}
