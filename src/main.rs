#![allow(warnings, unused)]
#[macro_use]
extern crate json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate listenfd;
extern crate serde;
extern crate tera;
use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use listenfd::ListenFd;
use log::Level;
use std::thread;

mod config;
mod db;
mod errors;
mod hashers;
mod index;
mod router;
mod services;
mod share;
mod user_api;

#[macro_use]
extern crate log;
extern crate env_logger;

use crate::db::DbExecutor;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub struct AppState {
    pub config: config::Config,
    pub db: Addr<DbExecutor>,
}

impl AppState {
    fn new(db: Addr<DbExecutor>) -> AppState {
        let config: config::Config = config::Config::from_file();
        AppState { config, db }
    }
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AppState")
            .field("config", &self.config)
            .field(
                "db",
                &format_args!("Pool<ConnectionManager<SqliteConnection>>"),
            )
            .finish()
    }
}

fn main() {
    let mut listenfd = ListenFd::from_env();

    let sys = System::builder()
        .stop_on_panic(true)
        .name("domains")
        .build();

    //    let db = SyncArbiter::start(num_cpus::get() * 3, move || db::DbExecutor::new());
    let db = SyncArbiter::start(1, move || db::DbExecutor::new());

    let app_state: AppState = AppState::new(db);
    let log_level = app_state.config.log_level.clone();

    let state: web::Data<AppState> = web::Data::new(app_state);
    let arc_state = Arc::new(state.clone());

    // Check if set env for logging
    match std::env::var("RUST_LOG") {
        Ok(_) => (),
        Err(_) => {
            std::env::set_var("RUST_LOG", format!("actix_web={}", log_level));
        }
    };
    env_logger::init();

    //    let ping = services::ping::Ping::new(state.clone()).start();

    debug!("CPU's num {}", num_cpus::get());

    thread::spawn(move || services::ping::ping_fn(arc_state.clone()));

    let mut server = HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/static", "src/static/").show_files_listing())
            .service(fs::Files::new("/ng", "src/ng/dist/").show_files_listing())
            .service(router::user_api_scope("api_user"))
            .service(web::resource("/").route(web::get().to_async(index::index)))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };
    server.start();
    sys.run();
}
