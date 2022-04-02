use actix_session::Session;
use actix_web::{http::header::HeaderValue, HttpRequest};
use lettre::message::Mailbox;
use serde::{Deserialize, Serialize};
use sproot::errors::{AppError, AppErrorType};
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

pub fn get_header_value(req: &HttpRequest, header_name: &str) -> Result<HeaderValue, AppError> {
    match req.headers().get(header_name) {
        Some(sptk) => Ok(sptk.to_owned()),
        None => Err(AppError {
            message: format!("No {} in the headers", header_name),
            error_type: AppErrorType::InvalidRequest,
        }),
    }
}

pub fn get_user_session(session: &Session) -> Result<Uuid, AppError> {
    match session.get::<String>("user_id") {
        Ok(Some(id)) => Ok(Uuid::parse_str(&id).unwrap()),
        _ => Err(AppError {
            message: "Missing user_id in the session".to_owned(),
            error_type: AppErrorType::InvalidRequest,
        }),
    }
}

pub fn exit_if_logged(session: &Session) -> Result<(), AppError> {
    // Check if the user is already "logged" (don't override a user_id)
    if let Some(user_id) = session.get::<String>("user_id")? {
        return Err(AppError {
            message: format!("You're already logged as {}", user_id),
            error_type: AppErrorType::InvalidRequest,
        });
    }

    Ok(())
}

pub fn extract_mailbox(wemail: EmailSso) -> Result<(String, Mailbox), AppError> {
    // This act as a email verification (Regex is used)
    let mailboxed: Mailbox = match wemail.email.parse() {
        Ok(recv) => recv,
        Err(e) => {
            error!("Cannot convert {} into a Mailbox: {}", wemail.email, e);
            return Err(AppError {
                message: "Bad email address".to_owned(),
                error_type: AppErrorType::InvalidRequest,
            });
        }
    };

    Ok((wemail.email, mailboxed))
}
