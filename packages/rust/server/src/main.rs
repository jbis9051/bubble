mod config;
mod models;
mod routes;

use sqlx::postgres::PgPoolOptions;

fn main() {
    println!("Hello, world!");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test")
        .await?;
}
