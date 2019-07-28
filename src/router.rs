use actix_files as fs;
use actix_web::{web, App, HttpResponse, Scope};

use crate::index;
use crate::models::db::ConnDsl;
use crate::share::common::AppState;
use crate::user_api::auth::login;
use actix::Addr;

pub fn user_api_scope(path: &str) -> Scope {
    web::scope(path)
        .data(web::JsonConfig::default().limit(4096))
        .service(
            web::resource("/login")
                .data(web::JsonConfig::default().limit(1024))
                .route(web::post().to_async(login)),
        )
    // .service(web::resource("/path2").to_async(|| HttpResponse::Ok()))
    // .service(web::resource("/path3").to_async(|| HttpResponse::MethodNotAllowed()))
}
