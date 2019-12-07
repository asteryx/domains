use crate::errors::ErrorResponse;
use crate::index;
use crate::user_api::auth::login;
use actix::Addr;

use actix_files as fs;
use actix_web::error::JsonPayloadError;
use actix_web::{error, web, App, HttpRequest, HttpResponse, Scope};
use regex::Regex;
use serde::private::de::IdentifierDeserializer;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

pub fn user_api_scope(path: &str) -> Scope {
    web::scope(path)
        .data(web::JsonConfig::default().limit(4096))
        .service(
            web::resource("/login")
                .data(
                    web::JsonConfig::default()
                        .limit(1024)
                        .error_handler(json_error_handler),
                )
                .route(web::post().to(login)),
        )
    // .service(web::resource("/path2").to_async(|| HttpResponse::Ok()))
    // .service(web::resource("/path3").to_async(|| HttpResponse::MethodNotAllowed()))
}

fn json_error_handler(err: JsonPayloadError, req: &HttpRequest) -> actix_web::Error {
    let error_message: String = match err {
        JsonPayloadError::Payload(payload_error) => format!("{}", payload_error),
        JsonPayloadError::Deserialize(error) => {
            let mut tmp: String = "".to_string();

            if error.is_data() {
                let (type_error, field_name): (String, String) =
                    parse_error_text(format!("{}", error));

                if field_name != "" {
                    tmp = format!("Data error: {} `{}`", type_error, field_name);
                }
            }

            if tmp == "".to_string() {
                tmp = format!("{}", &error);
            }
            tmp
        }
        _ => format!("{}", err),
    };
    ErrorResponse {
        msg: error_message,
        status: 400,
    }
    .into()
}

fn parse_error_text(input_str: String) -> (String, String) {
    let re = Regex::new(r"(?P<type_error>duplicate field|missing field) `(?P<field_name>.*)`")
        .expect("Error creating regex");

    log::info!("from error: {}", &input_str);

    let caps = re.captures(&input_str).unwrap();
    let ger_field_value = |field_name| match &caps.name(field_name) {
        Some(mtch) => mtch.as_str().to_string(),
        _ => String::from(""),
    };
    let type_error = ger_field_value("type_error");
    let field_name = ger_field_value("field_name");
    (type_error, field_name)
}
