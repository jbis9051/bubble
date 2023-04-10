use crate::models::session::Session;
use crate::types::DbPool;
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

pub async fn create_session(db: &DbPool, user_id: i32) -> Result<Uuid, sqlx::Error> {
    let mut session = Session {
        id: 0,
        user_id,
        token: Uuid::new_v4(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    session.create(db).await?;

    Ok(session.token)
}
