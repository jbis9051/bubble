use crate::DbPool;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::{Error, Row};

use rand_core::{RngCore, OsRng};
use serde::de::Unexpected::Str;

pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub name: String,
    pub created: String,
}

impl User {
    pub async fn signup(db: &DbPool, user: &User) -> Result<String, sqlx::Error> {
        sqlx::query("INSERT INTO user($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&user.uuid)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.profile_picture)
            .bind(Option::<String>::None)
            .bind(&user.phone)
            .bind(&user.name)
            .bind(&user.created)
            .execute(db)
            .await?;

        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);
        let link_id = String::from_utf8_lossy(key.as_slice()).to_string();

        sqlx::query("INSERT INTO confirmation($1, $2, $3, $4)")
            .bind(&user.id)
            .bind(&link_id)
            .bind(&user.email)
            .bind(&user.created)
            .execute(db)
            .await?;

        Ok(link_id)
    }

    async fn get_by_id(mut conn: PoolConnection<Postgres>, _id: String) -> Result<User, Error> {
        let select_query = sqlx::query("SELECT id FROM user");
        let user: User = select_query
            .map(|row: PgRow| User {
                id: row.get("id"),
                uuid: row.get("uuid"),
                username: row.get("username"),
                password: row.get("password"),
                profile_picture: row.get("profile_picture"),
                email: row.get("email"),
                phone: row.get("phone"),
                name: row.get("name"),
                created: row.get("created"),
            })
            .fetch_one(&mut conn)
            .await?;
        Ok(user)
    }

    fn get_by_uuid(_conn: PoolConnection<Postgres>, _uuid: String) -> Result<User, sqlx::Error> {
        todo!();
    }
    fn update(&self, _conn: PoolConnection<Postgres>) {
        todo!();
    }
    fn delete(&self, _conn: PoolConnection<Postgres>) {
        todo!();
        // remove routes from a whole bunch of things
        // delete routes row
    }
}
