use actix_web::{ web, HttpResponse, Scope };

use super::handlers::register;

pub fn user_api(path: &str) -> Scope {
    web::scope(path)
    .data(web::JsonConfig::default().limit(4096))
    .service(
        web::resource("/register")
        .data(web::JsonConfig::default().limit(1024))
        .route(web::post().to_async(register)))
    // .service(web::resource("/path2").to_async(|| HttpResponse::Ok()))
    // .service(web::resource("/path3").to_async(|| HttpResponse::MethodNotAllowed()))
}
