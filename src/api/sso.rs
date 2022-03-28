use crate::{
    api::{EmailSso, JwtToken},
    utils::{jwt, mail_sso::send_sso_mail},
};

use actix_session::Session;
use actix_web::{web, HttpResponse};
use lettre::message::Mailbox;
use sproot::{
    errors::{AppError, AppErrorType},
    models::{AuthPool, Customers, CustomersDTO},
};
use uuid::Uuid;

fn exit_if_logged(session: &Session) -> Result<(), AppError> {
    // Check if the user is already "logged" (don't override a user_id)
    if let Some(user_id) = session.get::<String>("user_id")? {
        return Err(AppError {
            message: format!("You're already logged as {}", user_id),
            error_type: AppErrorType::InvalidRequest,
        });
    }

    Ok(())
}

fn extract_email(wemail: EmailSso) -> Result<(String, Mailbox), AppError> {
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

/// POST /api/sso (login)
pub async fn handle_sso(
    db: web::Data<AuthPool>,
    session: Session,
    wemail: web::Json<EmailSso>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/sso");

    exit_if_logged(&session)?;

    web::block(move || {
        let (email, mailboxed) = extract_email(wemail.into_inner())?;
        // Get the customer_id from the email
        let customer = Customers::get(&db.pool.get()?, &email)?;
        // Create the JWT token
        let jwt = jwt::create_jwt(&customer.id.to_string())?;
        // Encode it in base64 for convenience
        let encoded = base64::encode(jwt);
        // Send the mail for the JWT token
        // at first, send it immediately, but then we can consider
        // creating a Queue for the mails to be sent (avoid limit, ...)
        send_sso_mail(mailboxed, &encoded)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

/// POST /api/rsso (register)
pub async fn handle_rsso(
    db: web::Data<AuthPool>,
    session: Session,
    wemail: web::Json<EmailSso>,
) -> Result<HttpResponse, AppError> {
    info!("Route POST /api/rsso");

    exit_if_logged(&session)?;

    web::block(move || {
        let (email, mailboxed) = extract_email(wemail.into_inner())?;
        // Create the user and generate a customer_id
        let customer = CustomersDTO { email: &email }.ginsert(&db.pool.get()?)?;
        // Create the JWT token
        let jwt = jwt::create_jwt(&customer.id.to_string())?;
        // Encode it in base64 for convenience
        let encoded = base64::encode(jwt);
        // Send the mail for the JWT token
        // at first, send it immediately, but then we can consider
        // creating a Queue for the mails to be sent (avoid limit, ...)
        send_sso_mail(mailboxed, &encoded)
    })
    .await??;

    Ok(HttpResponse::Ok().finish())
}

/// GET /api/csso
/// Exchange the code from the callback for a CookieSession
pub async fn handle_csso(
    db: web::Data<AuthPool>,
    session: Session,
    jwt_holder: web::Query<JwtToken>,
) -> Result<HttpResponse, AppError> {
    info!("Route GET /api/csso");

    exit_if_logged(&session)?;

    let customer_id = web::block(move || {
        // Get the customer_id from the jwt token
        let customer_id = match base64::decode(&jwt_holder.jwt) {
            Ok(decoded) => jwt::decode_jwt(std::str::from_utf8(&decoded).unwrap())?,
            Err(_) => {
                return Err(AppError {
                    message: "Invalid JWT token, access denied".to_string(),
                    error_type: AppErrorType::InvalidRequest,
                });
            }
        };

        // Check if the customer_id exists in the database
        if !Customers::exists(&db.pool.get()?, &Uuid::parse_str(&customer_id)?)? {
            return Err(AppError {
                message: "Bad id, not authorized".to_owned(),
                error_type: AppErrorType::InvalidToken,
            });
        }

        Ok(customer_id)
    })
    .await??;

    // If everything is correct, return a Cookie with the user_id == customer_id
    session.insert("user_id", customer_id)?;
    Ok(HttpResponse::Ok().finish())
}
