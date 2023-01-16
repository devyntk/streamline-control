use crate::api::state::SharedState;
use crate::api::types::{LoggedUser, UserLogin};
use crate::api::AppError;
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
use sqlx::{Error, Row};

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

    if let Ok(current_user) = Biscuit::from_base64(auth_header, |_| state.key.public()) {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn login_handler(
    State(state): State<SharedState>,
    Json(login): Json<UserLogin>,
) -> Result<impl IntoResponse, AppError> {
    let user_res =
        sqlx::query("SELECT id, display_name, username, pw FROM user WHERE username = ?")
            .bind(login.username)
            .fetch_one(&state.pool)
            .await;

    let user = match user_res {
        Ok(user) => user,
        Err(Error::RowNotFound) => {
            return Ok((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Incorrect username" })),
            ));
        }
        Err(err) => {
            log::error!("Error while fetching password: {:?}", err);
            return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Internal error" })),
            ));
        }
    };

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

async fn current_user_handler(
    Extension(current_user): Extension<Biscuit>,
) -> Result<impl IntoResponse, StatusCode> {
    return Ok(current_user.print());
}
