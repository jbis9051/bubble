mod config;
mod models;
mod routes;

use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await
        .unwrap();

    let router = Router::new()
        .nest("/user", routes::user::router())
        //    .nest("/group", routes::group::router())
        .layer(Extension(pool));

    axum::Server::bind(&config::CONFIG.listen_addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
