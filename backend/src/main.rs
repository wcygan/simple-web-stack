use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes; // Assuming routes.rs will be created next

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The address to bind the server to
    #[clap(
        short,
        long,
        env = "APP_BIND_ADDRESS",
        default_value = "127.0.0.1:3000"
    )]
    bind_address: String,

    /// Database connection string
    #[clap(
        long,
        env = "DATABASE_URL",
        default_value = "mysql://taskuser:taskpassword@localhost:3306/tasks_db"
    )]
    database_url: String, // This will be used later
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info,backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    // Build our application with a route
    let app = Router::new()
        .route("/health", get(routes::health_check)) // Mount health_check from routes.rs
        .layer(TraceLayer::new_for_http());

    // Parse the bind address
    let addr: SocketAddr = cli
        .bind_address
        .parse()
        .expect("Unable to parse bind address");

    tracing::info!("listening on {}", addr);

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
