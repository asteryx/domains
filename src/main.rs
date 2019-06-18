extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, Responder, fs};


fn index(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello to {}!", to)
    
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            // .resource("/", |r| r.f(index))
            .resource("/{name}", |r| {
                r.get().f(index);
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