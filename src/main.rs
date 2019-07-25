#[macro_use]
extern crate json;

#[macro_use]
extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate listenfd;
extern crate serde;

extern crate tera;
use crate::models::db::init;
use crate::share::common::AppState;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use listenfd::ListenFd;

mod index;
mod models;
mod router;
mod share;
mod user_api;

fn main() {
    let mut listenfd = ListenFd::from_env();
    std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();
    let manager = ConnectionManager::<SqliteConnection>::new("data.sqlite");
    let pool = Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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

    server.run().unwrap();
}
