#![allow(warnings, unused)]
#[macro_use]
extern crate json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate actix;
extern crate actix_files;
extern crate actix_web;
//extern crate actix_identity;
extern crate dotenv;
extern crate listenfd;
extern crate serde;
extern crate tera;
#[macro_use]
extern crate log;
extern crate env_logger;

use actix::prelude::*;
use actix_files as fs;
use actix_web::{client, middleware, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use listenfd::ListenFd;
use log::Level;
use std::thread;
use crate::db::DbExecutor;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::io;

mod config;
mod db;
mod errors;
mod hashers;
mod index;
mod jobs;
mod router;
mod services;
mod share;
mod state;
mod user_api;

#[macro_use]
extern crate log;
extern crate env_logger;

use crate::db::models::domains::Domain;
use crate::db::DbExecutor;
use crate::services::ping::Ping;
use crate::state::AppState;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[actix_rt::main]
async fn main() -> io::Result<()>{
    let mut listenfd = ListenFd::from_env();

    let sys = System::builder()
        .stop_on_panic(true)
        .name("domains")
        .build();

    //    let db = SyncArbiter::start(num_cpus::get() * 3, move || db::DbExecutor::new());
    let db = SyncArbiter::start(2, move || db::DbExecutor::new());
    let ping: Addr<Ping> = SyncArbiter::start(2, || Ping::new());

    let app_state: AppState = AppState::new(db, ping);
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

    debug!("CPU's num {}", num_cpus::get());

    thread::spawn(move || jobs::ping_fn(arc_state.clone()));

    let mut server = HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .wrap(middleware::Logger::default())
            //            .service(fs::Files::new("/static", "src/static/").show_files_listing())
            //            .service(fs::Files::new("/ng", "src/ng/dist/").show_files_listing())
            //            .service(router::user_api_scope("api_user"))
            .service(web::resource("/").route(web::get().to(index::index)))
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
    println!("{}", &server_address);
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(server_address).unwrap()
    };
    server.start();
    sys.run()
}
