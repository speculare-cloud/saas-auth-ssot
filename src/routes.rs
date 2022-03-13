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
                .route("/sso", web::post().to(api::sso::handle_sso))
                .route("/rsso", web::post().to(api::sso::handle_rsso))
                .route("/csso", web::get().to(api::sso::handle_csso)),
        );
}
