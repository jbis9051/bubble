use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;

use sqlx::Row;
use std::borrow::Borrow;

use crate::types::DbPool;

pub struct Message {
    pub id: i32,
    pub message: Vec<u8>,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Message {
    fn from(row: &PgRow) -> Self {
        Message {
            id: row.get("id"),
            message: row.get("message"),
            created: row.get("created"),
        }
    }
}

impl Message {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query("INSERT INTO message (message) VALUES ($1) RETURNING *;")
            .bind(&self.message)
            .fetch_one(db)
            .await?
            .borrow()
            .into();

        Ok(())
    }


    pub async fn from_id(db: &DbPool, id: i32) -> Result<Message, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM message WHERE id = $1;")
            .bind(id)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM message WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;

        Ok(())
    }
}
