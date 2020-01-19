use crate::errors::ErrorResponse;
use crate::index;
use crate::user_api::auth::login;
use actix::Addr;

use actix_files as fs;
use actix_web::error::JsonPayloadError;
use actix_web::{error, web, App, HttpRequest, HttpResponse, Scope};
use serde::private::de::IdentifierDeserializer;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

pub fn user_api_scope(path: &str) -> Scope {
    web::scope(path).service(web::resource("/login").route(web::post().to(login)))
    // .service(web::resource("/path2").to_async(|| HttpResponse::Ok()))
    // .service(web::resource("/path3").to_async(|| HttpResponse::MethodNotAllowed()))
}
