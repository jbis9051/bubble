use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::Uuid;
use sqlx::Row;

use crate::models::user::User;
use crate::types::DbPool;

pub struct ForgotRow {
    pub id: i32,
    pub user_id: i32,
    pub forgot_id: Uuid,
    pub created: NaiveDateTime,
}

pub async fn create_forgot(db: &DbPool, user: &User) -> Result<Uuid, sqlx::Error> {
    let forgot_id = Uuid::new_v4();
    sqlx::query("INSERT INTO forgot_password (user_id, forgot_id) VALUES ($1, $2);")
        .bind(&user.id)
        .bind(forgot_id)
        .execute(db)
        .await
        .unwrap();

    Ok(forgot_id)
}

pub async fn get_forgot(db: &DbPool, forgot_id: &str) -> Result<ForgotRow, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM forgot_password WHERE forgot_id = $1;")
        .bind(forgot_id)
        .fetch_one(db)
        .await
        .unwrap();

    Ok(ForgotRow {
        id: row.get("id"),
        user_id: row.get("user_id"),
        forgot_id: row.get("forgot_id"),
        created: row.get("created"),
    })
}

pub async fn delete_forgot(db: &DbPool, forgot_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM forgot_password WHERE forgot_id = $1")
        .bind(forgot_id)
        .execute(db)
        .await
        .unwrap();

    Ok(())
}
