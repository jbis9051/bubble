use crate::types::DbPool;
use sqlx::postgres::PgRow;
use sqlx::types::Uuid;
use sqlx::Row;
use std::borrow::Borrow;

use crate::models::client::Client;
use sqlx::types::chrono::NaiveDateTime;

pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub name: String,
    pub identity: Vec<u8>,
    pub primary_client_id: Option<i32>,
    pub created: NaiveDateTime,
}

impl From<&PgRow> for User {
    fn from(row: &PgRow) -> Self {
        User {
            id: row.get("id"),
            uuid: row.get("uuid"),
            username: row.get("username"),
            password: row.get("password"),
            email: row.get("email"),
            name: row.get("name"),
            identity: row.get("identity"),
            primary_client_id: row.get("primary_client_id"),
            created: row.get("created"),
        }
    }
}

impl User {
    pub async fn create(&mut self, db: &DbPool) -> Result<(), sqlx::Error> {
        *self = sqlx::query(
            "INSERT INTO \"user\" (uuid, username, password, email, name, identity)
                             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;",
        )
        .bind(self.uuid)
        .bind(&self.username)
        .bind(&self.password)
        .bind(&self.email)
        .bind(&self.name)
        .bind(&self.identity)
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
                      password = $3,
                      email = $4,
                      name = $5,
                      identity = $6,
                      primary_client_id = $7
                  WHERE id = $8;",
        )
        .bind(self.uuid)
        .bind(&self.username)
        .bind(&self.password)
        .bind(&self.email)
        .bind(&self.name)
        .bind(&self.identity)
        .bind(self.primary_client_id)
        .bind(self.id)
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, db: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM \"user\" WHERE id = $1;")
            .bind(self.id)
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn client(&self, db: &DbPool) -> Result<Option<Client>, sqlx::Error> {
        if let Some(client_id) = self.primary_client_id {
            Ok(Some(Client::from_id(db, client_id).await?))
        } else {
            Ok(None)
        }
    }
}
