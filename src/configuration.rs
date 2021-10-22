use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server: Server,
    pub postgres: Postgres,
}

impl Configuration {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();

        c.merge(File::with_name("Config")).unwrap();
        c.merge(Environment::new().separator("_")).unwrap();

        c.try_into()
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
    sslmode: bool,
}

impl Postgres {
    pub fn options(&self) -> PgConnectOptions {
        let ssl_mode = if self.sslmode {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .username(&self.user)
            .password(&self.password)
            .host(&self.host)
            .port(self.port)
            .database(&self.database)
            .ssl_mode(ssl_mode)
    }
}
