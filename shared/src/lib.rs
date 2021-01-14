use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoggedUser {
    id: usize,
    email: String,
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Status {
    pub version: String
}
