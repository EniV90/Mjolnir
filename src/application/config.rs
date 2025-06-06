use dotenvy::dotenv;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub service_host: String,
    pub service_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub postgres_db: String,
    pub postgres_password: String,
    pub postgres_user: String,
}

impl Config {
    pub fn service_http_addr(&self) -> String {
        format!("{}://{}:{}", "http", self.service_host, self.service_port)
    }

    pub fn service_socket_addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.service_host, self.service_port)).unwrap()
    }

    pub fn load() {
        let _ = dotenv();

        let config = Config {
            service_host: env_get("SERVICE_HOST"),
            service_port: env_parse("SERVICE_PORT"),
            database_url: env_get("DATABASE_URL"),
            redis_url: env_get("REDIS_URL"),
            postgres_db: env_get("POSTGRES_DB"),
            postgres_password: env_get("POSTGRES_PASSWORD"),
            postgres_user: env_get("POSTGRES_USER"),
        };

        tracing::trace!("Configuration: {:?}", config);
        let _ = CONFIG.set(config);
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }
}

#[inline]
fn env_get(key: &str) -> String {
    match std::env::var(key) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("{} {}", key, e);
            tracing::error!("{}", msg);
            panic!("{}", msg);
        }
    }
}

#[inline]
fn env_parse<T: std::str::FromStr>(key: &str) -> T {
    match env_get(key).parse() {
        Ok(v) => v,
        Err(_) => {
            let msg = format!("Failed to parse: {}", key);
            tracing::error!("{}", msg);
            panic!("{}", msg)
        }
    }
}
