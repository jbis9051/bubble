use axum_test_helper::TestClient;

use bubble::models::user::User;
use bubble::router;

use bubble::types::DbPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDateTime;
use std::future::Future;
use std::{env, thread};
use tokio::runtime::Runtime;
use uuid::Uuid;

#[macro_export]
macro_rules! cleanup {
    (|$dbarg:ident| $code: expr) => {{
        use sqlx::postgres::PgPoolOptions;
        use std::env;

        async fn cleanup() {
            let $dbarg = PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DB_URL").unwrap())
                .await
                .unwrap();
            $code
        }

        Cleanup::new(Box::new(cleanup))
    }};
}

type CB<F> = Box<dyn Fn() -> F>;

pub struct Cleanup<F: 'static + Future + Send> {
    cleanup: Option<CB<F>>,
}

impl<F: 'static + Future + Send> Cleanup<F> {
    pub fn new(cleanup: CB<F>) -> Self {
        Self {
            cleanup: Some(cleanup),
        }
    }
}
impl<F: 'static + Future + Send> Drop for Cleanup<F> {
    fn drop(&mut self) {
        let cleanup = self.cleanup.take().unwrap();
        let future = cleanup();
        thread::spawn(|| {
            Runtime::new().unwrap().block_on(future);
        })
        .join()
        .unwrap();
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

pub async fn initialize_user(db: &DbPool, _client: &TestClient) -> (String, User) {
    let test_user: User = User {
        id: 1,
        uuid: Uuid::new_v4(),
        username: "John Doe".to_string(),
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
