use crate::AppState;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use futures::stream::Stream;
use futures::{future::ok, Future};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct MyError {
    msg: String,
    status: u16,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for MyError {
    // builds the actual response to send back when an error occurs
    fn render_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.msg });

        println!("{}", &self);

        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

// web::Json

pub fn login(
    item: web::Json<Login>,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = MyError> {
    ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(item.0))
}

// pub fn register (pl: web::Payload) -> impl Future<Item = HttpResponse, Error = Error> {
//     pl.concat2().from_err().and_then(|body| {
//         // body is loaded, now we can deserialize json-rust
//         let result: Register = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap(); // return Result
//         // let injson = match result {
//         //     Ok(v) => v,
//         //     Err(e) => json::object! {"err" => e.to_string() },
//         // };

//         println!("{:?}", result);

//         Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(result))
//     })
// }
