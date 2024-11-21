mod models;
mod config;
mod dtos;
mod error;
mod db;
mod utils;
mod middleware;
mod mail;
mod services;
mod routes;

use std::sync::Arc;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use config::Config;
use db::DBClient;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();
    dotenv().ok();
    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url).await
    {
        Ok(pool) => {
            println!("Successfully connected to database");
            pool
        }
        Err(err) => {
            println!("Failed to connect the database: {:?}", err);
            std::process::exit(1)
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().expect("Failed parse header value")) // ex single allow "http://localhost:3000".parse::<HeaderValue>().unwrap()
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        // .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let db_client = DBClient::new(pool);
    let app_state = AppState{
        env: config.clone(),
        db_client
    };

    let app = routes::create(Arc::new(app_state.clone())).layer(cors.clone());

    println!("{}", format!("Server is running on http://localhost:{}", config.port));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
