use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;
use std::borrow::Borrow;

use crate::types::DbPool;

pub struct Forgot {
    pub id: i32,
    pub user_id: i32,
    pub token: Uuid,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Forgot {
    fn from(row: &PgRow) -> Self {
        Forgot {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            created: row.get("created"),
        }
    }
}

impl Forgot {
    pub async fn from_token(db: &DbPool, uuid: &Uuid) -> Result<Forgot, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM forgot WHERE token = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn filter_user_id(db: &DbPool, user_id: i32) -> Result<Vec<Forgot>, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM forgot WHERE user_id = $1;")
            .bind(user_id)
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.into())
            .collect())
    }

    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query("INSERT INTO forgot (user_id, token) VALUES ($1, $2) RETURNING *;")
            .bind(self.user_id)
            .bind(self.token)
            .fetch_one(db)
            .await?
            .borrow()
            .into();

        Ok(())
    }

    pub async fn delete_all(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM forgot WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;

        Ok(())
    }
}
