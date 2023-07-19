use crate::types::DbPool;
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use uuid::Uuid;

pub struct Group {
    pub id: i32,
    pub uuid: Uuid,
    pub name: Option<String>,
    pub image: Option<Vec<u8>>,
    pub updated_at: NaiveDateTime,
    pub in_group: bool,
}

impl From<&SqliteRow> for Group {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            uuid: row.get("uuid"),
            name: row.get("name"),
            image: row.get("image"),
            updated_at: row.get("updated_at"),
            in_group: row.get("in_group"),
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

    pub async fn update(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = (&sqlx::query(
            "UPDATE \"group\" SET name = $1, image = $2, updated_at = $3, in_group = $4 WHERE id = $5 RETURNING *;",
        )
            .bind(&self.name)
            .bind(&self.image)
            .bind(self.updated_at)
            .bind(self.in_group)
            .bind(self.id)
            .fetch_one(db)
            .await?)
            .into();
        Ok(())
    }

    pub async fn all(db: &DbPool) -> Result<Vec<Group>, sqlx::Error> {
        let locations = sqlx::query("SELECT * FROM \"group\"").fetch_all(db).await?;
        Ok(locations.iter().map(Group::from).collect())
    }

    pub async fn all_in_group(db: &DbPool) -> Result<Vec<Group>, sqlx::Error> {
        let locations = sqlx::query("SELECT * FROM \"group\" WHERE in_group = TRUE")
            .fetch_all(db)
            .await?;
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
