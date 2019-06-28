extern crate actix_web;
extern crate actix_files;
extern crate listenfd;
extern crate serde;
// #[macro_use] 
// extern crate serde_derive;
// extern crate serde_json;
// #[macro_use]
extern crate tera;

use listenfd::ListenFd;
use actix_web::{web, App, HttpServer};
use actix_files as fs;

mod index;


fn main() {
    let mut listenfd = ListenFd::from_env();
    
    let mut server = HttpServer::new(
        || App::new()
        .service(fs::Files::new("/static", "src/static/").show_files_listing())
        .service(fs::Files::new("/ng", "src/ng/dist/").show_files_listing())
        .service(web::resource("/{tail:.*}").route(web::get().to(index::index)))
        
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run().unwrap();
}