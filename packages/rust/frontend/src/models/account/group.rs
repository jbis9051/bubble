use crate::types::DbPool;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;
use uuid::Uuid;

pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub name: Option<String>,
    pub image: Option<Vec<u8>>,
}

impl From<&SqliteRow> for Group {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            name: row.get("name"),
            image: row.get("image"),
        }
    }
}

impl Group {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = (&sqlx::query(
            "INSERT INTO \"group\" (uuid, name, image) VALUES ($1, $2, $3) RETURNING *;",
        )
        .bind(self.uuid)
        .bind(&self.name)
        .bind(&self.image)
        .fetch_one(db)
        .await?)
            .into();
        Ok(())
    }

    pub async fn all(db: &DbPool) -> Result<Vec<Group>, sqlx::Error> {
        let locations = sqlx::query("SELECT * FROM \"group\"").fetch_all(db).await?;
        Ok(locations.iter().map(Group::from).collect())
    }

    pub async fn from_uuid(db: &DbPool, uuid: Uuid) -> Result<Option<Group>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"group\" WHERE uuid = $1")
            .bind(uuid)
            .fetch_optional(db)
            .await?
            .map(|r| (&r).into());
        Ok(row)
    }
}
