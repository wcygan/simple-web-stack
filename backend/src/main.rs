use axum::{routing::get, Router};
use clap::Parser;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod auth;
mod db;
mod errors;
mod middleware;
mod models;
mod routes;

use auth::{AuthConfig, AuthService};
use db::{create_pool, init_db, AppState};

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The address to bind the server to
    #[clap(long, env = "APP_BIND_ADDRESS", default_value = "127.0.0.1:3000")]
    app_bind_address: String,

    /// Database connection string
    #[clap(
        long,
        env = "DATABASE_URL",
        default_value = "mysql://taskuser:taskpassword@mysql:3306/tasks_db"
    )]
    database_url: String,
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize tracing subscriber with info logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();

    let cli = Cli::parse();

    tracing::info!("Starting server, listening on {}", cli.app_bind_address);
    tracing::debug!("CLI arguments: {:?}", cli);

    // Create the database connection pool
    let pool = create_pool(&cli.database_url)
        .await
        .expect("Failed to create database pool");

    // Initialize database schema
    init_db(&pool).await.expect("Failed to initialize database");

    // Create auth service
    let auth_config = AuthConfig::from_env();
    let auth_service = AuthService::new(auth_config);

    // Create the application state
    let app_state = AppState { pool, auth_service };

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(routes::health_check)) // Mount health_check from routes.rs
        .nest("/tasks", routes::task_routes())
        .nest("/auth", routes::public_auth_routes())
        .nest("/auth", routes::protected_auth_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Parse the bind address
    let addr: SocketAddr = cli
        .app_bind_address
        .parse()
        .expect("Unable to parse bind address");

    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
