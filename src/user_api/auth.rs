use crate::db;
use crate::errors::ErrorResponse;
use crate::jwt::encode_token;
use crate::AppState;

use actix_web::{web, HttpRequest, HttpResponse};

use crate::utils::json_response;
use serde_derive::{Deserialize, Serialize};

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
) -> Result<HttpResponse, ErrorResponse> {
    let res = data
        .db
        .send(db::models::users::FindUser {
            email: login.email.clone(),
        })
        .await?;

    if let Ok(user) = res {
        if user.check_password(&login.password) {
            return Ok(json_response(LoginResult {
                token: encode_token(&data.config, &user)?,
                email: user.email,
                name: user.name,
            }));
        }
    }
    Err(ErrorResponse {
        msg: "Username/password didn't match".to_string(),
        status: 400,
    })
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
