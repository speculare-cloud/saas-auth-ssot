use crate::{api, CONFIG};

use actix_session::{storage::CookieSessionStore, CookieContentSecurity, SessionMiddleware};
use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    let cookie_session = SessionMiddleware::builder(
        CookieSessionStore::default(),
        actix_web::cookie::Key::from(CONFIG.cookie_secret.as_bytes()),
    )
    .cookie_name("SP-CKS".to_string())
    .cookie_content_security(CookieContentSecurity::Signed)
    .build();

    // The /ping is used only to get a status over the server
    cfg.route("/ping", web::get().to(|| async { "zpour" }))
        .route("/ping", web::head().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                .wrap(cookie_session)
                .route("/sso", web::post().to(api::sso::handle_sso))
                .route("/rsso", web::post().to(api::sso::handle_rsso))
                .route("/csso", web::get().to(api::sso::handle_csso)),
        );
}
