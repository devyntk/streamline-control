use serde::{Deserialize, Serialize};

pub const API_PREFIX: &str = "api";

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoggedUser {
    id: usize,
    display_name: String,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Status {
    pub version: String
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
