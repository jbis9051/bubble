use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;
use std::borrow::Borrow;

use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::types::DbPool;

use crate::models::member::Member;
use sqlx::types::chrono::{NaiveDateTime, Utc};

pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub password: Option<String>,
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
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO \"user\" (uuid, username, password, profile_picture, email, phone, name, deleted)
                             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
        )
            .bind(&self.uuid)
            .bind(&self.username)
            .bind(&self.password)
            .bind(&self.profile_picture)
            .bind(&self.email)
            .bind(&self.phone)
            .bind(&self.name)
            .bind(&self.deleted)
            .fetch_one(db)
            .await?
            .borrow()
            .into();

        Ok(())
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
            "SELECT \"user\".*
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

    pub async fn from_uuid(db: &DbPool, uuid: &Uuid) -> Result<User, sqlx::Error> {
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
                  SET uuid = $1,
                      username = $2,
                      profile_picture = $3,
                      email = $4,
                      phone = $5,
                      name = $6,
                      deleted = $7
                  WHERE id = $8;",
        )
        .bind(&self.uuid)
        .bind(&self.username)
        .bind(&self.profile_picture)
        .bind(&self.email)
        .bind(&self.phone)
        .bind(&self.name)
        .bind(&self.deleted)
        .bind(&self.id)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn update_password(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE \"user\"
                  SET password = $1
                  WHERE id = $2;",
        )
        .bind(&self.password)
        .bind(self.id)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn delete(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        let mut tx = db.begin().await?;

        Member::delete_all_by_user_id(db, self.id).await?;
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
                          SET username = $1,
                              password = $2,
                              profile_picture = $3,
                              email = $4,
                              phone = $5,
                              name = $6,
                              deleted = $7
                         WHERE id = $8;",
        )
        .bind(format!("DELETED_USER_{}", Uuid::new_v4()))
        .bind(None::<String>)
        .bind(None::<String>)
        .bind(None::<String>)
        .bind(None::<String>)
        .bind("Deleted Account")
        .bind(Some(Utc::now()))
        .bind(self.id)
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let correct = match &self.password {
            None => return false,
            Some(pass) => pass.as_str(),
        };
        let correct = match PasswordHash::new(correct) {
            Ok(pass) => pass,
            Err(_) => return false,
        };
        let password = password.as_bytes();

        Argon2::default()
            .verify_password(password, &correct)
            .is_ok()
    }
}
