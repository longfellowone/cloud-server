use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub server: ServerConfiguration,
    pub database: PostgresConfiguration,
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
pub struct ServerConfiguration {
    host: String,
    port: u16,
}

impl ServerConfiguration {
    pub fn addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfiguration {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl PostgresConfiguration {
    pub fn connection_string(self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }
}
