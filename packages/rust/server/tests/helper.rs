use axum_test_helper::{TestClient, TestResponse};

use bubble::models::user::User;
use bubble::router;

use bubble::routes::group::GroupName;
use bubble::types::DbPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::Row;
use std::future::Future;
use std::{env, thread};
use tokio::runtime::Runtime;
use uuid::Uuid;

#[macro_export]
macro_rules! cleanup {
    ($structdef: tt, |$dbarg:ident, $resourcesarg: ident| $code: expr) => {{
        #[derive(Default)]
        struct CleanupResources
            $structdef

        let cleanup_resources = CleanupResources::default();

        use sqlx::postgres::PgPoolOptions;
        use std::env;

        async fn cleanup($resourcesarg: CleanupResources) {
            let $dbarg = PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DB_URL").unwrap())
                .await
                .unwrap();
            $code
        }

        $crate::helper::Cleanup::new(Box::new(cleanup), cleanup_resources)
    }};

    (|$dbarg:ident| $code: expr) => {{
        use sqlx::postgres::PgPoolOptions;
        use std::env;

        async fn cleanup(_: ()) {
            let $dbarg = PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DB_URL").unwrap())
                .await
                .unwrap();
            $code
        }

        $crate::helper::Cleanup::new(Box::new(cleanup), ())
    }}
}

type CB<F, Resources> = Box<dyn Fn(Resources) -> F>;

pub struct Cleanup<F: 'static + Future + Send, Resources: Default> {
    cleanup: Option<CB<F, Resources>>,
    pub resources: Resources,
}

impl<F: 'static + Future + Send, Resources: Default> Cleanup<F, Resources> {
    pub fn new(cleanup: CB<F, Resources>, resources: Resources) -> Self {
        Self {
            cleanup: Some(cleanup),
            resources,
        }
    }
}
impl<F: 'static + Future + Send, Resources: Default> Drop for Cleanup<F, Resources> {
    fn drop(&mut self) {
        let cleanup = self.cleanup.take().unwrap();
        let resources = std::mem::take(&mut self.resources);
        let future = cleanup(resources);
        thread::spawn(|| {
            Runtime::new().unwrap().block_on(future);
        })
        .join();
    }
}

pub async fn start_server() -> (DbPool, TestClient) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DB_URL").unwrap())
        .await
        .unwrap();

    let pool2 = pool.clone();

    let router = router::router(pool);

    (pool2, TestClient::new(router))
}

pub async fn initialize_user(db: &DbPool, _client: &TestClient, username_in: &str) -> (Uuid, User) {
    let temp_username: String = username_in.parse().unwrap();
    let test_user: User = User {
        id: 1,
        uuid: Uuid::new_v4(),
        username: temp_username,
        password: "johndoe".to_string(),
        profile_picture: None,
        email: None,
        phone: None,
        name: "John Doe".to_string(),
        created: NaiveDateTime::from_timestamp(0, 0),
    };

    let test_user = User::create(db, test_user).await.unwrap();

    let token = User::create_session(db, &test_user).await.unwrap();

    (token, test_user)
}

//Return Type: role_id refers to ID of user
pub async fn get_user_group(db: &DbPool, group_id: i32) -> Result<i32, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM user_group WHERE group_id = $1")
        .bind(group_id)
        .fetch_one(db)
        .await?;
    let role_id = row.get("role_id");
    Ok(role_id)
}

pub async fn create_group(
    _db: &DbPool,
    client: &TestClient,
    group_name: &str,
    bearer: String,
) -> Result<TestResponse, sqlx::Error> {
    let group_name: String = group_name.parse().unwrap();
    let res = client
        .post("/group/create")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&GroupName { name: group_name }).unwrap())
        .header("Authorization", bearer)
        .send()
        .await;
    Ok(res)
}
