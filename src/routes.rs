use actix_web::{guard, web};
use sproot::get_session_middleware;

use crate::{
    api::{apikey, sso},
    CONFIG,
};

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        .service(
            web::scope("/api")
                .guard(guard::Patch())
                .route("/key", web::patch().to(apikey::update_apikey)),
        )
        .service(
            web::scope("/api")
                .wrap(get_session_middleware(
                    CONFIG.cookie_secret.as_bytes(),
                    "SP-CKS".to_string(),
                    CONFIG.cookie_domain.to_owned(),
                ))
                .route("/sso", web::post().to(sso::handle_sso))
                .route("/rsso", web::post().to(sso::handle_rsso))
                .route("/csso", web::get().to(sso::handle_csso))
                .route("/whoami", web::get().to(sso::handle_who))
                .route("/logout", web::get().to(sso::handle_logout))
                .route("/key", web::get().to(apikey::get_apikey))
                .route("/key", web::post().to(apikey::post_apikey))
                .route("/key", web::delete().to(apikey::delete_apikey)),
        );
}
