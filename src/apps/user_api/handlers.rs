use actix_web::{web, HttpRequest, HttpResponse, Error};
use futures::{future::ok, Future};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    email: String,
    name: String,
    password: String,
    confirm_password: String
}


pub fn register (item: web::Json<Register>, req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    // HttpResponse::Ok().json(item.0) // <- send json response
    ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(item.0))
}
