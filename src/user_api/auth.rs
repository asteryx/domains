use crate::db;
use crate::errors::ErrorResponse;
use crate::jwt::encode_token;
use crate::AppState;

use actix_web::{web, HttpRequest, HttpResponse};

use serde_derive::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputLoginData {
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
    login: web::Json<InputLoginData>,
    _req: HttpRequest,
    data: web::Data<AppState>,
    //    TODO add to this from request trait
    //    claims: Claims,
) -> Result<HttpResponse, ErrorResponse> {
    //    let exts = req.extensions();
    //    let clms = exts.get::<Claims>();
    //    dbg!(&clms);

    let res = data
        .db
        .send(db::models::users::FindUser {
            email: login.email.clone(),
        })
        .await?;

    match res {
        Ok(user) => {
            if user.check_password(&login.password) {
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(json!(LoginResult {
                        token: encode_token(&data.config, &user)?,
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
        _ => Err(ErrorResponse {
            msg: "Username/password didn't match".to_string(),
            status: 400,
        }),
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
