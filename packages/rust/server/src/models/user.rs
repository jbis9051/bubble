use sqlx::{FromRow, Row};
use sqlx::postgres::{PgPoolOptions, PgRow};

pub struct User {
    id: i32,
    uuid: String,
    username: String,
    password: String,
    profile_picture: String,
    email: String,
    phone: Option<String>,
    name: String,
    created: String
}

impl User {
    pub fn create(username: String, email: String, password: String, phone: Option<String>, name: String) -> Result<User, Error> {


        let t_id: i32 = 0;
        let t_uuid: String = "0".to_string();
        let t_created: String = "0".to_string();

        let user = sqlx::query("select * from user where id is (id)")
            .fetch_one(&mut pool)
            .await?;

        let user = User {
            id: t_id,
            uuid: t_uuid,
            username,
            email,
            password,
            phone,
            name,
            created: t_created,
        };


        Ok(user)
    }

    async fn get_by_id(id: String) -> Option<User> {
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
            .fetch_one(&pool)
            .await?;
        user
    }
    fn get_by_uuid(uuid: String) -> Option<User> {
        todo!();
    }
    fn update(&self) {
        todo!();
    }
    fn delete(&self) {
        todo!();
        // remove routes from a whole bunch of things
        // delete routes row
    }
}

