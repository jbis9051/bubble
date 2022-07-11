use axum::Router;
use bubble::{config, router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config::CONFIG.db_url)
        .await
        .unwrap();

    let router = router::router(pool);

    axum::Server::bind(&config::CONFIG.listen_addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
