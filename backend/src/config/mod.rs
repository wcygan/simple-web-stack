use clap::Parser;
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(name = "simple-web-stack-backend")]
#[command(about = "A simple web stack backend server")]
#[command(version = "0.1.0")]
pub struct AppConfig {
    /// Server host address
    #[arg(long, default_value = "0.0.0.0", env = "SERVER_HOST")]
    pub host: String,

    /// Server port
    #[arg(short, long, default_value = "3000", env = "SERVER_PORT")]
    pub port: u16,

    /// Log level
    #[arg(long, default_value = "info", env = "LOG_LEVEL")]
    pub log_level: String,

    /// Database URL (optional for development)
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: Option<String>,

    /// Database max connections
    #[arg(long, default_value = "10", env = "DATABASE_MAX_CONNECTIONS")]
    pub database_max_connections: u32,

    /// Database connect timeout in seconds
    #[arg(long, default_value = "30", env = "DATABASE_CONNECT_TIMEOUT")]
    pub database_connect_timeout: u64,

    /// Skip database connection (useful for testing)
    #[arg(long, default_value = "false", env = "SKIP_DATABASE")]
    pub skip_database: bool,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

impl AppConfig {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get server configuration
    pub fn server(&self) -> ServerConfig {
        ServerConfig {
            host: self.host.clone(),
            port: self.port,
        }
    }

    /// Get logging configuration
    pub fn logging(&self) -> LoggingConfig {
        LoggingConfig {
            level: self.log_level.clone(),
        }
    }

    /// Get database configuration if database is enabled
    pub fn database(&self) -> Option<DatabaseConfig> {
        if self.skip_database {
            return None;
        }

        self.database_url.as_ref().map(|url| DatabaseConfig {
            url: url.clone(),
            max_connections: self.database_max_connections,
            min_connections: 1,
            connect_timeout: self.database_connect_timeout,
            idle_timeout: 600,
        })
    }

    /// Get database URL with fallback for development
    pub fn get_database_url(&self) -> String {
        self.database_url
            .clone()
            .unwrap_or_else(|| "mysql://todo_user:todo_password@localhost:3306/todo_db".to_string())
    }
}

// Basic default configuration for now
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            log_level: "info".to_string(),
            database_url: None,
            database_max_connections: 10,
            database_connect_timeout: 30,
            skip_database: false,
        }
    }
}
