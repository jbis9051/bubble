use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use uuid::Uuid;

use crate::types::DbPool;

pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: Uuid,
    pub created: NaiveDateTime,
}

impl Session {
    fn from_row(row: &PgRow) -> Session {
        Session {
            id: row.get("id"),
            user_id: row.get("user_id"),
            token: row.get("token"),
            created: row.get("created"),
        }
    }

    pub async fn create(&self, db: &DbPool) -> Result<Session, sqlx::Error> {
        let row = sqlx::query("INSERT INTO session (user_id, token) VALUES ($1, $2) RETURNING *;")
            .bind(&self.user_id)
            .bind(&self.token)
            .fetch_one(db)
            .await?;
        let session = Session::from_row(&row);
        Ok(session)
    }

    pub async fn filter_user_id(db: &DbPool, user_id: i32) -> Result<Vec<Session>, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM session WHERE user_id = $1;")
            .bind(user_id)
            .fetch_all(db)
            .await?
            .iter()
            .map(Self::from_row)
            .collect())
    }

    pub async fn from_token(db: &DbPool, token: Uuid) -> Result<Session, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM session WHERE token = $1;")
            .bind(token)
            .fetch_one(db)
            .await?;
        let session = Session::from_row(&row);
        Ok(session)
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM session WHERE token = $1;")
            .bind(&self.token)
            .execute(db)
            .await?;
        Ok(())
    }
}
