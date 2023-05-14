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
    pub signing_key: Vec<u8>, // this is the public MLS signing key for the user
    pub signature: Vec<u8>, // this is is Sign(H(signing_key), identity) where identity is user's identity key
    pub created: NaiveDateTime,
}

impl From<&PgRow> for Client {
    fn from(row: &PgRow) -> Self {
        Client {
            id: row.get("id"),
            user_id: row.get("user_id"),
            uuid: row.get("uuid"),
            signing_key: row.get("signing_key"),
            signature: row.get("signature"),
            created: row.get("created"),
        }
    }
}

impl Client {
    pub async fn from_uuid(db: &DbPool, uuid: Uuid) -> Result<Client, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM client WHERE uuid = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn filter_uuids(db: &DbPool, uuids: Vec<Uuid>) -> Result<Vec<Client>, sqlx::Error> {
        // TODO better/cleaner way to get "$1, $2,...$n"
        let mut params = format!("$1");
        for i in 2..=uuids.len() {
            params.push_str(&format!(", ${}", i));
        }

        let query_string = format!("SELECT * FROM client WHERE uuid IN ({});", params);

        let mut query = sqlx::query(&query_string);
        for uuid in uuids {
            query = query.bind(uuid);
        }

        Ok(query
            .fetch_all(db)
            .await?
            .iter()
            .map(|row| row.into())
            .collect())
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
        *self = sqlx::query(
            "INSERT INTO client (user_id, uuid, signing_key, signature) VALUES ($1, $2, $3, $4) RETURNING *;",
        )
        .bind(self.user_id)
        .bind(self.uuid)
        .bind(&self.signing_key)
        .bind(&self.signature)
        .fetch_one(db)
        .await?
        .borrow()
        .into();

        Ok(())
    }

    pub async fn update(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "UPDATE client SET signing_key = $1, signature = $2 WHERE id = $3 RETURNING *;",
        )
        .bind(&self.signing_key)
        .bind(&self.signature)
        .bind(self.id)
        .fetch_one(db)
        .await?
        .borrow()
        .into();

        Ok(())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM client WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;

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
