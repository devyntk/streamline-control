use warp::{Filter, get, Reply};
use std::convert::Infallible;
use sqlx::{Pool, Sqlite, Row};
use crate::api::with_db;
use shared::{AUTH_PREFIX, TOKEN_PREFIX, LOGIN_PREFIX, UserLogin, LoginResult, LoggedUser};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use rand_core::OsRng;

pub fn auth_filters(db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    check_token(db.clone())
        .or(login(db.clone()))
}

fn check_token(db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(AUTH_PREFIX)
        .and(warp::path(TOKEN_PREFIX))
        .and(warp::path!(String))
        .and(with_db(db))
        .and(get())
        .and_then(check_token_handler)
}

async fn check_token_handler(token: String, db: Pool<Sqlite>) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&token))
}

fn login(db: Pool<Sqlite>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path(AUTH_PREFIX)
        .and(warp::path(LOGIN_PREFIX))
        .and(warp::body::json())
        .and(with_db(db))
        .and(get())
        .and_then(login_handler)
}

async fn login_handler(login: UserLogin, db: Pool<Sqlite>) -> Result<impl Reply, Infallible> {
    let user = sqlx::query("SELECT id, display_name, username, pw FROM user WHERE username = ?")
        .bind(login.username)
        .fetch_one(&db)
        .await;

    match user {
        Ok(row) => {
            let pw = row.get("pw");
            let hash = PasswordHash::new(pw).expect("Cannot get DB Hash");

            let argon2 = Argon2::default();
            match argon2.verify_password(login.password.as_ref(), &hash) {
                Ok(_) => {
                    // PW okay, get token
                    Ok(warp::reply::json(&LoginResult::Ok(LoggedUser{
                        id: 0,
                        display_name: row.get("display_name"),
                        username: row.get("username"),
                        token: "".to_string()
                    })))
                }
                Err(_) => {
                    Ok(warp::reply::json(&LoginResult::IncorrectAuth))
                }
            }
        }
        Err(_) => {
            Ok(warp::reply::json(&LoginResult::IncorrectAuth))
        }
    }
}

#[derive(Debug)]
struct AuthFailed {}

impl warp::reject::Reject for AuthFailed {}