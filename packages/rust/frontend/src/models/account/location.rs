use crate::types::DbPool;
use crate::Error;

use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Row, SqlitePool};

use uuid::Uuid;

pub struct Location {
    pub id: i32,
    pub client_uuid: Uuid,
    pub group_uuid: Uuid,
    pub longitude: f64,
    pub latitude: f64,
    pub location_date: NaiveDateTime,
    pub raw: Vec<u8>,
    pub created_date: NaiveDateTime,
}

impl From<&SqliteRow> for Location {
    fn from(row: &SqliteRow) -> Self {
        let location_date: i64 = row.get("location_date");
        let location_date = NaiveDateTime::from_timestamp_millis(location_date).unwrap();
        Self {
            id: row.get("id"),
            client_uuid: row.get("client_uuid"),
            group_uuid: row.get("group_uuid"),
            longitude: row.get("longitude"),
            latitude: row.get("latitude"),
            location_date,
            raw: row.get("raw"),
            created_date: row.get("created_date"),
        }
    }
}

impl Location {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), Error> {
        *self = (&sqlx::query("INSERT INTO location (client_uuid, group_uuid, longitude, latitude, location_date, raw) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(self.client_uuid)
            .bind(self.group_uuid)
            .bind(self.longitude)
            .bind(self.latitude)
            .bind(self.location_date.timestamp_millis())
            .bind(&self.raw)
            .fetch_one(db).await?).into();
        Ok(())
    }

    pub async fn query(
        db: &DbPool,
        group_uuid: &Uuid,
        client_uuid: &Uuid,
        before: &NaiveDateTime,
        amount: u32,
    ) -> Result<Vec<Location>, sqlx::Error> {
        let locations = sqlx::query("SELECT * FROM location WHERE location_date < $1 AND group_uuid = $2 AND client_uuid = $3 ORDER BY location_date DESC LIMIT $4")
            .bind(before.timestamp_millis())
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
            .bind(from.timestamp_millis())
            .bind(to.timestamp_millis())
            .bind(group_uuid)
            .bind(client_uuid)
            .fetch_one(db)
            .await?
            .get("count");
        Ok(count)
    }
}
