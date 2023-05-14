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

    pub async fn create_all(
        db: &DbPool,
        client_ids: Vec<i32>,
        message_id: i32,
    ) -> Result<(), sqlx::Error> {
        let mut params = format!("($1, $2)");
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
            query = query.bind(message_id);
        }

        query.fetch_all(db).await?;

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
