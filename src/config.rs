use configuration::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database: PostgresConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();

        c.merge(File::with_name("config")).unwrap();
        c.merge(Environment::new()).unwrap();

        c.try_into()
    }

    pub fn addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl PostgresConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
