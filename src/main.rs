
extern crate actix_web;
extern crate listenfd;
extern crate serde;
// #[macro_use] 
// extern crate serde_derive;
// extern crate serde_json;
// #[macro_use]
extern crate tera;

use listenfd::ListenFd;
use actix_web::{server, App, fs};

mod index;


fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| {
                r.get().f(index::index);
            })
            .handler("/static", fs::StaticFiles::new("src/static/dist/")
            .unwrap()
            .show_files_listing())
            .handler("/ng", fs::StaticFiles::new("src/static/dist/")
            .unwrap()
            .show_files_listing())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}