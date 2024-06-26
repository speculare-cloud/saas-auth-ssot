use actix_session::Session;
use actix_web::{web, HttpResponse};
use base64::Engine;
use sproot::{
    apierrors::ApiError,
    models::{AuthPool, Customers, CustomersDTO, DtoBase},
};
use uuid::Uuid;

use crate::{
    api::{exit_if_logged, extract_mailbox, EmailSso, JwtToken},
    utils::{jwt, mail_sso::send_sso_mail},
};

/// POST /api/sso
///
/// Login a customer (get a Magic Link Mail)
pub async fn handle_sso(
    db: web::Data<AuthPool>,
    session: Session,
    wemail: web::Json<EmailSso>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/sso");

    exit_if_logged(&session)?;

    web::block(move || {
        let (email, mailboxed) = extract_mailbox(wemail.into_inner())?;
        // Get the customer_id from the email
        let customer = Customers::get_specific(&mut db.pool.get()?, &email)?;
        // Create the JWT token
        let jwt = jwt::create_jwt(&customer.id.to_string())?;
        // Encode it in base64 for convenience
        let encoded = base64::prelude::BASE64_STANDARD.encode(jwt);
        // Send the mail for the JWT token
        // at first, send it immediately, but then we can consider
        // creating a Queue for the mails to be sent (avoid limit, ...)
        send_sso_mail(mailboxed, &encoded)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

/// POST /api/rsso
///
/// Create a new customer based on his email address
/// and then get a Magic Link Mail
pub async fn handle_rsso(
    db: web::Data<AuthPool>,
    session: Session,
    wemail: web::Json<EmailSso>,
) -> Result<HttpResponse, ApiError> {
    info!("Route POST /api/rsso");

    exit_if_logged(&session)?;

    web::block(move || {
        let (email, mailboxed) = extract_mailbox(wemail.into_inner())?;
        // Create the user and generate a customer_id (auto in Postgres)
        let customer =
            Customers::insert_and_get(&mut db.pool.get()?, &CustomersDTO { email: &email })?;
        // Create the JWT token
        let jwt = jwt::create_jwt(&customer.id.to_string())?;
        // Encode it in base64 for convenience
        let encoded = base64::prelude::BASE64_STANDARD.encode(jwt);
        // Send the mail for the JWT token
        // at first, send it immediately, but then we can consider
        // creating a Queue for the mails to be sent (avoid limit, ...)
        send_sso_mail(mailboxed, &encoded)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

/// GET /api/csso
///
/// Exchange the code from the callback for a CookieSession
/// eg: http://xyz/api/csso?jwt=base64_jwttoken
pub async fn handle_csso(
    db: web::Data<AuthPool>,
    session: Session,
    jwt_holder: web::Query<JwtToken>,
) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/csso");

    exit_if_logged(&session)?;

    let customer_id = web::block(move || {
        // Get the customer_id from the jwt token
        let customer_id = match base64::prelude::BASE64_STANDARD.decode(&jwt_holder.jwt) {
            Ok(decoded) => jwt::decode_jwt(std::str::from_utf8(&decoded).unwrap())?,
            Err(_) => return Err(ApiError::AuthorizationError(None)),
        };

        // Check if the customer_id exists in the database
        if !Customers::exists(&mut db.pool.get()?, &Uuid::parse_str(&customer_id)?)? {
            return Err(ApiError::AuthorizationError(None));
        }

        Ok(customer_id)
    })
    .await??;

    // If everything is correct, return a Cookie with the user_id == customer_id
    session.insert("user_id", customer_id.clone())?;
    Ok(HttpResponse::Ok().body(customer_id))
}

/// Simple route that check if the user is logged
pub async fn handle_who(session: Session) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/whoami");

    // If there's no user_id in the session, it's not logged
    if session.get::<String>("user_id")?.is_none() {
        return Err(ApiError::AuthorizationError(None));
    }

    Ok(HttpResponse::Ok().body(session.get::<String>("user_id")?.unwrap()))
}

/// Clear the Session on client & server side
pub async fn handle_logout(session: Session) -> Result<HttpResponse, ApiError> {
    info!("Route GET /api/logout");

    session.purge();

    Ok(HttpResponse::Ok().finish())
}
