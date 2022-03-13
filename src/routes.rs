use crate::{api, CONFIG};

use actix_session::CookieSession;
use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                .wrap(CookieSession::signed(CONFIG.cookie_secret.as_bytes()).name("SP-CKS"))
                .route("/login", web::post().to(api::login::handle_login))
                .route("/register", web::post().to(api::register::handle_register))
                .route("/callback", web::get().to(api::callback::handle_callback)),
        );
}
