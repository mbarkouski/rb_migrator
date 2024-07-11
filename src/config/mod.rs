use std::{path::PathBuf, process::exit};

use confique::Config;

#[derive(Config, Debug)]
pub struct ClickHouseCfg {
    pub host: String,
    pub port: u16,
    pub user_name: String,
    pub db: String,
    pub password: String,
    pub cert_path: Option<String>,
}

impl ClickHouseCfg {
    pub fn new(path: PathBuf) -> Self {
         ClickHouseCfg::from_file(path).unwrap_or_else(|err| {
            log::error!("Read config for connection to ClickHouse Error: {:?}", err);
            exit(1);
        })
    }
}
