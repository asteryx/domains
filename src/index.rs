use actix_web::{HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};


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



pub fn index(_req: &HttpRequest) -> impl Responder {
    // let tera = compile_templates!("src/static/dist/*");
    let mut tera = Tera::default();
    let context = Context::new();  
    match tera.add_template_file("src/static/dist/index.html", Some("index")) {
        Ok(res)=> res,
        Err(e) => println!("{:?}", e),
    };
    
    println!("templates {}", tera.templates.len());

    let rendered = tera.render("index", &context).unwrap();

    // println!("terra {}", rendered.unwrap());
    HttpResponse::Ok()
            .content_type("text/html; charset=UTF-8")
            .body(rendered)
}
