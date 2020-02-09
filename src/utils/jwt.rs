use crate::config::Config;
use crate::db::models::users::User;
use crate::errors::ErrorResponse;
use actix_web::dev::Payload;
use actix_web::HttpRequest;
use chrono::{Duration, NaiveDateTime, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::errors::{Error as JWT_Error, ErrorKind as JWT_ErrorKind};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fmt::{Display, Formatter, Result as ResultFormat};

#[derive(Debug, Serialize, Deserialize)]
pub enum JWTError {
    TokenInvalid,
    TokenExpired,
    TokenParsingError,
}

impl Display for JWTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_string = match self {
            JWTError::TokenInvalid => "Token is invalid",
            JWTError::TokenExpired => "Token is expired",
            _ => "Error parsing token",
        };
        write!(f, "{}", error_string)
    }
}

impl From<jsonwebtoken::errors::Error> for JWTError {
    fn from(outer: JWT_Error) -> Self {
        match outer.kind() {
            JWT_ErrorKind::ExpiredSignature => JWTError::TokenExpired,
            _ => JWTError::TokenInvalid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    user_id: i32,
    email: String,
    expire: NaiveDateTime,
    exp: usize,
}

impl Claims {
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut Formatter) -> ResultFormat {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl actix_web::FromRequest for Claims {
    // The associated error which can be returned.
    type Error = ErrorResponse;

    // Future that resolves to a Self
    type Future = Ready<Result<Claims, Self::Error>>;

    // Configuration for this extractor
    type Config = ();

    // Convert request to a Self
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let qp = extensions.get::<Claims>();
        match qp {
            Some(q_param) => ok(q_param.clone()),
            _ => err(ErrorResponse {
                msg: "Error in authorization".to_string(),
                status: 403,
            }),
        }
    }

    // Convert request to a Self
    //
    // This method uses `Payload::None` as payload stream.
    //    fn extract(req: &HttpRequest) -> Self::Future {
    //        Self::from_request(req, &mut Payload::None)
    //    }
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

    Ok(token)
}

pub fn decode_token(config: &Config, token: &str) -> Result<TokenData<Claims>, JWTError> {
    let secret = config.jwt_secret_key();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    )?;

    Ok(token_data)
}
