use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;
use std::borrow::Borrow;

use crate::types::DbPool;

pub struct Client {
    pub id: i32,
    pub user_id: i32,
    pub uuid: Uuid,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Client {
    fn from(row: &PgRow) -> Self {
        Client {
            id: row.get("id"),
            user_id: row.get("user_id"),
            uuid: row.get("uuid"),
            created: row.get("created"),
        }
    }
}

impl Client {
    pub async fn from_uuid(db: &DbPool, uuid: &Uuid) -> Result<Client, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM client WHERE uuid = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn filter_user_id(db: &DbPool, user_id: i32) -> Result<Vec<Client>, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM client WHERE user_id = $1;")
            .bind(user_id)
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.into())
            .collect())
    }

    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query("INSERT INTO client (user_id, uuid) VALUES ($1, $2) RETURNING *;")
            .bind(self.user_id)
            .bind(self.uuid)
            .fetch_one(db)
            .await?
            .borrow()
            .into();

        Ok(())
    }

    pub async fn delete_all(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM client WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;

        Ok(())
    }
}
