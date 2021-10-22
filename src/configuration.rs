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
        c.merge(Environment::new()).unwrap();

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
    pghost: String,
    pgport: u16,
    pguser: String,
    pgpassword: String,
    pgdatabase: String,
    require_ssl: bool,
}

impl Postgres {
    pub fn options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .username(&self.pguser)
            .password(&self.pgpassword)
            .host(&self.pghost)
            .port(self.pgport)
            .database(&self.pgdatabase)
            .ssl_mode(ssl_mode)
    }
}
