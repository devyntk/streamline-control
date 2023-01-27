use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoggedUser {
    pub id: i64,
    pub username: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
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
