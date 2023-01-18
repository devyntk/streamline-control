use serde::{de::DeserializeOwned, Serialize};
use sqlx::{Pool, Row, Sqlite};

pub struct Config<T: DeserializeOwned + Serialize + Clone> {
    key: &'static str,
    value: T,
    db: Pool<Sqlite>,
}

impl<T: DeserializeOwned + Serialize + Clone> Config<T> {
    pub async fn new(key: &'static str, default: T, db: Pool<Sqlite>) -> anyhow::Result<Self> {
        let default_enc = serde_json::to_string(&default)?;
        let value_raw: String = sqlx::query(
            "INSERT OR IGNORE INTO \"config\"(key, value) VALUES(?, ?);
            SELECT value FROM \"config\" WHERE key = ?",
        )
        .bind(key)
        .bind(default_enc)
        .bind(key)
        .fetch_one(&db.clone())
        .await?
        .get("value");
        let value = serde_json::from_str(&*value_raw)?;
        Ok(Config { key, value, db })
    }

    pub fn get(&self) -> T {
        return self.value.clone();
    }

    pub async fn set(&mut self, value: T) -> anyhow::Result<T> {
        let value_enc = serde_json::to_string(&value.clone())?;
        self.value = value.clone();
        tokio::spawn(persist(self.key.clone(), value_enc, self.db.clone()));
        return Ok(value.clone());
    }
}
async fn persist(key: &'static str, value: String, db: Pool<Sqlite>) {
    sqlx::query("UPDATE \"config\" SET value = ? WHERE key = ?;")
        .bind(value)
        .bind(key)
        .execute(&db.clone())
        .await
        .expect("Unable to update DB value");
}
