use std::pin::Pin;
use std::task::{Context, Poll};

use crate::errors::ErrorResponse;
use actix_service::{Service, Transform};
use actix_web::dev::Payload;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpRequest};
use futures::future::{ok, Ready};
use futures::Future;
use std::collections::HashMap;
use urldecode::decode;

#[derive(Clone, Debug)]
pub struct QueryParams {
    inner: HashMap<String, String>,
}

impl QueryParams {
    fn new(encoded_query_string: String) -> Self {
        let mut inner: HashMap<String, String> = HashMap::new();
        let decoded_string = decode(encoded_query_string);

        let res = decoded_string
            .split("&")
            .map(|part| part.to_string())
            .collect::<Vec<String>>();

        for part in res.iter() {
            let kv = part.split('=').collect::<Vec<&str>>();
            if kv.len() == 2 && kv[0].len() > 0 && kv[1].len() > 0 {
                inner.insert(kv[0].to_string(), kv[1].to_string());
            }
        }

        QueryParams { inner }
    }

    pub fn get(&self, name: &str, default: &str) -> String {
        match self.inner.get(name) {
            Some(res) => res.to_string(),
            _ => default.to_string(),
        }
    }
}

impl actix_web::FromRequest for QueryParams {
    // The associated error which can be returned.
    type Error = ErrorResponse;

    // Future that resolves to a Self
    type Future = Ready<Result<Self, Self::Error>>;

    // Configuration for this extractor
    type Config = ();

    // Convert request to a Self
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let qp = extensions.get::<QueryParams>();
        match qp {
            Some(q_param) => ok(q_param.clone()),
            _ => ok(QueryParams::new("".to_string())),
        }
    }

    // Convert request to a Self
    //
    // This method uses `Payload::None` as payload stream.
    //    fn extract(req: &HttpRequest) -> Self::Future {
    //        Self::from_request(req, &mut Payload::None)
    //    }
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct QueryParametersService {}

impl Default for QueryParametersService {
    fn default() -> Self {
        Self {}
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for QueryParametersService
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = QueryParametersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(QueryParametersMiddleware { service })
    }
}

pub struct QueryParametersMiddleware<S> {
    service: S,
}

impl<S, B> Service for QueryParametersMiddleware<S>
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
        let req_string = &req.query_string();

        if req_string.len() > 0 {
            req.extensions_mut()
                .insert(QueryParams::new(req_string.to_string()));
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
