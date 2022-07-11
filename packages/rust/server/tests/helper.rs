use axum_test_helper::TestClient;
use bubble::router;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub const LISTEN_ADDR: &str = "127.0.0.1:8080";

pub async fn start_server() -> TestClient {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DB_URL").unwrap())
        .await
        .unwrap();

    let router = router::router(pool);

    TestClient::new(router)
}
