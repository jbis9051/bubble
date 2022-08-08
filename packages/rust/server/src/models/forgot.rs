use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;

use crate::types::DbPool;

pub struct Forgot {
    pub id: i32,
    pub user_id: i32,
    pub forgot_id: Uuid,
    pub created: NaiveDateTime,
}

impl Forgot {
    fn from_row(row: &PgRow) -> Forgot {
        Forgot {
            id: row.get("id"),
            user_id: row.get("user_id"),
            forgot_id: row.get("forgot_id"),
            created: row.get("created"),
        }
    }

    pub async fn from_uuid(db: &DbPool, uuid: &str) -> Result<Forgot, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM forgot_password WHERE forgot_id = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?;

        Ok(Forgot {
            id: row.get("id"),
            user_id: row.get("user_id"),
            forgot_id: row.get("forgot_id"),
            created: row.get("created"),
        })
    }

    pub async fn create(&self, db: &DbPool) -> Result<Forgot, sqlx::Error> {
        let row = sqlx::query(
            "INSERT INTO forgot_password (user_id, forgot_id) VALUES ($1, $2) RETURNING *;",
        )
        .bind(&self.user_id)
        .bind(&self.forgot_id)
        .fetch_one(db)
        .await?;

        let forgot = Forgot::from_row(&row);
        Ok(forgot)
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM forgot_password WHERE forgot_id = $1")
            .bind(&self.forgot_id)
            .execute(db)
            .await?;

        Ok(())
    }
}
