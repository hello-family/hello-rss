use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::{config::APP_CONFIG, error::Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i32) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);

        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub async fn sign(id: i32) -> Result<String> {
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(APP_CONFIG.jwt_secret.as_bytes()),
    )?)
}

pub async fn verify(token: &str) -> Result<Claims> {
    Ok(jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(APP_CONFIG.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
