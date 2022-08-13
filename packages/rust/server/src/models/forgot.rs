use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;
use std::borrow::Borrow;

use crate::types::DbPool;

pub struct Forgot {
    pub id: i32,
    pub user_id: i32,
    pub forgot_id: Uuid,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Forgot {
    fn from(row: &PgRow) -> Self {
        Forgot {
            id: row.get("id"),
            user_id: row.get("user_id"),
            forgot_id: row.get("forgot_id"),
            created: row.get("created"),
        }
    }
}

impl Forgot {
    pub async fn from_uuid(db: &DbPool, uuid: &str) -> Result<Forgot, sqlx::Error> {
        let uuid = Uuid::parse_str(uuid).unwrap();

        Ok(
            sqlx::query("SELECT * FROM forgot_password WHERE forgot_id = $1;")
                .bind(uuid)
                .fetch_one(db)
                .await?
                .borrow()
                .into(),
        )
    }

    pub async fn create(&self, db: &DbPool) -> Result<Forgot, sqlx::Error> {
        Ok(sqlx::query(
            "INSERT INTO forgot_password (user_id, forgot_id) VALUES ($1, $2) RETURNING *;",
        )
        .bind(&self.user_id)
        .bind(&self.forgot_id)
        .fetch_one(db)
        .await?
        .borrow()
        .into())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM forgot_password WHERE forgot_id = $1")
            .bind(&self.forgot_id)
            .execute(db)
            .await?;

        Ok(())
    }
}
