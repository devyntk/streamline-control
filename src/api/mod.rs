use sqlx::{Pool, Sqlite};
use warp::{Filter, get, Reply};
use shared::Status;
use std::convert::Infallible;
use warp::reply::json;
use clap::crate_version;

pub fn api_filter(_db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    status()
}

fn status() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "status")
        .and(get())
        .and_then(get_status)
}

async fn get_status() -> Result<impl Reply, Infallible>{
    let status = Status {
        version: crate_version!().into()
    };
    Ok(json(&status))
}
