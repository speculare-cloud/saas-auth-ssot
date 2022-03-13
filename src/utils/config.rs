use crate::Args;

use clap::Parser;
use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]

pub struct Config {
    // POSTGRESQL CONNECTION
    pub database_url: String,
    #[serde(default = "default_maxconn")]
    pub database_max_connection: u32,

    // API SETTINGS
    pub binding: String,
    #[serde(default = "default_workers")]
    pub workers: usize,

    // API SECURITY SETTINGS
    #[serde(default = "default_https")]
    pub https: bool,
    pub key_priv: Option<String>,
    pub key_cert: Option<String>,

    pub cookie_secret: String,
    pub cookie_domain: String,

    // AUTH SETTINGS
    pub jwt_ec_priv: String,
    pub jwt_ec_pub: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let args = Args::parse();

        let config_builder = config::Config::builder().add_source(config::File::new(
            &args
                .config_path
                .unwrap_or_else(|| "/etc/speculare/ssot.config".to_owned()),
            config::FileFormat::Toml,
        ));

        config_builder.build()?.try_deserialize()
    }
}

fn default_https() -> bool {
    false
}

fn default_maxconn() -> u32 {
    10
}

fn default_workers() -> usize {
    match sys_metrics::cpu::get_logical_count() {
        Ok(count) => count as usize,
        Err(e) => {
            error!(
                "Workers: failed to get the number of workers automatically, defaulting to 4: {}",
                e
            );
            4
        }
    }
}
