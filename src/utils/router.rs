use crate::guards::{login_required, unlogged_required};
use crate::index;
use crate::user_api;
use crate::user_api::{domain_create, domain_list, domain_states, domain_statistic, domain_update};
use actix_files as fs;
use actix_web::{guard, web, HttpResponse, Scope};

pub fn user_api_scope(path: &str) -> Scope {
    web::scope(path)
        .service(
            web::scope("auth").service(
                web::resource("login/")
                    .default_service(web::resource("").route(web::to(HttpResponse::Forbidden)))
                    .route(
                        web::post()
                            .guard(guard::fn_guard(unlogged_required))
                            .to(user_api::login),
                    ),
            ),
        )
        .service(
            web::scope("domain")
                .service(
                    web::resource("/")
                        .guard(guard::fn_guard(login_required))
                        .route(web::get().to(domain_list))
                        .route(web::post().to(domain_create))
                        .route(web::put().to(domain_update)),
                )
                .service(
                    web::resource("states/")
                        .guard(login_required)
                        .route(web::get().to(domain_states)),
                )
                .default_service(web::resource("").route(web::to(HttpResponse::Forbidden))),
        )
        .service(
            web::scope("statistic").service(
                web::resource("/")
                    .guard(guard::fn_guard(login_required))
                    .route(web::get().to(domain_statistic)),
            ),
        )
}

pub fn configuration(cfg: &mut web::ServiceConfig) {
    cfg.service(fs::Files::new("/static", "static/"))
        .service(fs::Files::new("/media", "media/"))
        .service(fs::Files::new("/ng", "src/ng/dist/").show_files_listing())
        .service(user_api_scope("api_user"))
        .service(web::resource("/").route(web::get().to(index::index)));
}
