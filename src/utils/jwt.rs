use crate::{JWT_DECODINGKEY, JWT_ENCODINGKEY};

use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use sproot::errors::{AppError, AppErrorType};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(customer_id: &str) -> Result<String, AppError> {
    let expiration = match Utc::now().checked_add_signed(chrono::Duration::minutes(5)) {
        Some(time) => time.timestamp(),
        None => {
            return Err(AppError {
                message: "Cannot build expiration time".to_owned(),
                error_type: AppErrorType::ServerError,
            });
        }
    };

    let claims = Claims {
        sub: customer_id.to_owned(),
        exp: expiration as usize,
    };

    Ok(encode(
        &Header::new(Algorithm::ES256),
        &claims,
        &JWT_ENCODINGKEY,
    )?)
}

pub fn decode_jwt(jwt: &str) -> Result<String, AppError> {
    let decoded = decode::<Claims>(jwt, &JWT_DECODINGKEY, &Validation::new(Algorithm::ES256))?;

    Ok(decoded.claims.sub)
}
