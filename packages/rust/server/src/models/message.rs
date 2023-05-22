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
        let mut params = "($1, $2)".to_string();
        for i in (3..=client_ids.len()).step_by(2) {
            params.push_str(&format!(", (${}, ${})", i, i + 1));
        }

        let query_string = format!(
            "INSERT INTO recipient (client_id, message_id) VALUES {};",
            params
        );

        let mut query = sqlx::query(&query_string);
        for client_id in client_ids {
            query = query.bind(client_id);
            query = query.bind(self.id);
        }

        query.fetch_all(db).await?;
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

    pub async fn delete_ids(
        message_ids: &Vec<i32>,
        client_id: i32,
        db: &DbPool,
    ) -> Result<(), sqlx::Error> {
        let mut recipients_id: Vec<i32> = Vec::new();
        recipients_id = sqlx::query("SELECT id FROM recipient WHERE client_id = $1;")
            .bind(client_id)
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.get::<i32, _>("id"))
            .collect();

        let mut params = "$1".to_string();
        for i in 2..=recipients_id.len() {
            params.push_str(&format!(", ${}", i));
        }

        let query_string = format!("DELETE FROM recipient WHERE id IN ({});", params);

        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
        for id in recipients_id {
            query = query.bind(id);
        }
        query.fetch_all(db).await?;

        let mut params = "$1".to_string();
        for i in 2..=message_ids.len() {
            params.push_str(&format!(", ${}", i));
        }

        let query_string = format!("DELETE FROM message WHERE id IN ({});", params);

        let mut query = sqlx::query::<sqlx::Postgres>(&query_string);
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
