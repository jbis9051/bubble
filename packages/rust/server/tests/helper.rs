use axum_test_helper::TestClient;
use bubble::router;
use bubble::types::DbPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

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
