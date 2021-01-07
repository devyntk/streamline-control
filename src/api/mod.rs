use sqlx::{Pool, Sqlite};
use warp::Filter;

pub fn api_filter(db: Pool<Sqlite>) -> impl Filter + Clone {
    warp::path!("api")
}
