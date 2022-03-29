use crate::{
    api::{apikey, sso},
    CONFIG,
};

use actix_web::{guard, web};
use sproot::{check_sessions::CheckSessions, get_session_middleware};

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        .service(
            web::resource("/api/key")
                .guard(guard::Patch())
                .route(web::patch().to(apikey::update_apikey)),
        )
        .service(
            web::scope("/api")
                .wrap(CheckSessions)
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                ))
                .route("/sso", web::post().to(sso::handle_sso))
                .route("/rsso", web::post().to(sso::handle_rsso))
                .route("/csso", web::get().to(sso::handle_csso))
                .route("/key", web::post().to(apikey::post_apikey))
                .route("/key/{key}", web::delete().to(apikey::delete_apikey)),
        );
}
