//#![allow(warnings, unused)]
#[macro_use]
extern crate json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate actix;
extern crate actix_files;
extern crate actix_identity;
extern crate actix_service;
extern crate actix_web;
extern crate dotenv;
extern crate listenfd;
extern crate serde;
extern crate tera;
#[macro_use]
extern crate log;
extern crate env_logger;
use log::Level;

use crate::db::models::domains::Domain;
use crate::db::DbExecutor;
use crate::services::ping::Ping;
use actix::prelude::*;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{client, middleware as actix_middleware, web, App, HttpResponse, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use env_logger::{builder, Builder};
use listenfd::ListenFd;
use middleware::AuthenticationService;
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tera::ast::ExprVal::StringConcat;

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
    let db = SyncArbiter::start(2, move || db::DbExecutor::new());
    let ping: Addr<Ping> = SyncArbiter::start(2, || Ping::new());

    let app_state: AppState = AppState::new(db.clone(), ping.clone());
    let jobs_state: AppState = AppState::new(db.clone(), ping.clone());
    let config_log_level = app_state.config.log_level().clone();

    let state: web::Data<AppState> = web::Data::new(app_state);
    let arc_state = Arc::new(web::Data::new(jobs_state));

    let env_name = String::from("DOMAINS_LOGLEVEL");
    // Check if set env for logging
    match std::env::var(&env_name) {
        Ok(_) => (),
        Err(_) => {
            std::env::set_var(&env_name, format!("{}", config_log_level));
        }
    };
    let mut builder = Builder::from_env(&env_name);
    builder.init();

    if let Ok(res) = std::env::var(&env_name) {
        info!("Log level set is {}", res);
    }

    debug!("CPU's num {}", num_cpus::get());
    thread::spawn(move || jobs::ping_fn(arc_state));

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(
                web::JsonConfig::default()
                    .limit(1024)
                    .error_handler(errors::json_error_handler),
            )
            .wrap(actix_middleware::Logger::default())
            .wrap(AuthenticationService::default())
            .configure(router::configuration)
    });

    let server_ip = match std::env::var("SERVER_ADDR") {
        Ok(var) => var,
        Err(_) => "127.0.0.1".to_string(),
    };
    let port = match std::env::var("SERVER_PORT") {
        Ok(var) => var,
        Err(_) => "8000".to_string(),
    };

    let server_address = format!("{}:{}", server_ip, port);
    debug!("{}", &server_address);

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(server_address).unwrap()
    };
    server.run().await
}
