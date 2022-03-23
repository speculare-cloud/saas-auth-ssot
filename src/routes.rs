use crate::{api, CONFIG};

use actix_web::web;
use sproot::get_session_middleware;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                .route("/key", web::patch().to(api::apikey::update_apikey))
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                ))
                .route("/sso", web::post().to(api::sso::handle_sso))
                .route("/rsso", web::post().to(api::sso::handle_rsso))
                .route("/csso", web::get().to(api::sso::handle_csso)),
        );
}
