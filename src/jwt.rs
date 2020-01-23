use crate::config::Config;
use crate::db::models::users::User;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::errors::{Error as JWT_Error, ErrorKind as JWT_ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub enum JWTError {
    TokenInvalid,
    TokenExpired,
}

impl From<jsonwebtoken::errors::Error> for JWTError {
    fn from(outer: JWT_Error) -> Self {
        match outer.kind() {
            _ => JWTError::TokenInvalid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i32,
    expire: NaiveDateTime,
}

pub fn encode_token(config: &Config, user: &User) -> Result<String, JWTError> {
    let now = Utc::now();
    let expiration_hours = config.jwt_expiration_hours();
    let secret = config.jwt_secret_key();
    let my_claims = Claims {
        user_id: 1,
        expire: now
            .checked_add_signed(Duration::hours(expiration_hours as i64))
            .unwrap()
            .naive_utc(),
    };
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("v1".to_owned());
    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn decode_token(config: &Config) -> Result<Claims, JWTError> {
    let now = Utc::now();

    Ok(Claims {
        user_id: 1,
        expire: now
            .checked_add_signed(Duration::hours(5))
            .unwrap()
            .naive_utc(),
    })
}
