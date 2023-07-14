use crate::types::DbPool;
use sqlx::sqlite::SqliteRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;

pub struct Inbox {
    pub id: i32,
    pub message: Vec<u8>,
    pub server_received_date: NaiveDateTime,
    pub received_date: NaiveDateTime,
}

impl From<&SqliteRow> for Inbox {
    fn from(row: &SqliteRow) -> Self {
        Self {
            id: row.get("id"),
            message: row.get("message"),
            server_received_date: row.get("server_received_date"),
            received_date: row.get("received_date"),
        }
    }
}

impl Inbox {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self =
            (&sqlx::query("INSERT INTO inbox (message, server_received_date, received_date) VALUES (?, ?, ?) RETURNING *")
                .bind(&self.message)
                .bind(self.server_received_date)
                .bind(self.received_date)
                .fetch_one(db)
                .await?)
                .into();
        Ok(())
    }

    pub async fn all(db: &DbPool) -> Result<Vec<Inbox>, sqlx::Error> {
        sqlx::query("SELECT * FROM inbox ORDER BY server_received_date ASC")
            .map(|row: SqliteRow| Inbox::from(&row))
            .fetch_all(db)
            .await
    }

    pub async fn delete_by_id(db: &DbPool, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM inbox WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;
        Ok(())
    }
}
