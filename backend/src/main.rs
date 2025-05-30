use axum::{routing::get, Router};
use clap::Parser;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod routes; // Assuming routes.rs will be created next

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
    database_url: String, // This will be used later
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer())
        .init();

    let cli = Cli::parse();

    tracing::info!("Starting server, listening on {}", cli.app_bind_address);
    tracing::debug!("CLI arguments: {:?}", cli);
    // DATABASE_URL will be used later. For now, we just log it if provided.
    if !cli.database_url.is_empty() {
        tracing::info!("DATABASE_URL is set (will be used for DB connection later)");
    } else {
        tracing::warn!("DATABASE_URL is not set. This will be required for database operations.");
    }

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(routes::health_check)) // Mount health_check from routes.rs
        .layer(TraceLayer::new_for_http());

    // Parse the bind address
    let addr: SocketAddr = cli
        .app_bind_address
        .parse()
        .expect("Unable to parse bind address");

    tracing::info!("Listening on {}", addr);

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
