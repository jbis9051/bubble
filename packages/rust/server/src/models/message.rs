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

    pub async fn from_ids(db: &DbPool, ids: &[i32]) -> Result<Vec<Message>, sqlx::Error> {
        let mut params = "$1".to_string();
        for i in 2..=ids.len() {
            params.push_str(&format!(", ${}", i));
        }

        let query_string = format!("SELECT * FROM message WHERE id IN ({});", params);

        let mut query = sqlx::query(&query_string);
        for id in ids {
            query = query.bind(id);
        }

        Ok(query
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

    pub async fn delete_ids(message_ids: &Vec<i32>, db: &DbPool) -> Result<(), sqlx::Error> {
        let mut params = "$1".to_string();
        for i in 2..=message_ids.len() {
            params.push_str(&format!(", ${}", i));
        }

        let query_string = format!("DELETE FROM message WHERE id IN ({});", params);

        let mut query = sqlx::query(&query_string);
        for id in message_ids {
            query = query.bind(id);
        }

        query.fetch_all(db).await?;
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
