use crate::db;
use crate::errors::ErrorResponse;
use crate::AppState;
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse, ResponseError};
use diesel::prelude::*;
use futures::stream::Stream;
use futures::{future::err, future::ok, Future};
use json::JsonValue;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

pub fn login(
    login: web::Json<Login>,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Future<Item = HttpResponse, Error = ErrorResponse> {
    data.db
        .send(db::models::users::ActorUser {
            email: login.email.clone(),
        })
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                dbg!(&user);
                if user.check_password(&*login.password) {
                    ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .json(json!(user)))
                } else {
                    err(ErrorResponse {
                        msg: "Username/password didn't match".to_string(),
                        status: 400,
                    })
                }
            }
            Err(_) => {
                dbg!("error in db");
                err(ErrorResponse {
                    msg: "Username/password didn't match".to_string(),
                    status: 400,
                })
            }
        })

    //    let count_users: usize = results.iter().count();
    //
    //    if count_users == 1 {
    //        let user = &results.pop().expect("Error pop user from users array");
    //
    //        if user.check_password(&*login.password) {
    //            return ok(HttpResponse::Ok()
    //                .content_type("application/json")
    //                .json(json!(user)));
    //        };
    //    } else if count_users > 1 {
    //        log::error!("Too many users selected with username {}", &login.email);
    //    }

    //    err(ErrorResponse {
    //        msg: "Temporary technical problems on the server".to_string(),
    //        status: 500,
    //    })
    // If db is no user or password invalid
    //    dbg!("not used actors");
    //    err(ErrorResponse {
    //        msg: "Username/password didn't match".to_string(),
    //        status: 400,
    //    })
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
