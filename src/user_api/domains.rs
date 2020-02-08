use crate::db::models::domains::DomainList;
use crate::errors::ErrorResponse;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn domain_list(
    _req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    let db = &data.db;
    let domains = db
        .send(DomainList {
            limit: 0,
            offset: 0,
            status: None,
            search_string: None,
        })
        .await?;

    Ok(json_response(domains?))
}

pub async fn domain_create(
    _req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    Ok(json_response(10))
}
