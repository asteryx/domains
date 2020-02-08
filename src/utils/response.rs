use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::json;

pub fn json_response<R: Serialize>(value: R) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!(value))
}
