use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;

use sqlx::Row;
use std::borrow::Borrow;

use crate::types::DbPool;

pub struct KeyPackage {
    pub id: i32,
    pub client_id: i32,
    pub key_package: Vec<u8>,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for KeyPackage {
    fn from(row: &PgRow) -> Self {
        KeyPackage {
            id: row.get("id"),
            client_id: row.get("client_id"),
            key_package: row.get("key_package"),
            created: row.get("created"),
        }
    }
}

impl KeyPackage {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO key_package (client_id, key_package) VALUES ($1,$2) RETURNING *;",
        )
        .bind(self.client_id)
        .bind(&self.key_package)
        .fetch_one(db)
        .await?
        .borrow()
        .into();

        Ok(())
    }
}
