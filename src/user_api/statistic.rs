use crate::db::models::statistic::Statistic;
use crate::errors::ErrorResponse;
use crate::utils::json_response;
use crate::AppState;
use actix_web::{web, HttpResponse};
use validator::Validate;

pub async fn domain_statistic(
    web::Query(query_params): web::Query<Statistic>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    // RAW format
    // 2020-03-15T18%3A09%3A09.549189368%2B02%3A00
    // dbg!(&query_params.dt_start.unwrap()); // 2020-03-15T18:09:09.549189368+02:00
    // dbg!(&query_params.dt_start.unwrap().naive_utc()); // 2020-03-15T16:09:09.549189368
    &query_params.validate()?;

    let res = &data.db.send(query_params).await??;

    Ok(json_response(res))
}
