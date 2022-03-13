#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use std::{ffi::OsStr, path::Path};

use crate::utils::config::Config;

use clap::Parser;
use clap_verbosity_flag::WarnLevel;
use diesel::{r2d2::ConnectionManager, PgConnection};

mod flow_run;
mod routes;
mod server;
mod utils;

// Helper types for less boilerplate code
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type ConnType = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Evaluate an Enum into the value it hold
#[macro_export]
macro_rules! field_isset {
    ($value:expr, $name:literal) => {
        match $value {
            Some(x) => x,
            None => {
                error!(
                    "Config: optional field {} is not defined but is needed.",
                    $name
                );
                std::process::exit(1);
            }
        }
    };
}

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
}

// Embed migrations into the binary
embed_migrations!();

fn prog() -> Option<String> {
    std::env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Init logger
    env_logger::Builder::new()
        .filter_module(
            &prog().map_or_else(|| "speculare_server".to_owned(), |f| f.replace('-', "_")),
            args.verbose.log_level_filter(),
        )
        .init();

    flow_run::flow_run_start().await
}
