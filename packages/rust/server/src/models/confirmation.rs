use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use std::borrow::Borrow;
use uuid::Uuid;

use crate::types::DbPool;

pub struct Confirmation {
    pub id: i32,
    pub user_id: i32,
    pub link_id: Uuid,
    pub email: String,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Confirmation {
    fn from(row: &PgRow) -> Self {
        Confirmation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            link_id: row.get("link_id"),
            email: row.get("email"),
            created: row.get("created"),
        }
    }
}

impl Confirmation {
    pub async fn create(&self, db: &DbPool) -> Result<Confirmation, sqlx::Error> {
        Ok(sqlx::query(
            "INSERT INTO confirmation (user_id, link_id, email)
                             VALUES ($1, $2, $3) RETURNING *;",
        )
        .bind(&self.user_id)
        .bind(&self.link_id)
        .bind(&self.email)
        .fetch_one(db)
        .await?
        .borrow()
        .into())
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

    pub async fn from_link_id(db: &DbPool, link_id: Uuid) -> Result<Confirmation, sqlx::Error> {
        Ok(
            sqlx::query("SELECT * FROM confirmation WHERE link_id = $1;")
                .bind(link_id)
                .fetch_one(db)
                .await?
                .borrow()
                .into(),
        )
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM confirmation WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
}
