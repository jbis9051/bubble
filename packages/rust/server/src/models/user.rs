use crate::DbPool;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgRow, Postgres};
use sqlx::Row;

use rand_core::{OsRng, RngCore};

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
        sqlx::query("INSERT INTO user($1, $2, $3, $4, $5, $6, $7)")
            .bind(&user.uuid)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.profile_picture)
            .bind(Option::<String>::None)
            .bind(&user.phone)
            .bind(&user.name)
            .execute(db)
            .await?;

        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);
        let link_id = String::from_utf8_lossy(key.as_slice()).to_string();

        sqlx::query("INSERT INTO confirmation($1, $2, $3)")
            .bind(&user.id)
            .bind(&link_id)
            .bind(&user.email)
            .execute(db)
            .await?;

        Ok(link_id)
    }

    async fn get_by_id(db: &DbPool, id: i32) -> Result<User, sqlx::Error> {
        let row = sqlx::query("SELECT (1) FROM user WHERE id IS $1")
            .bind(id)
            .fetch_one(db)
            .await?;

        let user = User::user_by_row(row);
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
    fn user_by_row(row: PgRow) -> User {
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
