use sqlx::postgres::{PgRow, Postgres};
use sqlx::pool::PoolConnection;
use sqlx::{Error, Row};
use crate::DbPool;

pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub name: String,
    pub created: String
}

impl User {
    pub async fn signup(db: & DbPool, user: &User) -> Result<String, sqlx::Error> {

        sqlx::query("INSERT INTO user($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(&user.id)
            .bind(&user.uuid)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.profile_picture)
            .bind(None: Option<String>)
            .bind(&user.phone)
            .bind(&user.name)
            .bind(&user.created)
            .execute(db)
            .await?;

        let confirmation_id = "".to_string();
        let link_id = "qwerty".to_string();
        sqlx::query("INSERT INTO confirmation($1, $2, $3, $4, $5) RETURNING link_id")
            .bind(confirmation_id)
            .bind(&user.id)
            .bind(&link_id)
            .bind(&user.email)
            .bind(&user.created)
            .execute(db)
            .await?;

        Ok(link_id)
    }

    async fn get_by_id(mut conn: PoolConnection<Postgres>, id: String) -> Result<User, Error> {
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

    fn get_by_uuid(mut conn: PoolConnection<Postgres>, uuid: String) -> Result<User, sqlx::Error> {
        todo!();
    }
    fn update(&self, mut conn: PoolConnection<Postgres>) {
        todo!();
    }
    fn delete(&self, mut conn: PoolConnection<Postgres>) {
        todo!();
        // remove routes from a whole bunch of things
        // delete routes row
    }
}

