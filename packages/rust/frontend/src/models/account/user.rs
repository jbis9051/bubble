use crate::types::DbPool;
use common::http_types::PublicUser;

use sqlx::sqlite::SqliteRow;

use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use uuid::Uuid;

pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub name: String,
    pub identity: Vec<u8>,
    pub updated_date: NaiveDateTime,
}

impl From<&SqliteRow> for User {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            username: row.get("username"),
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
            username: user.username,
            name: user.name,
            identity: user.identity.0,
            updated_date: Default::default(),
        }
    }
}

impl User {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = (&sqlx::query(
            "INSERT INTO \"user\" (uuid, username, name, identity) VALUES ($1, $2, $3, $4) RETURNING *;",
        )
        .bind(self.uuid)
        .bind(&self.username)
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
}
