use configuration::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    // sql: SQLConfig,
}

impl AppConfig {
    pub fn new() -> Result<AppConfig, ConfigError> {
        let mut c = Config::default();

        c.merge(File::with_name("config")).unwrap();
        c.merge(Environment::new()).unwrap();

        c.try_into()
    }
}

// #[derive(Debug, Serialize)]
// pub struct SQLConfig {
//     pub host: String,
//     pub port: u16,
//     pub username: String,
//     pub password: String,
//     pub database: String,
// }
