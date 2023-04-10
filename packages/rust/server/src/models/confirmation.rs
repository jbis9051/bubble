use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use std::borrow::Borrow;
use uuid::Uuid;

use crate::types::DbPool;

pub struct Confirmation {
    pub id: i32,
    pub user_id: i32,
    pub token: Uuid,
    pub email: String,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Confirmation {
    fn from(row: &PgRow) -> Self {
        Confirmation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            email: row.get("email"),
            created: row.get("created"),
        }
    }
}

impl Confirmation {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO confirmation (user_id, token, email)
                             VALUES ($1, $2, $3) RETURNING *;",
        )
        .bind(self.user_id)
        .bind(self.token)
        .bind(&self.email)
        .fetch_one(db)
        .await?
        .borrow()
        .into();
        Ok(())
    }

    pub async fn filter_user_id(
        db: &DbPool,
        user_id: i32,
    ) -> Result<Vec<Confirmation>, sqlx::Error> {
        Ok(
            sqlx::query("SELECT * FROM confirmation WHERE user_id = $1;")
                .bind(user_id)
                .fetch_all(db)
                .await?
                .iter()
                .map(|row| row.into())
                .collect(),
        )
    }

    pub async fn from_token(db: &DbPool, token: &Uuid) -> Result<Confirmation, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM confirmation WHERE token = $1;")
            .bind(token)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn delete_all(db: &DbPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM confirmation WHERE user_id = $1")
            .bind(user_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM confirmation WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
}
