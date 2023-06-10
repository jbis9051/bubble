#[cfg(test)]
use server::services::email::PrinterEmailService;
#[cfg(not(test))]
use server::services::email::SendGridEmailService;
use server::{config, router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config::CONFIG.db_url)
        .await
        .unwrap();

    #[cfg(not(test))]
    let email_service = Arc::new(SendGridEmailService::default());

    #[cfg(test)]
    let email_service = Arc::new(PrinterEmailService::default());

    let router = router::router(pool, email_service);

    axum::Server::bind(&config::CONFIG.listen_addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
