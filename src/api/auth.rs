use crate::api::types::{LoggedUser, UserLogin};
use crate::api::AppError;
use crate::server::SharedState;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::debug_handler;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn_with_state, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{http, Extension, Json, Router};
use biscuit_auth::Biscuit;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Row, Sqlite};

pub fn auth_routes(state: SharedState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route(
            "/user",
            get(current_user_handler).route_layer(from_fn_with_state(state.clone(), auth)),
        )
        .with_state(state)
}

async fn auth<B>(
    State(state): State<SharedState>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header, state.pool).await {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str, db: Pool<Sqlite>) -> Option<LoggedUser> {
    Some(LoggedUser {
        id: 0,
        display_name: "".to_owned(),
        username: "".to_owned(),
        token: "".to_owned(),
    })
}

async fn login_handler(
    State(state): State<SharedState>,
    Json(login): Json<UserLogin>,
) -> Result<impl IntoResponse, AppError> {
    let user = sqlx::query("SELECT id, display_name, username, pw FROM user WHERE username = ?")
        .bind(login.username)
        .fetch_one(&state.pool)
        .await?;

    let pw = user.get("pw");
    let hash = PasswordHash::new(pw).expect("Cannot get DB Hash");

    let argon2 = Argon2::default();
    match argon2.verify_password(login.password.as_ref(), &hash) {
        Ok(_) => {
            let username = user.get("username");
            let builder = Biscuit::builder(&state.key);
            let token = builder.build()?.to_base64()?;

            return Ok((
                StatusCode::OK,
                Json(serde_json::json!(LoggedUser {
                    id: user.get("id"),
                    display_name: user.get("display_name"),
                    username,
                    token,
                })),
            ));
        }
        Err(err) => {
            log::warn!("{}", err);
            return Ok((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Incorrect password" })),
            ));
        }
    }
}

#[debug_handler]
async fn current_user_handler(
    Extension(current_user): Extension<LoggedUser>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, StatusCode> {
    return Ok(Json(current_user));
}
