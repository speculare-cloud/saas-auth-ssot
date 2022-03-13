use std::{fs::File, io::BufReader};

use crate::{field_isset, routes, Pool, CONFIG};

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use rustls::{Certificate, PrivateKey, ServerConfig};

/// Return the ServerConfig needed for Actix to be binded on HTTPS
///
/// Use key and cert for the path to find the files.
fn get_ssl_builder(key: &str, cert: &str) -> ServerConfig {
    // Open BufReader on the key and cert files to read their content
    let cert_file = &mut BufReader::new(
        File::open(cert).unwrap_or_else(|_| panic!("Certificate file not found at {}", cert)),
    );
    let key_file = &mut BufReader::new(
        File::open(key).unwrap_or_else(|_| panic!("Key file not found at {}", key)),
    );
    // Create a Vec of certificate by extracting all cert from cert_file
    let cert_chain = rustls_pemfile::certs(cert_file)
        .unwrap()
        .iter()
        .map(|v| Certificate(v.clone()))
        .collect();
    // Extract all PKCS8-encoded private key from key_file and generate a Vec from them
    let mut keys = rustls_pemfile::pkcs8_private_keys(key_file).unwrap();
    // If no keys are found, we try using the rsa type
    if keys.is_empty() {
        // Reopen a new BufReader as pkcs8_private_keys took over the previous one
        let key_file = &mut BufReader::new(
            File::open(&key).unwrap_or_else(|_| panic!("Key file not found at {}", key)),
        );
        keys = rustls_pemfile::rsa_private_keys(key_file).unwrap();
    }
    // Convert the first key to be a PrivateKey
    let key: PrivateKey = PrivateKey(keys.remove(0));

    // Return the ServerConfig to be used
    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("bad certificate/key")
}

/// Construct and run the actix server instance
///
/// Start by initializating a link to the database. And finish by binding and running the actix serv
pub async fn server(pool: Pool) -> std::io::Result<()> {
    // Construct the HttpServer instance.
    // Passing the pool of PgConnection and defining the logger / compress middleware.
    let serv = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(pool.clone())
            .configure(routes::routes)
    })
    .workers(CONFIG.workers);
    // Bind the server (https or no)
    if !CONFIG.https {
        if !cfg!(debug_assertions) {
            warn!("You're starting speculare-server as HTTP on a production build, are you sure about what you're doing ?")
        } else {
            info!("Server started as HTTP on {}", &CONFIG.binding);
        }
        serv.bind(&CONFIG.binding)?.run().await
    } else {
        info!("Server started as HTTPS on {}", &CONFIG.binding);
        let key_priv = field_isset!(CONFIG.key_priv.as_ref(), "key_priv");
        let key_cert = field_isset!(CONFIG.key_cert.as_ref(), "key_cert");
        serv.bind_rustls(&CONFIG.binding, get_ssl_builder(key_priv, key_cert))?
            .run()
            .await
    }
}
