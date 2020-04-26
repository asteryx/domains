use crate::db::models::domains::{DomainInsertUpdate, DomainList, DomainState};
use crate::errors::ErrorResponse;
use crate::jwt::Claims;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

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

pub async fn domain_create(
    input_domain: web::Json<DomainInsertUpdate>,
    claims: Claims,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    &input_domain.validate()?;

    let res = data
        .db
        .send(DomainInsertUpdate {
            id: None,
            name: input_domain.name.clone(),
            url: input_domain.url.clone(),
            state: input_domain.state.clone(),
            author: Some(claims.user_id()),
            color: input_domain.color.clone(),
        })
        .await?;

    Ok(json_response(res?))
}

pub async fn domain_update(
    input_domain: web::Json<DomainInsertUpdate>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    &input_domain.validate()?;

    Ok(json_response(
        data.db
            .send(DomainInsertUpdate {
                id: input_domain.id,
                name: input_domain.name.clone(),
                url: input_domain.url.clone(),
                state: input_domain.state.clone(),
                author: None,
                color: input_domain.color.clone(),
            })
            .await??,
    ))
}

pub async fn domain_states(_: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    use std::iter::FromIterator;
    use strum::IntoEnumIterator;

    let list_objects: Vec<_> = Vec::from_iter(DomainState::iter().map(|ds| {
        serde_json::json!({
        "id": ds as i32,
        "name": ds.to_string()
        })
    }));

    Ok(json_response(list_objects))
}
