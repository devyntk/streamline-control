use crate::server::get_pool;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use clap::Subcommand;
use rand_core::OsRng;
use sqlx::{Pool, Sqlite};

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// add a new user
    AddUser { username: String, password: String },
}

#[tokio::main]
pub async fn handle_command(command: Commands) -> anyhow::Result<()> {
    let pool = get_pool().await?;
    return match command {
        Commands::AddUser { username, password } => add_new_user(username, password, pool).await,
    };
}

async fn add_new_user(username: String, password: String, db: Pool<Sqlite>) -> anyhow::Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_ref(), &salt).unwrap();
    sqlx::query("INSERT INTO user (username, pw) VALUES ( ?, ?)")
        .bind(username)
        .bind(password_hash.to_string())
        .execute(&db)
        .await?;
    Ok(())
}
