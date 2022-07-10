use actix_session::Session;
use actix_web::{http::header::HeaderValue, HttpRequest};
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};
use sproot::apierrors::ApiError;
use uuid::Uuid;

pub mod apikey;
pub mod sso;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailSso {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specific {
    pub uuid: String,
}

/// Return the HeaderValue of the header 'header_name'
/// or return an ApiError - InvalidRequest if not present
pub fn get_header_value(req: &HttpRequest, header_name: &str) -> Result<HeaderValue, ApiError> {
    match req.headers().get(header_name) {
        Some(sptk) => Ok(sptk.to_owned()),
        None => Err(ApiError::InvalidRequestError(None)),
    }
}

/// Get the Uuid of the user from his Session or
/// return an InvalidToken error if not found
pub fn get_user_session(session: &Session) -> Result<Uuid, ApiError> {
    match session.get::<String>("user_id") {
        Ok(Some(id)) => Ok(Uuid::parse_str(&id).unwrap()),
        _ => Err(ApiError::SessionError(None)),
    }
}

/// Simply return an error if the user is already logged.
/// Used to protect the login route (sso)
pub fn exit_if_logged(session: &Session) -> Result<(), ApiError> {
    // Check if the user is already "logged" (don't override a user_id)
    if (session.get::<String>("user_id")?).is_some() {
        Err(ApiError::InvalidRequestError(None))
    } else {
        Ok(())
    }
}

/// Return the plain text email and the Mailbox object
/// from the EmailSso or return an error if the email
/// is not correctly formatted.
pub fn extract_mailbox(wemail: EmailSso) -> Result<(String, Mailbox), ApiError> {
    // This act as a email verification (Regex is used)
    let mailboxed: Mailbox = match wemail.email.parse() {
        Ok(recv) => recv,
        Err(_) => {
            return Err(ApiError::InvalidRequestError(None));
        }
    };

    Ok((wemail.email, mailboxed))
}
