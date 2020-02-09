use crate::db::models::domains::{DomainInsert, DomainList, DomainState};
use crate::errors::ErrorResponse;
use crate::jwt::Claims;
use crate::middleware::QueryParams;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

pub async fn domain_list(
    data: web::Data<AppState>,
    query_params: QueryParams,
    _req: HttpRequest,
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DomainCreate {
    pub name: String,
    #[validate(url)]
    pub url: String,
    pub state: DomainState,
}

pub async fn domain_create(
    input_domain: web::Json<DomainCreate>,
    claims: Claims,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    match &input_domain.validate() {
        Ok(_) => (),
        Err(err) => {
            let error_exp = err
                .errors()
                .keys()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            return Err(ErrorResponse {
                msg: format!("Validation errors in fields: {}", error_exp),
                status: 400,
            });
        }
    };

    let res = data
        .db
        .send(DomainInsert {
            name: input_domain.name.clone(),
            url: input_domain.url.clone(),
            state: input_domain.state.clone(),
            author: claims.user_id(),
        })
        .await?;

    Ok(json_response(res?))
}
