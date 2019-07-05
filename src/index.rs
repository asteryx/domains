use actix_web::{HttpResponse, HttpRequest, Error};
use tera::{Tera, Context};
use futures::{future::ok, Future};


pub fn index(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut tera = Tera::default();
    let context = Context::new();  

    tera.add_template_file("src/ng/dist/index.html", Some("index"))
    .expect("Cannot open template file");

    let rendered = tera.render("index", &context)
    .expect("Cannot render template 'index'");
    
    ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered))
}
