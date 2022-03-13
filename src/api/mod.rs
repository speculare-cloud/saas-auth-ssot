use serde::{Deserialize, Serialize};

pub mod sso;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailSso {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub jwt: String,
}
