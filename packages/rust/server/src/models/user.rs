use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;

use crate::routes::user::Confirmation;
use crate::types::DbPool;
use rand_core::{OsRng, RngCore};
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

impl User {
    pub async fn create(db: &DbPool, mut user: User) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            "INSERT INTO \"user\" (uuid, username, password, profile_picture, email, phone, name)
                             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *;",
        )
        .bind(&user.uuid)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.profile_picture)
        .bind(Option::<String>::None)
        .bind(&user.phone)
        .bind(&user.name)
        .fetch_one(db)
        .await
        .unwrap();

        user.user_from_row(&row).await.unwrap();
        Ok(user)
    }

    pub async fn create_confirmation(
        db: &DbPool,
        user: &User,
        email: &str,
    ) -> Result<Uuid, sqlx::Error> {
        let link_id = Uuid::new_v4();

        sqlx::query(
            "INSERT INTO confirmation (user_id, link_id, email)
                             VALUES ($1, $2, $3);",
        )
        .bind(&user.id)
        .bind(&link_id)
        .bind(&email)
        .execute(db)
        .await?;

        Ok(link_id)
    }

    pub async fn get_by_link_id(db: &DbPool, link_id: Uuid) -> Result<Confirmation, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM confirmation WHERE link_id = $1;")
            .bind(link_id)
            .fetch_one(db)
            .await?;

        let confirmation = Confirmation {
            id: row.get("id"),
            user_id: row.get("user_id"),
            link_id: row.get("link_id"),
            email: row.get("email"),
            created: row.get("created"),
        };
        Ok(confirmation)
    }

    pub async fn delete_confirmation(db: &DbPool, conf_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM confirmation WHERE id = $1")
            .bind(conf_id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn create_session(db: &DbPool, user: &User) -> Result<String, sqlx::Error> {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        let token = String::from_utf8_lossy(key.as_slice()).to_string();

        sqlx::query("INSERT INTO session_token (user_id, token) VALUES ($1, $2);")
            .bind(&user.id)
            .bind(&token)
            .execute(db)
            .await?;

        Ok(token)
    }

    pub async fn get_by_id(db: &DbPool, id: i32) -> Result<User, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"user\" WHERE id = $1;")
            .bind(id)
            .fetch_one(db)
            .await?;

        let mut user = User::empty_user().await;
        user.user_from_row(&row).await.unwrap();
        Ok(user)
    }

    pub async fn get_by_email(db: &DbPool, email: &str) -> Result<User, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM \"user\" WHERE email = $1")
            .bind(email)
            .fetch_one(db)
            .await?;

        let mut user = User::empty_user().await;
        user.user_from_row(&row).await.unwrap();
        Ok(user)
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

    pub async fn delete_session(db: &DbPool, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM session_token WHERE token = $1;")
            .bind(token)
            .execute(db)
            .await?;
        Ok(())
    }

    async fn user_from_row(&mut self, row: &PgRow) -> Result<(), sqlx::Error> {
        self.id = row.get("id");
        self.uuid = row.get("uuid");
        self.username = row.get("username");
        self.password = row.get("password");
        self.profile_picture = row.get("profile_picture");
        self.email = row.get("email");
        self.phone = row.get("phone");
        self.name = row.get("name");
        self.created = row.get("created");

        Ok(())
    }

    pub async fn empty_user() -> User {
        let user: User = User {
            id: Default::default(),
            uuid: Default::default(),
            username: "".to_string(),
            password: "".to_string(),
            profile_picture: None,
            email: None,
            phone: None,
            name: "".to_string(),
            created: NaiveDateTime::from_timestamp(0, 0),
        };
        user
    }
}
