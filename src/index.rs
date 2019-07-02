use actix_web::{HttpResponse, HttpRequest, Error};
use tera::{Tera, Context};
use futures::{future::ok, Future};

// #[derive(Serialize)]
// struct MyObj {
//     name: &'static str,
// }

// /// Responder
// impl Responder for MyObj {
//     type Item = HttpResponse;
//     type Error = Error;

//     fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
//         let body = serde_json::to_string(&self)?;

//         // Create response and set content type
//         Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body))
//     }
// }



pub fn index(_req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    let mut tera = Tera::default();
    let context = Context::new();  
    match tera.add_template_file("src/ng/dist/index.html", Some("index")) {
        Ok(res)=> res,
        Err(e) => println!("Error ALARM!!!!! {:?}", e),
    };

    let rendered = tera.render("index", &context).unwrap();

    ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered))
}
