#[allow(dead_code)]

use serde::{Deserialize, Serialize};

pub const API_PREFIX: &str = "api";
pub const AUTH_PREFIX: &str = "auth";
pub const TOKEN_PREFIX: &str = "token";
pub const LOGIN_PREFIX: &str = "login";

#[derive(Serialize, Deserialize)]
pub struct LoggedUser {
    pub id: usize,
    pub display_name: String,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub enum LoginResult {
    Ok(LoggedUser),
    IncorrectAuth,
    AuthFailed
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub version: String
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
