use sqlx::postgres::{PgRow, Postgres};
use std::time::SystemTime;
use sqlx::pool::PoolConnection;
use sqlx::{Error, Row};

pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub profile_picture: String,
    pub email: String,
    pub phone: Option<String>,
    pub name: String,
    pub created: String
}

impl User {
    pub async fn signup(mut conn: PoolConnection<Postgres>, user: &User) -> Result<User, sqlx::Error> {

        let user = sqlx::query("INSERT INTO user($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(tmp_id)
            .bind(tmp_uuid)
            .bind(username)
            .bind(password)
            .bind(tmp_profile_picture)
            .bind(email)
            .bind(phone)
            .bind(name)
            .bind(tmp_created)
            .fetch_one(&mut conn)
            .await?;

        let confirm = sqlx::query()

        let user = User {
            id: tmp_id,
            uuid: tmp_uuid,
            username,
            email,
            password,
            phone,
            name,
            created: tmp_created,
            profile_picture: "".to_string()
        };


        Ok(user)
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

    fn get_by_uuid(mut conn: PoolConnection<Postgres>, uuid: String) -> Option<User> {
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

