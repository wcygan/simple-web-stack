use config;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub logging: LoggingConfig,
    pub database: DatabaseConfig,
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
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()?;
        config.try_deserialize()
    }
}

// Basic default configuration for now
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            logging: LoggingConfig {
                level: "backend=debug,tower_http=debug,axum::rejection=trace".to_string(),
            },
            database: DatabaseConfig {
                url: "mysql://todo_user:todo_password@localhost:3306/todo_db".to_string(),
                max_connections: 10,
                min_connections: 1,
                connect_timeout: 30,
                idle_timeout: 600,
            },
        }
    }
}
