use std::pin::Pin;
use std::task::{Context, Poll};

use crate::config::Config;
use crate::errors::ErrorResponse;
use crate::jwt::{decode_token, Claims, JWTError};
use crate::state::AppState;
use actix_service::{Service, Transform};
use actix_web::http::HeaderMap;
use actix_web::web::Data;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::Future;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthenticationService {}

impl Default for AuthenticationService {
    fn default() -> Self {
        Self {}
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for AuthenticationService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S> AuthenticationMiddleware<S> {
    fn get_header_parts<'a>(
        &self,
        headers: &'a HeaderMap,
    ) -> Result<Option<(&'a str, &'a str)>, JWTError> {
        if let Some(auth_header) = headers.get("Authorization".to_string()) {
            let parts: Vec<&str> = auth_header.to_str().unwrap_or("").split(" ").collect();
            if parts.len() == 2 {
                Ok(Some((parts[0], parts[1])))
            } else {
                error!("Unable parsing header 'Authorization': {:#?}", &parts);
                Err(JWTError::TokenParsingError)
            }
        } else {
            Ok(None)
        }
    }

    fn parse_claims(
        &self,
        config: &Config,
        _name: &str,
        raw_token: &str,
    ) -> Result<Claims, JWTError> {
        let token_data = decode_token(config, raw_token);
        match token_data {
            Ok(tdata) => Ok(tdata.claims),
            Err(err) => Err(err),
        }
    }

    fn get_claims(
        &self,
        headers: &HeaderMap,
        option_state: &Option<Data<AppState>>,
    ) -> Result<Option<Claims>, JWTError> {
        if let Some((name, token)) = self.get_header_parts(headers)? {
            let conf_from_file = Config::from_file();

            let config = match option_state {
                Some(app_state) => &app_state.config,
                None => &conf_from_file,
            };
            match self.parse_claims(&config, name, token) {
                Ok(claims) => Ok(Some(claims)),
                Err(err) => Err(err),
            }
        } else {
            Ok(None)
        }
    }
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        let app_state = &req.app_data::<AppState>();

        let clms = self.get_claims(headers, app_state);
        let mut result_error: Option<JWTError> = None;

        match clms {
            Ok(Some(claims)) => req.extensions_mut().insert::<Claims>(claims),
            Ok(None) => {}
            Err(err) => result_error = Some(err),
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(err) = result_error {
                Err(Error::from(ErrorResponse::from(err)))
            } else {
                let res = fut.await?;
                Ok(res)
            }
        })
    }
}
