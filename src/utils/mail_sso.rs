use crate::CONFIG;

use askama::Template;
use chrono::Utc;
use lettre::message::{Mailbox, MultiPart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::transport::smtp::PoolConfig;
use lettre::SmtpTransport;
use lettre::{
    message::{header, SinglePart},
    Message, Transport,
};
use sproot::errors::{AppError, AppErrorType};

pub fn test_smtp_transport() {
    // Check if the SMTP server host is "ok"
    match MAILER.test_connection() {
        Ok(result) => {
            info!("MAILER: No fatal error, connect is: {}", result);
        }
        Err(e) => {
            error!("MAILER: test of the smtp_transport failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_smtp_transport() -> Result<SmtpTransport, lettre::transport::smtp::Error> {
    let creds = Credentials::new(CONFIG.smtp_user.to_owned(), CONFIG.smtp_password.to_owned());

    let transport = if CONFIG.smtp_tls {
        SmtpTransport::builder_dangerous(&CONFIG.smtp_host).tls(Tls::Required(TlsParameters::new(
            (&CONFIG.smtp_host).to_owned(),
        )?))
    } else {
        SmtpTransport::builder_dangerous(&CONFIG.smtp_host)
    };

    // Open a remote connection to gmail
    Ok(transport
        .port(CONFIG.smtp_port)
        .credentials(creds)
        .pool_config(PoolConfig::new().max_size(16))
        .build())
}

lazy_static::lazy_static! {
    // Lazy static for SmtpTransport used to send mails
    // Build it using rustls and a pool of 16 items.
    static ref MAILER: SmtpTransport = {
        match get_smtp_transport() {
            Ok(smtp) => smtp,
            Err(e) => {
                error!("MAILER: cannot get the smtp_transport: {}", e);
                std::process::exit(1);
            }
        }
    };
}

fn send_mail(email_addr: Mailbox, template: String, jwt: &str) -> Result<(), AppError> {
    // Build the email with all params
    let email = Message::builder()
        // Sender is the email of the sender, which is used by the SMTP
        // if the sender is not equals to the smtp server account, the mail will ends in the spam.
        .from(CONFIG.smtp_email_sender.to_owned())
        // Receiver is the person who should get the email
        .to(email_addr)
        .subject(format!("Speculare - Authentication Requested at {} (utc)", Utc::now().format("%H:%M:%S")))
        .multipart(
            // Use multipart to have a fallback
        MultiPart::alternative()
                // This singlepart is the fallback for the html code
                // ==> Very basic.
                .singlepart(
                    SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(format!("Speculare - Passwordless Authentication. Use the following link to sign in on Speculare: {}/csso?jwt={}", CONFIG.sso_base_url, jwt))
                )
                // This singlepart is the html design with all fields replaced
                // ==> Prettier, ...
                .singlepart(
                    SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(template)
                )
    ).unwrap();

    // Send the email
    match MAILER.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Could not send email: {}", e);
            Err(AppError {
                message: format!("Error while sending verification email: {}", e),
                error_type: AppErrorType::ServerError,
            })
        }
    }
}

/// Structure representing the incident template html sent by mail
#[derive(Template)]
#[template(path = "sso.html")]
struct SsoTemplate<'a> {
    sso_base: &'a str,
    jwt: &'a str,
}

/// Send an email alerting that a new incident was created.
pub fn send_sso_mail(email: Mailbox, jwt: &str) -> Result<(), AppError> {
    // Build the SsoTemplate (html code)
    // The SsoTemplate struct is used to hold all the information
    // about the template, which values are needed, ...
    let sso_template = SsoTemplate {
        sso_base: &CONFIG.sso_base_url,
        jwt,
    }
    .render()?;

    send_mail(email, sso_template, jwt)
}
