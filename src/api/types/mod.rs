#[allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggedUser {
    pub id: usize,
    pub display_name: String,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum LoginResult {
    Ok(LoggedUser),
    IncorrectAuth,
    AuthFailed,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub version: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
