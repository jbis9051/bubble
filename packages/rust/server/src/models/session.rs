use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use std::borrow::Borrow;

use uuid::Uuid;

use crate::types::DbPool;

pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: Uuid,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Session {
    fn from(row: &PgRow) -> Self {
        Session {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            created: row.get("created"),
        }
    }
}

impl Session {
    pub async fn create(&self, db: &DbPool) -> Result<Session, sqlx::Error> {
        Ok(
            sqlx::query("INSERT INTO session (user_id, token) VALUES ($1, $2) RETURNING *;")
                .bind(&self.user_id)
                .bind(&self.token)
                .fetch_one(db)
                .await?
                .borrow()
                .into(),
        )
    }

    pub async fn filter_user_id(db: &DbPool, user_id: i32) -> Result<Vec<Session>, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM session WHERE user_id = $1;")
            .bind(user_id)
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.into())
            .collect())
    }

    pub async fn from_token(db: &DbPool, token: Uuid) -> Result<Session, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM session WHERE token = $1;")
            .bind(token)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM session WHERE token = $1;")
            .bind(&self.token)
            .execute(db)
            .await?;
        Ok(())
    }
}
