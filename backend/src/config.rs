use serde::Deserialize;
use std::process;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub audd_io_token: String
}

pub fn get_config() -> Config {
    envy::from_env().unwrap_or_else(|err| {
        log::error!("Failed to read valid configuration from environment variables: {}", err);
        process::exit(exitcode::CONFIG);
    })
}
