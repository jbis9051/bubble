use crate::types::DbPool;
use common::http_types::PublicUser;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use uuid::Uuid;

pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub identity: Vec<u8>,
    pub updated_date: NaiveDateTime,
}

impl From<&SqliteRow> for User {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            name: row.get("name"),
            identity: row.get("identity"),
            updated_date: row.get("updated_date"),
        }
    }
}

impl From<PublicUser> for User {
    fn from(user: PublicUser) -> Self {
        Self {
            id: 0,
            uuid: user.uuid,
            name: user.name,
            identity: user.identity.0,
            updated_date: Default::default(),
        }
    }
}

impl User {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = (&sqlx::query(
            "INSERT INTO \"user\" (uuid, name, identity) VALUES ($1, $2, $3) RETURNING *;",
        )
        .bind(self.uuid)
        .bind(&self.name)
        .bind(&self.identity)
        .fetch_one(db)
        .await?)
            .into();
        Ok(())
    }

    pub async fn from_uuid(db: &DbPool, uuid: &Uuid) -> Result<User, sqlx::Error> {
        Ok((&sqlx::query("SELECT * FROM \"user\" WHERE uuid = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?)
            .into())
    }

    pub async fn try_from_uuid(db: &DbPool, uuid: &Uuid) -> Result<Option<User>, sqlx::Error> {
        let user: Option<User> = sqlx::query("SELECT * FROM \"user\" WHERE uuid = $1;")
            .bind(uuid)
            .fetch_optional(db)
            .await?
            .as_ref()
            .map(|row| row.into());
        Ok(user)
    }
    pub async fn create_client(
        &self,
        db: &DbPool,
        signing_key: &[u8],
        signature: &Signature,
    ) -> Result<(), sqlx::Error> {
        let public_user_key: PublicKey = PublicKey::from_bytes(&self.identity).unwrap();
        if public_user_key.verify(signing_key, signature).is_err() {
            return Err(sqlx::Error::RowNotFound);
        }
        sqlx::query(
            "INSERT INTO client (user_id, signing_key, validated_date) VALUES ($1, $2, $3);",
        )
        .bind(self.id)
        .bind(signing_key)
        .bind(chrono::Utc::now().timestamp())
        .execute(db)
        .await
        .unwrap();
        Ok(())
    }
}
