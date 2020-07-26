use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LoggedUser {
    id: usize,
    email: String,
    username: String,
    token: String,
}