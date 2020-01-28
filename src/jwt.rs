use crate::config::Config;
use crate::db::models::users::User;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::errors::{Error as JWT_Error, ErrorKind as JWT_ErrorKind};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fmt::{Display, Formatter, Result as ResultFormat};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub enum JWTError {
    TokenInvalid,
    TokenExpired,
}

impl From<jsonwebtoken::errors::Error> for JWTError {
    fn from(outer: JWT_Error) -> Self {
        match outer.kind() {
            JWT_ErrorKind::ExpiredSignature => JWTError::TokenExpired,
            _ => JWTError::TokenInvalid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: i32,
    email: String,
    expire: NaiveDateTime,
    exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut Formatter) -> ResultFormat {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

pub fn encode_token(config: &Config, user: &User) -> Result<String, JWTError> {
    let now = Utc::now();
    let expiration_hours = config.jwt_expiration_hours();
    let secret = config.jwt_secret_key();
    let my_claims = Claims {
        user_id: 1,
        email: user.email.clone(),
        expire: now
            .checked_add_signed(Duration::hours(expiration_hours as i64))
            .unwrap()
            .naive_utc(),
        exp: now
            .checked_add_signed(Duration::hours(expiration_hours as i64))
            .unwrap()
            .timestamp() as usize,
    };

    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("v1".to_owned());
    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    debug!("{}", &token);

    Ok(token)
}

pub fn decode_token(config: &Config, token: &str) -> Result<TokenData<Claims>, JWTError> {
    let secret = config.jwt_secret_key();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    )?;
    debug!("{:?}", &token_data);

    Ok(token_data)
}
