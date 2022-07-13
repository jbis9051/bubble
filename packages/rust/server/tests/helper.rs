use axum_test_helper::TestClient;
use bubble::router;
use bubble::types::DbPool;
use sqlx::postgres::PgPoolOptions;
use std::future::Future;
use std::{env, thread};
use tokio::runtime::Runtime;

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
