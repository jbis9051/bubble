use crate::types::DbPool;
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

pub type GlobalKv = Kv;
pub type AccountKv = Kv;

pub type Key = String;
pub type Value = String;

pub struct Kv {
    pub id: i32,
    pub key: Key,
    pub value: Value,
    pub created: NaiveDateTime,
}

impl From<&SqliteRow> for Kv {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            key: row.get("key"),
            value: row.get("value"),
            created: row.get("created"),
        }
    }
}

impl Kv {
    pub async fn get(db: &DbPool, key: &str) -> Result<Option<Value>, sqlx::Error> {
        sqlx::query("SELECT value FROM kv WHERE key = $1;")
            .bind(key)
            .fetch_optional(db)
            .await
            .map(|row| row.map(|row| row.get("value")))
    }

    pub async fn set(db: &DbPool, key: &str, value: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM kv WHERE key = $1; INSERT INTO kv (key, value) VALUES ($1,$2);")
            .bind(key)
            .bind(value)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete(db: &DbPool, key: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM kv WHERE key = $1;")
            .bind(key)
            .execute(db)
            .await?;
        Ok(())
    }
}
