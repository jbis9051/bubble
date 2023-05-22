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
    pub async fn create(&mut self, db: &DbPool, client_ids: &Vec<i32>) -> Result<(), sqlx::Error> {
        *self = sqlx::query("INSERT INTO message (message) VALUES ($1) RETURNING *;")
            .bind(&self.message)
            .fetch_one(db)
            .await?
            .borrow()
            .into();

        let v = vec![self.id; client_ids.len()];

        sqlx::query("INSERT INTO recipient (client_id, message_id) SELECT * FROM UNNEST($1::int8[], $2::int8[]);")
            .bind(&client_ids[..])
            .bind(&v[..])
            .execute(db)
            .await?;
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

    pub async fn from_ids(db: &DbPool, ids: &[i32]) -> Result<Vec<Message>, sqlx::Error> {
        // a bug of the parameter typechecking code requires all array parameters to be slices
        Ok(sqlx::query("SELECT * FROM message WHERE id = ANY($1);")
            .bind(ids)
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.into())
            .collect())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM message WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn delete_ids(
        message_ids: &Vec<i32>,
        client_id: i32,
        db: &DbPool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM recipient WHERE client_id = $1;")
            .bind(client_id)
            .execute(db)
            .await?;

        sqlx::query("DELETE FROM message WHERE id = ANY($1);")
            .bind(message_ids)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn from_client_id(db: &DbPool, client_id: i32) -> Result<Vec<Message>, sqlx::Error> {
        Ok(sqlx::query(
            "SELECT *
        FROM message
        INNER JOIN recipient ON message.id = recipient.message_id
        WHERE recipient.client_id = $1;",
        )
        .bind(client_id)
        .fetch_all(db)
        .await?
        .iter()
        .map(|row| row.into())
        .collect())
    }
}
