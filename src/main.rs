//#![allow(warnings, unused)]

extern crate json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
extern crate actix;
extern crate actix_files;
extern crate actix_identity;
extern crate actix_service;
extern crate actix_web;
extern crate dotenv;
extern crate listenfd;
extern crate serde;
extern crate tera;
extern crate validator;
#[macro_use]
extern crate log;
extern crate env_logger;

use crate::services::ping::Ping;
use actix::prelude::*;
use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use env_logger::Builder;
use listenfd::ListenFd;
use middleware::AuthenticationService;
use std::io;
use std::sync::Arc;
use std::thread;

mod db;
mod index;
mod jobs;
mod middleware;
mod services;
mod share;
mod user_api;
mod utils;

pub use utils::{config, errors, guards, hashers, jwt, router, state, state::AppState};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut listenfd = ListenFd::from_env();

    //    let db = SyncArbiter::start(num_cpus::get() * 3, move || db::DbExecutor::new());
    let db = SyncArbiter::start(2, move || db::DbExecutor::default());
    let ping: Addr<Ping> = SyncArbiter::start(2, || Ping::new());

    let app_state: AppState = AppState::new(db.clone(), ping.clone());
    let jobs_state: AppState = AppState::new(db.clone(), ping.clone());

    let cloned_config = app_state.config.clone();

    let state: web::Data<AppState> = web::Data::new(app_state);
    let arc_state = Arc::new(web::Data::new(jobs_state));

    std::env::set_var(
        cloned_config.log_level_env_name(),
        format!("{}", cloned_config.log_level()),
    );

    let mut builder = Builder::from_env(cloned_config.log_level_env_name());
    builder.init();

    info!("Log level set is {}", cloned_config.log_level());

    let server_ip = cloned_config.server_addr();
    let port = cloned_config.server_port();

    info!("CPU's num {}", num_cpus::get());
    thread::spawn(move || jobs::ping_fn(arc_state));

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(
                web::JsonConfig::default()
                    .limit(1024)
                    .error_handler(errors::json_error_handler),
            )
            .app_data(web::QueryConfig::default().error_handler(errors::query_error_handler))
            .wrap(actix_middleware::Logger::default())
            .wrap(AuthenticationService::default())
            .configure(router::configuration)
    });

    let server_address = format!("{}:{}", server_ip, port);
    info!("{}", &server_address);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(server_address).unwrap()
    };
    server.run().await
}
