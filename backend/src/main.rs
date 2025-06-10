mod models;
mod db;
mod api;

use tokio;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    env_logger::init();

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Pulse backend server...");

    // Initialize database
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db = db::Database::new(&database_url).await?;
    db.init().await?;

    // Initialize API state
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    let state = api::AppState {
        db,
        jwt_secret,
    };

    // Create and start the API server
    let app = api::create_router(state);
    let addr = format!(
        "{}:{}",
        env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string())
    );
    
    info!("Server running on {}", addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
} 