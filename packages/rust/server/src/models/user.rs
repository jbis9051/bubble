use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;
use std::borrow::Borrow;

use crate::models::group::Group;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use crate::types::DbPool;

use sqlx::types::chrono::{NaiveDateTime, Utc};

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
    pub deleted: Option<NaiveDateTime>,
}

impl From<&PgRow> for User {
    fn from(row: &PgRow) -> Self {
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
            deleted: row.get("deleted"),
        }
    }
}

impl User {
    pub async fn create(
        &mut self,
        db: &DbPool,
        email: &str,
        password: &str,
    ) -> Result<User, sqlx::Error> {
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        self.password = argon2.hash_password(password, &salt).unwrap().to_string();
        let mut tx = db.begin().await?;

        sqlx::query(
            "INSERT INTO \"user\" (uuid, username, password, email, phone, name)
                             VALUES ($1, $2, $3, $4, $5, $6);",
        )
        .bind(&self.uuid)
        .bind(&self.username)
        .bind(&self.password)
        .bind(Some(email))
        .bind(&self.phone)
        .bind(&self.name)
        .execute(&mut tx)
        .await?;

        let user = sqlx::query(
            "UPDATE \"user\"
                  SET email = $1
                  WHERE email = $2
                  RETURNING *;",
        )
        .bind(&self.email)
        .bind(Some(email))
        .fetch_one(&mut tx)
        .await?
        .borrow()
        .into();

        tx.commit().await?;

        Ok(user)
    }

    pub async fn from_id(db: &DbPool, id: i32) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE id = $1;")
            .bind(id)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn from_username(db: &DbPool, username: &str) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE username = $1;")
            .bind(username)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn from_session(db: &DbPool, session_token: Uuid) -> Result<User, sqlx::Error> {
        Ok(sqlx::query(
            "SELECT *
                 FROM \"user\"
                 INNER JOIN session
                 ON \"user\".id = session.user_id
                 WHERE session.token = $1;",
        )
        .bind(session_token)
        .fetch_one(db)
        .await?
        .borrow()
        .into())
    }

    pub async fn from_email(db: &DbPool, email: &str) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE email = $1;")
            .bind(email)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn from_uuid(db: &DbPool, uuid: Uuid) -> Result<User, sqlx::Error> {
        Ok(sqlx::query("SELECT * FROM \"user\" WHERE uuid = $1;")
            .bind(uuid)
            .fetch_one(db)
            .await?
            .borrow()
            .into())
    }

    pub async fn update(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE \"user\"
                  SET username = $1,
                      profile_picture = $2,
                      email = $3,
                      phone = $4,
                      name = $5
                  WHERE id = $6;",
        )
        .bind(&self.username)
        .bind(&self.profile_picture)
        .bind(&self.email)
        .bind(&self.phone)
        .bind(&self.name)
        .bind(&self.id)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn update_password(
        &mut self,
        db: &DbPool,
        password: &str,
    ) -> Result<(), sqlx::Error> {
        let byte_password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(byte_password, &salt)
            .unwrap()
            .to_string();
        sqlx::query(
            "UPDATE \"user\"\
                  SET password = $1
                  WHERE id = $2;",
        )
        .bind(&hashed_password)
        .bind(self.id)
        .execute(db)
        .await?;
        self.password = hashed_password.to_string();
        Ok(())
    }

    pub async fn delete(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        let mut tx = db.begin().await?;

        Group::delete_user_by_user_id(db, self.id).await?;
        sqlx::query("DELETE FROM confirmation WHERE user_id = $1;")
            .bind(self.id)
            .execute(&mut tx)
            .await?;
        sqlx::query("DELETE FROM forgot_password WHERE user_id = $1;")
            .bind(self.id)
            .execute(&mut tx)
            .await?;
        sqlx::query("DELETE FROM session WHERE user_id = $1;")
            .bind(self.id)
            .execute(&mut tx)
            .await?;
        sqlx::query(
            "UPDATE \"user\"
                          SET deleted = $1
                         WHERE id = $2;",
        )
        .bind(Utc::now())
        .bind(self.id)
        .execute(&mut tx)
        .await?;
        tx.commit().await?;

        self.username = format!("DELETED_USER_{}", Uuid::new_v4());
        self.profile_picture = None;
        self.email = None;
        self.phone = None;
        self.name = "".to_string();
        self.update(db).await?;
        self.update_password(db, "").await?;

        Ok(())
    }
}
