use crate::{
    api::{EmailSso, JwtToken},
    utils::jwt,
    Pool,
};

use actix_session::Session;
use actix_web::{web, HttpResponse};
use sproot::errors::{AppError, AppErrorType};

/// POST /api/sso
pub async fn handle_sso(
    db: web::Data<Pool>,
    wemail: web::Json<EmailSso>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/sso");

    let email = wemail.into_inner().email;

    Ok(HttpResponse::Ok().finish())
}

/// POST /api/rsso
pub async fn handle_rsso(
    db: web::Data<Pool>,
    wemail: web::Json<EmailSso>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/rsso");

    let email = wemail.into_inner().email;

    Ok(HttpResponse::Ok().finish())
}

/// GET /api/csso
/// Exchange the code from the callback for a CookieSession
pub async fn handle_csso(
    db: web::Data<Pool>,
    session: Session,
    jwt_holder: web::Query<JwtToken>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/csso");

    // Check if the user is already "logged" (don't override a user_id)
    if let Some(user_id) = session.get::<String>("user_id")? {
        return Err(AppError {
            message: Some(format!("You're already logged as {}", user_id)),
            cause: None,
            error_type: AppErrorType::InvalidRequest,
        });
    }

    // Get the customer_id from the jwt token
    let customer_id = match base64::decode(&jwt_holder.jwt) {
        Ok(decoded) => jwt::decode_jwt(std::str::from_utf8(&decoded).unwrap())?,
        Err(e) => {
            return Err(AppError {
                message: Some("Cannot decode your base64 encoded JWT".into()),
                cause: Some(format!("DecodeError: {:?}", e)),
                error_type: AppErrorType::InvalidRequest,
            });
        }
    };

    // Check if the customer_id exists in the database

    // If everything is correct, return a Cookie with the user_id == customer_id
    session.insert("user_id", customer_id)?;
    Ok(HttpResponse::Ok().finish())
}
