#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate sproot;

use crate::utils::config::Config;

use clap::Parser;
use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel_migrations::EmbeddedMigrations;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use sproot::prog;

mod api;
mod flow_run;
mod routes;
mod server;
mod utils;

// Helper types for less boilerplate code
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(short = 'c', long = "config")]
    config_path: Option<String>,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

// Lazy static of the Config which is loaded from the config file
static CONFIG: Lazy<Config> = Lazy::new(|| match Config::new() {
    Ok(config) => config,
    Err(e) => {
        error!("Cannot build the Config: {}", e);
        std::process::exit(1);
    }
});

static JWT_ENCODINGKEY: Lazy<EncodingKey> = Lazy::new(|| {
    let content = std::fs::read(&CONFIG.jwt_ec_priv).unwrap();
    let secret = String::from_utf8_lossy(&content);
    EncodingKey::from_ec_pem(secret.as_bytes()).unwrap()
});

static JWT_DECODINGKEY: Lazy<DecodingKey> = Lazy::new(|| {
    let content = std::fs::read(&CONFIG.jwt_ec_pub).unwrap();
    let secret = String::from_utf8_lossy(&content);
    DecodingKey::from_ec_pem(secret.as_bytes()).unwrap()
});

// Embed migrations into the binary
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Define log level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{}={level},actix_web={level},sproot={level}",
                &prog().map_or_else(|| "saas_auth_ssot".to_owned(), |f| f.replace('-', "_")),
                level = args.verbose.log_level_filter()
            ),
        )
    }

    // Init logger/tracing
    tracing_subscriber::fmt::init();

    flow_run::flow_run_start().await
}
