use crate::db;
use crate::errors::ErrorResponse;
use crate::jwt::{decode_token, encode_token, Claims};
use crate::AppState;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use diesel::prelude::*;
use futures::stream::Stream;
use futures::{future::err, future::ok, Future};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResult {
    token: String,
    email: String,
    name: String,
}

pub async fn login(
    login: web::Json<Login>,
    req: HttpRequest,
    data: web::Data<AppState>,
    //    TODO add this to from request
    //    claims: Claims,
) -> Result<HttpResponse, ErrorResponse> {
    let exts = req.extensions();
    let clms = exts.get::<Claims>();
    dbg!(&clms);

    let res = data
        .db
        .send(db::models::users::FindUser {
            email: login.email.clone(),
        })
        .await?;

    match res {
        Ok(user) => {
            if user.check_password(&login.password) {
                let token = encode_token(&data.config, &user)?;

                //                let claims_data = decode_token(&data.config, &token)?;
                //                dbg!(&claims_data);

                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(json!(LoginResult {
                        token: token,
                        email: user.email,
                        name: user.name
                    })))
            } else {
                Err(ErrorResponse {
                    msg: "Username/password didn't match".to_string(),
                    status: 400,
                })
            }
        }
        _ => {
            //            Add check password
            Err(ErrorResponse {
                msg: "Username/password didn't match".to_string(),
                status: 400,
            })
        }
    }
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
