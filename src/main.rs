#![allow(warnings, unused)]
#[macro_use]
extern crate json;

#[macro_use]
extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate listenfd;
extern crate serde;

extern crate tera;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use listenfd::ListenFd;

mod config;
mod db;
mod index;
mod router;
mod share;
mod user_api;

pub struct AppState {
    pub config: config::Config,
    pub db: Pool<ConnectionManager<SqliteConnection>>,
}

impl Default for AppState {
    fn default() -> AppState {
        let config: config::Config = config::Config::from_file();
        let db: Pool<ConnectionManager<SqliteConnection>> = db::init_pool(&config);
        AppState {
            config: config,
            db: db,
        }
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
    //    std::env::set_var("RUST_LOG", "actix_web=info");
    //    env_logger::init();
    let state: web::Data<AppState> = web::Data::new(AppState::default());

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

    server.run().unwrap();
}
