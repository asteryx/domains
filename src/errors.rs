use actix::MailboxError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_derive::{Deserialize, Serialize};
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
