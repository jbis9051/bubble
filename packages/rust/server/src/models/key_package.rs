use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;


use sqlx::Row;
use std::borrow::Borrow;


use crate::types::DbPool;

pub struct KeyPackage {
    pub id: i32,
    pub client_id: i32,
    pub key_package: Vec<u8>,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for KeyPackage {
    fn from(row: &PgRow) -> Self {
        KeyPackage {
            id: row.get("id"),
            client_id: row.get("client_id"),
            key_package: row.get("key_package"),
            created: row.get("created"),
        }
    }
}

impl KeyPackage {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO key_package (client_id, key_package) VALUES ($1,$2) RETURNING *;",
        )
        .bind(self.client_id)
        .bind(&self.key_package)
        .fetch_one(db)
        .await?
        .borrow()
        .into();

        Ok(())
    }

    pub async fn delete_all_by_client_id(db: &DbPool, client_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM key_package WHERE client_id = $1;")
            .bind(client_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn get_one_with_count(
        db: &DbPool,
        client_id: i32,
    ) -> Result<(Option<Self>, i32), sqlx::Error> {
        let res = sqlx::query("SELECT *, COUNT(*) as count FROM key_package WHERE client_id = $1;")
            .bind(client_id)
            .fetch_one(db)
            .await?;
        if res.get::<i32, _>("count") == 0i32 {
            return Ok((None, 0));
        }

        Ok((Some(res.borrow().into()), res.get("count")))
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM key_package WHERE id = $1;")
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
}
