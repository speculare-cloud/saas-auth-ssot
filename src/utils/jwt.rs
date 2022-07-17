use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use sproot::apierrors::ApiError;

use crate::{JWT_DECODINGKEY, JWT_ENCODINGKEY};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(customer_id: &str) -> Result<String, ApiError> {
    let expiration = match Utc::now().checked_add_signed(chrono::Duration::minutes(5)) {
        Some(time) => time.timestamp(),
        None => {
            return Err(ApiError::ServerError(None));
        }
    };

    let claims = Claims {
        sub: customer_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::new(Algorithm::ES256), &claims, &JWT_ENCODINGKEY).map_err(|err| {
        trace!("jwt encode error: {}", err);
        ApiError::ServerError(None)
    })
}

pub fn decode_jwt(jwt: &str) -> Result<String, ApiError> {
    let decoded = decode::<Claims>(jwt, &JWT_DECODINGKEY, &Validation::new(Algorithm::ES256))
        .map_err(|err| {
            trace!("jwt decode error: {}", err);
            ApiError::AuthorizationError(None)
        })?;

    Ok(decoded.claims.sub)
}
