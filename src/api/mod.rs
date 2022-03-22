use serde::{Deserialize, Serialize};

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
