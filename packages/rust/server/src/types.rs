use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;
