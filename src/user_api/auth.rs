use crate::errors::ErrorResponse;
use crate::AppState;
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
    use crate::db::models::User;
    use crate::db::schema::users::dsl::*;

    let connection = match data.db.get() {
        Ok(conn) => conn,
        Err(_) => {
            return err(ErrorResponse {
                msg: "error dsfsdfdf".to_string(),
                status: 400,
            })
        }
    };

    let results: Vec<User> = users
        .filter(email.eq(&login.email))
        .limit(2)
        .load::<User>(&connection)
        .expect("Error loading posts");

    let count_users: usize = results.iter().count();

    if count_users == 1 {
        let user = &results[0];
        //check password

        ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(json!(user)))
    } else if count_users > 1 {
        //too many users ???????
        unreachable!()
    } else {
        // If db is no user create user with password
        err(ErrorResponse {
            msg: "Username/password didn't match".to_string(),
            status: 400,
        })
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
