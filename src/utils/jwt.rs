use crate::{JWT_DECODINGKEY, JWT_ENCODINGKEY};

use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};
use sproot::apierrors::ApiError;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(customer_id: &str) -> Result<String, ApiError> {
    let expiration = match Utc::now().checked_add_signed(chrono::Duration::minutes(5)) {
        Some(time) => time.timestamp(),
        None => {
            return Err(ApiError::ServerError(String::from(
                "cannot build expiration time",
            )));
        }
    };

    let claims = Claims {
        sub: customer_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::new(Algorithm::ES256), &claims, &JWT_ENCODINGKEY).map_err(|err| {
        trace!("jwt encode error: {}", err);
        ApiError::ServerError(String::from("failed to encode your JWT"))
    })
}

pub fn decode_jwt(jwt: &str) -> Result<String, ApiError> {
    let decoded = decode::<Claims>(jwt, &JWT_DECODINGKEY, &Validation::new(Algorithm::ES256))
        .map_err(|err| {
            trace!("jwt decode error: {}", err);
            ApiError::AuthorizationError(String::from("failed to decode the JWT"))
        })?;

    Ok(decoded.claims.sub)
}
