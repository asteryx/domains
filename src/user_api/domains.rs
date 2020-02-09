use crate::db::models::domains::{DomainList, DomainState};
use crate::errors::ErrorResponse;
use crate::middleware::QueryParams;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn domain_list(
    //    req: HttpRequest,
    data: web::Data<AppState>,
    query_params: QueryParams,
) -> Result<HttpResponse, ErrorResponse> {
    let db = &data.db;
    let limit = query_params.get("limit", "0").parse().unwrap_or(0);
    let offset = query_params.get("offset", "0").parse().unwrap_or(0);
    let state_str = query_params.get("state", "");
    let q = query_params.get("q", "");

    let state = if state_str.len() > 0 {
        Some(DomainState::from(state_str))
    } else {
        None
    };

    let search_string = if q.len() > 0 { Some(q) } else { None };

    let domains = db
        .send(DomainList {
            limit,
            offset,
            state,
            search_string,
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
