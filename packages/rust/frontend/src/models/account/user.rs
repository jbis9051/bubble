use crate::types::DbPool;
use sqlx::sqlite::SqliteRow;
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

impl User {
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
