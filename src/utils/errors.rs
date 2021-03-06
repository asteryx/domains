use crate::jwt;
use actix::MailboxError;
use actix_web::error::{JsonPayloadError, QueryPayloadError};
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use regex::Regex;
use serde_derive::Serialize;
use serde_json::{json, to_string_pretty};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub msg: String,
    pub status: u16,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for ErrorResponse {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "msg": self.msg });

        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(err_json)
    }
}

impl From<MailboxError> for ErrorResponse {
    fn from(error: MailboxError) -> Self {
        log::error!("Error in mailbox {}", error);

        ErrorResponse {
            msg: "Something went wrong. Please try again later".to_string(),
            status: 500,
        }
    }
}

impl From<jwt::JWTError> for ErrorResponse {
    fn from(outer: jwt::JWTError) -> Self {
        ErrorResponse {
            msg: format!("{}", outer),
            status: 401,
        }
    }
}

impl From<std::io::Error> for ErrorResponse {
    fn from(outer: std::io::Error) -> Self {
        ErrorResponse {
            msg: format!("{}", outer),
            status: 400,
        }
    }
}

impl From<QueryPayloadError> for ErrorResponse {
    fn from(outer: QueryPayloadError) -> Self {
        ErrorResponse {
            msg: format!("{}", outer),
            status: 400,
        }
    }
}

impl From<validator::ValidationErrors> for ErrorResponse {
    fn from(outer: validator::ValidationErrors) -> Self {
        let error_exp = outer
            .errors()
            .keys()
            .map(|&r| r.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        ErrorResponse {
            msg: format!("Validation errors in fields: {}", error_exp),
            status: 400,
        }
    }
}

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> actix_web::Error {
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

pub fn query_error_handler(err: QueryPayloadError, _req: &HttpRequest) -> actix_web::Error {
    let error_message: String = match err {
        QueryPayloadError::Deserialize(deserialize_error) => format!("{}", deserialize_error),
        // _ => format!("{}", err),
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

    let caps = re.captures(&input_str);

    let ger_field_value = |field_name: &str| match &caps {
        Some(cap) => match &cap.name(field_name) {
            Some(mtch) => mtch.as_str().to_string(),
            _ => String::from(""),
        },
        _ => String::from(""),
    };

    let type_error = ger_field_value("type_error");
    let field_name = ger_field_value("field_name");
    (type_error, field_name)
}
