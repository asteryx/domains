use crate::db::models::domains::{DomainInsert, DomainList, DomainState};
use crate::errors::ErrorResponse;
use crate::jwt::Claims;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LimitOffset {
    limit: Option<usize>,
    offset: Option<usize>,
    state: Option<DomainState>,
    q: Option<String>,
}

pub async fn domain_list(
    data: web::Data<AppState>,
    web::Query(query_params): web::Query<LimitOffset>,
    _req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let db = &data.db;

    let domains = db
        .send(DomainList {
            state: query_params.state,
            limit: query_params.limit,
            offset: query_params.offset,
            search_string: query_params.q,
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

// pub async fn domain_delete(
//     input_domain: web::Json<DomainCreate>,
//     claims: Claims,
//     data: web::Data<AppState>,
// ) -> Result<HttpResponse, ErrorResponse> {
// }
