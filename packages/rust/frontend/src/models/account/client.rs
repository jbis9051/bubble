use crate::types::DbPool;
use common::http_types::PublicClient;
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

pub struct Client {
    pub id: i32,
    pub uuid: Uuid,
    pub user_uuid: Uuid,
    pub signing_key: Vec<u8>,
    pub validated_date: NaiveDateTime,
    pub created_date: NaiveDateTime,
}

impl From<&SqliteRow> for Client {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            user_uuid: row.get("user_uuid"),
            signing_key: row.get("signing_key"),
            validated_date: row.get("validated_date"),
            created_date: row.get("created_date"),
        }
    }
}

impl From<PublicClient> for Client {
    fn from(value: PublicClient) -> Self {
        Self {
            id: 0,
            uuid: value.uuid,
            user_uuid: value.user_uuid,
            signing_key: value.signing_key.0,
            validated_date: Utc::now().naive_utc(),
            created_date: Default::default(),
        }
    }
}

impl Client {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = (&sqlx::query("INSERT INTO \"client\" (uuid, user_uuid, signing_key, validated_date) VALUES ($1, $2, $3, $4) RETURNING *;")
            .bind(self.uuid)
            .bind(self.user_uuid)
            .bind(&self.signing_key)
            .bind(self.validated_date)
            .fetch_one(db)
            .await?)
            .into();
        Ok(())
    }

    pub async fn from_uuid(db: &DbPool, uuid: &Uuid) -> Result<Client, sqlx::Error> {
        Ok((&sqlx::query("SELECT * FROM \"client\" WHERE uuid = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?)
            .into())
    }

    pub async fn try_from_uuid(db: &DbPool, uuid: &Uuid) -> Result<Option<Client>, sqlx::Error> {
        let client: Option<Client> = sqlx::query("SELECT * FROM \"client\" WHERE uuid = $1;")
            .bind(uuid)
            .fetch_optional(db)
            .await?
            .as_ref()
            .map(|row| row.into());
        Ok(client)
    }

    pub async fn delete_by_uuid(db: &DbPool, uuid: &Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM \"client\" WHERE uuid = $1;")
            .bind(uuid)
            .execute(db)
            .await?;
        Ok(())
    }
}
