mod auth;

use sqlx::{Pool, Sqlite};
use warp::{Filter, get, Reply, path, Rejection};
use shared::{Status, API_PREFIX, ErrorMessage};
use std::convert::Infallible;
use warp::reply::json;
use clap::crate_version;
use crate::api::auth::auth_filters;
use warp::http::StatusCode;
use std::error::Error;


fn with_db(db: Pool<Sqlite>) -> impl Filter<Extract = (Pool<Sqlite>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn api_filter(db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    path(API_PREFIX).and(
        status()
        .or(auth_filters(db.clone()))
    )

}

fn status() -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    warp::path!("status")
        .and(get())
        .and_then(get_status)
}

async fn get_status() -> Result<impl Reply, Infallible>{
    let status = Status {
        version: crate_version!().into()
    };
    Ok(json(&status))
}

pub async fn handle_api_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        // This error happens if the body could not be deserialized correctly
        // We can use the cause to analyze the error and customize the error message
        message = match e.source() {
            Some(cause) => {
                if cause.to_string().contains("denom") {
                    "FIELD_ERROR: denom"
                } else {
                    "BAD_REQUEST"
                }
            }
            None => "BAD_REQUEST",
        };
        code = StatusCode::BAD_REQUEST;
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // We can handle a specific error, here METHOD_NOT_ALLOWED,
        // and render it however we want
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
