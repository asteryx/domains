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

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new("/static", "static/"))
        .service(fs::Files::new("/media", "media/"))
        .service(fs::Files::new("/ng", "src/ng/dist/").show_files_listing())
        .service(user_api_scope("api_user"))
        .service(web::resource("/").route(web::get().to(index::index)));
}
