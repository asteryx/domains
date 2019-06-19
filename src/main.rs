extern crate actix_web;
extern crate listenfd;

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
            .handler("/static", fs::StaticFiles::new("src/static")
            .unwrap())
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8000").unwrap()
    };

    server.run();
}