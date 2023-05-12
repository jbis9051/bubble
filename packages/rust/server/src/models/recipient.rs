use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;

use sqlx::Row;
use std::borrow::Borrow;

use crate::types::DbPool;

pub struct Recipient {
    pub id: i32,
    pub client_id: i32,
    pub message_id: i32,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Recipient {
    fn from(row: &PgRow) -> Self {
        Recipient {
            id: row.get("id"),
            client_id: row.get("client_id"),
            message_id: row.get("message_id"),
            created: row.get("created"),
        }
    }
}

impl Recipient {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO recipient (client_id, message_id) VALUES ($1,$2) RETURNING *;",
        )
        .bind(self.client_id)
        .bind(self.message_id)
        .fetch_one(db)
        .await?
        .borrow()
        .into();

        Ok(())
    }

    pub async fn filter_client_id(
        db: &DbPool,
        client_id: i32,
    ) -> Result<Vec<Recipient>, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM recipient WHERE client_id = $1;")
            .bind(client_id)
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.into())
            .collect())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM recipient WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;

        Ok(())
    }
}
