use crate::db::models::domains::{DomainInsertUpdate, DomainList, DomainState};
use crate::errors::ErrorResponse;
use crate::jwt::Claims;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ListQuery {
    limit: Option<usize>,
    offset: Option<usize>,
    state: Option<DomainState>,
    q: Option<String>,
}

pub async fn domain_list(
    data: web::Data<AppState>,
    web::Query(query_params): web::Query<ListQuery>,
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

fn validate_domain(input_domain: &web::Json<DomainInsertUpdate>) -> Result<(), ErrorResponse> {
    match &input_domain.validate() {
        Ok(_) => Ok(()),
        Err(err) => {
            let error_exp = err
                .errors()
                .keys()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            Err(ErrorResponse {
                msg: format!("Validation errors in fields: {}", error_exp),
                status: 400,
            })
        }
    }
}

pub async fn domain_create(
    input_domain: web::Json<DomainInsertUpdate>,
    claims: Claims,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    validate_domain(&input_domain)?;

    let res = data
        .db
        .send(DomainInsertUpdate {
            id: None,
            name: input_domain.name.clone(),
            url: input_domain.url.clone(),
            state: input_domain.state.clone(),
            author: Some(claims.user_id()),
        })
        .await?;

    dbg!(&res);

    Ok(json_response(res?))
}

pub async fn domain_update(
    input_domain: web::Json<DomainInsertUpdate>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    validate_domain(&input_domain)?;

    dbg!(&input_domain);

    Ok(json_response(
        data.db
            .send(DomainInsertUpdate {
                id: input_domain.id,
                name: input_domain.name.clone(),
                url: input_domain.url.clone(),
                state: input_domain.state.clone(),
                author: None,
            })
            .await??,
    ))
}

pub async fn domain_status(_: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    use std::collections::HashMap;
    use strum::IntoEnumIterator;
    use std::iter::FromIterator;

    let res: HashMap<_, _> = FromIterator::from_iter(DomainState::iter().map(|ds| (ds as i32, ds.to_string())));
    Ok(json_response(res))
}
