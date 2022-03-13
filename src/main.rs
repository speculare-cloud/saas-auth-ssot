#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate sproot;

use crate::utils::config::Config;

use clap::Parser;
use clap_verbosity_flag::WarnLevel;
use diesel::{r2d2::ConnectionManager, PgConnection};
use jsonwebtoken::{DecodingKey, EncodingKey};

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
    verbose: clap_verbosity_flag::Verbosity<WarnLevel>,
}

// Lazy static of the Token from Config to use in validator
lazy_static::lazy_static! {
    // Lazy static of the Config which is loaded from the config file
    static ref CONFIG: Config = match Config::new() {
        Ok(config) => config,
        Err(e) => {
            error!("Cannot build the Config: {}", e);
            std::process::exit(1);
        }
    };

    static ref JWT_ENCODINGKEY: EncodingKey = {
        let content = std::fs::read(&CONFIG.jwt_ec_priv).unwrap();
        let secret = String::from_utf8_lossy(&content);
        EncodingKey::from_ec_pem(secret.as_bytes()).unwrap()
    };

    static ref JWT_DECODINGKEY: DecodingKey = {
        let content = std::fs::read(&CONFIG.jwt_ec_pub).unwrap();
        let secret = String::from_utf8_lossy(&content);
        DecodingKey::from_ec_pem(secret.as_bytes()).unwrap()
    };
}

// Embed migrations into the binary
embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Init logger
    env_logger::Builder::new()
        .filter_module(
            &sproot::prog().map_or_else(|| "speculare_server".to_owned(), |f| f.replace('-', "_")),
            args.verbose.log_level_filter(),
        )
        .filter_module("actix_web", args.verbose.log_level_filter())
        .init();

    let jwt = utils::jwt::create_jwt("customer_id");
    info!("Plain JWT {:?}", jwt);
    let base64_jwt = base64::encode(jwt.unwrap());
    info!("base64 JWT {:?}", base64_jwt);

    flow_run::flow_run_start().await
}
