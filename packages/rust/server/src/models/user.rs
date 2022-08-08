use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;

use crate::types::DbPool;

use sqlx::types::chrono::NaiveDateTime;

pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: String,
    pub created: NaiveDateTime,
}

impl From<PgRow> for User {
    fn from(row: PgRow) -> Self {
        User {
            id: row.get("id"),
            uuid: row.get("uuid"),
            username: row.get("username"),
            password: row.get("password"),
            profile_picture: row.get("profile_picture"),
            email: row.get("email"),
            phone: row.get("phone"),
            name: row.get("name"),
            created: row.get("created"),
        }
    }
}

impl User {
    pub async fn create(&self, db: &DbPool) -> Result<User, sqlx::Error> {
        Ok(sqlx::query(
            "INSERT INTO \"user\" (uuid, username, password, profile_picture, email, phone, name)
                             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;",
        )
        .bind(&self.uuid)
        .bind(&self.username)
        .bind(&self.password)
        .bind(&self.profile_picture)
        .bind(&self.email)
        .bind(&self.phone)
        .bind(&self.name)
        .fetch_one(db)
        .await?
        .into())
    }

    pub async fn from_id(db: &DbPool, id: i32) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE id = $1;")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap()
            .into())
    }

    pub async fn from_username(db: &DbPool, username: &str) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE username = $1;")
            .bind(username)
            .fetch_one(db)
            .await?
            .into())
    }

    pub async fn from_session(db: &DbPool, session_token: &str) -> Result<User, sqlx::Error> {
        let token = Uuid::parse_str(session_token).unwrap();
        Ok(sqlx::query(
            "SELECT *
                 FROM session_token
                 INNER JOIN \"user\"
                 ON session_token.user_id = \"user\".id
                 WHERE session_token.token = $1;",
        )
        .bind(token)
        .fetch_one(db)
        .await?
        .into())
    }

    pub async fn from_email(db: &DbPool, email: &str) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE email = $1")
            .bind(email)
            .fetch_one(db)
            .await?
            .into())
    }

    pub async fn from_uuid(db: &DbPool, uuid: Uuid) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE uuid = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?
            .into())
    }

    pub async fn update(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE \"user\"
                  SET username = $1,
                      password = $2,
                      profile_picture = $3,
                      email = $4,
                      phone = $5,
                      name = $6
                  WHERE id = $7",
        )
        .bind(&self.username)
        .bind(&self.password)
        .bind(&self.profile_picture)
        .bind(&self.email)
        .bind(&self.phone)
        .bind(&self.name)
        .bind(&self.id)
        .execute(db)
        .await?;
        Ok(())
    }

    //TODO
    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM \"user\" WHERE id = $1;")
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }
}
