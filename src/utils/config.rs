use crate::Args;

use clap::Parser;
use config::ConfigError;
use lettre::message::Mailbox;
use serde::{de, Deserialize, Deserializer};

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
    pub sso_base_url: String,
    pub jwt_ec_priv: String,
    pub jwt_ec_pub: String,

    // SMTP SETTINGS
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    #[serde(default = "default_smtp_tls")]
    pub smtp_tls: bool,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_password: String,
    #[serde(deserialize_with = "mailbox_deser")]
    pub smtp_email_sender: Mailbox,
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

        let config: Result<Self, ConfigError> = config_builder.build()?.try_deserialize();

        // Assert that the config is correct
        if let Ok(ref config) = config {
            if config.key_priv.is_none() || config.key_cert.is_none() {
                error!(
                    "error: config: 'https' is true but no 'key_priv' and/or 'key_cert' defined"
                );
                std::process::exit(1);
            }
        }

        config
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

fn default_smtp_port() -> u16 {
    587
}

fn default_smtp_tls() -> bool {
    true
}

fn mailbox_deser<'de, D>(data: D) -> Result<Mailbox, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(data)?;
    match s.parse() {
        Ok(res) => Ok(res),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Mailbox error for \"{}\": {}", s, e),
        )),
    }
    .map_err(de::Error::custom)
}
