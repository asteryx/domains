use std::pin::Pin;
use std::task::{Context, Poll};

use crate::config::Config;
use crate::jwt::{decode_token, Claims, JWTError};
use crate::state::AppState;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::Future;
use std::rc::Rc;
use std::sync::Mutex;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthenticationService {
    inner: Rc<String>,
}

impl Default for AuthenticationService {
    fn default() -> Self {
        Self {
            inner: Rc::new("JWT".to_string()),
        }
    }
}

impl AuthenticationService {
    fn new(auth_name: &str) -> Self {
        Self {
            inner: Rc::new(auth_name.to_string()),
        }
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

        if let Some(auth_header) = headers.get("Authorization".to_string()) {
            let parts: Vec<&str> = auth_header.to_str().unwrap_or("").split(" ").collect();
            dbg!(&parts);

            if parts.len() == 2 {
                let name = parts[0];
                let token = parts[1];
                if let Some(app_state) = &req.app_data::<AppState>() {
                    let opt_claims = get_claims(&app_state.config, name, token);
                    dbg!(&opt_claims);
                    if let Some(claims) = opt_claims {
                        req.extensions_mut().insert::<Claims>(claims)
                    }
                }
            }
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}

fn get_claims(config: &Config, name: &str, raw_token: &str) -> Option<Claims> {
    match decode_token(config, raw_token) {
        Ok(token_data) => Some(token_data.claims),
        _ => None,
    }
}
