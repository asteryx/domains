extern crate actix_web;

use actix_web::{HttpRequest, Responder};


pub fn index(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello to {}!", to)
    
}
