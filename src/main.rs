// #[macro_use]
extern crate actix_web;
extern crate actix_files;
extern crate listenfd;
extern crate serde;

extern crate tera;

use listenfd::ListenFd;
use actix_web::{web, App, HttpServer, middleware};
use actix_files as fs;

mod index;
mod apps;
use apps::user_api::urls;

fn main() {
    let mut listenfd = ListenFd::from_env();
    std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    let mut server = HttpServer::new(
        || App::new()
        .wrap(middleware::Logger::default())
        .service(fs::Files::new("/static", "src/static/").show_files_listing())
        .service(fs::Files::new("/ng", "src/ng/dist/").show_files_listing())
        .service(
            urls::user_api("api/user")
        )
        .service(web::resource("/").route(web::get().to_async(index::index)))        
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run().unwrap();
}