mod routes;
mod utils;
mod model;

use std::error::Error;
use axum::Router;
use axum::routing::{get, patch, post};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use crate::routes::{create_link, health, redirect, update_link};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("Environment variable 'DATABASE_URL' is required");

    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("Could not connect to the database");

    let app = Router::new()
        .route("/create", post(create_link))
        .route("/:id", patch(update_link).get(redirect))
        .route("/health", get(health))
        .with_state(db_pool);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not initialize TcpListener");

    axum::serve(listener, app)
        .await
        .expect("Could not start the server");

    Ok(())
}
