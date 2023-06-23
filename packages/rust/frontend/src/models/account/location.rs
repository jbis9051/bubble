use crate::types::DbPool;
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct Location {
    pub id: i32,
    pub client_uuid: Uuid,
    pub group_uuid: String,
    pub longitude: f64,
    pub latitude: f64,
    pub location_date: NaiveDateTime,
    pub raw: Vec<u8>,
    pub created_date: NaiveDateTime,
}

impl From<&SqliteRow> for Location {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            client_uuid: row.get("client_uuid"),
            group_uuid: row.get("group_uuid"),
            longitude: row.get("longitude"),
            latitude: row.get("latitude"),
            location_date: row.get("location_date"),
            raw: row.get("raw"),
            created_date: row.get("created_date"),
        }
    }
}

impl Location {
    pub async fn query(
        db: &DbPool,
        group_uuid: &Uuid,
        client_uuid: &Uuid,
        before: &NaiveDateTime,
        amount: u32,
    ) -> Result<Vec<Location>, sqlx::Error> {
        let locations = sqlx::query("SELECT * FROM location WHERE location_date < $1 AND group_uuid = $2 AND client_uuid = $3 ORDER BY location_date DESC LIMIT $4")
            .bind(before)
            .bind(group_uuid)
            .bind(client_uuid)
            .bind(amount)
            .fetch_all(db)
            .await?;
        Ok(locations.iter().map(Location::from).collect())
    }

    pub async fn count_query(
        db: &SqlitePool,
        group_uuid: &Uuid,
        client_uuid: &Uuid,
        from: &NaiveDateTime,
        to: &NaiveDateTime,
    ) -> Result<i64, sqlx::Error> {
        let count = sqlx::query("SELECT COUNT(*) as count FROM location WHERE location_date BETWEEN $1 AND $2 AND group_uuid = $3 AND client_uuid = $4")
            .bind(from)
            .bind(to)
            .bind(group_uuid)
            .bind(client_uuid)
            .fetch_one(db)
            .await?
            .get("count");
        Ok(count)
    }
}
