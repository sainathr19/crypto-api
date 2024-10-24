use crate::helpers::query_parser::QueryParser;
use crate::models::earnings_history::EarningsHistoryInterval;
use crate::routes::members_history::CommonQueryParams;
use crate::{db::connection::MongoDB, services::earnings_service::get_earnings_history_data};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct EarningsHistoryParams {
    #[serde(flatten)]
    common: CommonQueryParams,
    interval: Option<String>,
    sort_by: Option<String>,
    order: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EarningsHistoryMeta {
    pub count: i64,
    pub page: i64,
    pub hasNextPage: bool,
}
#[derive(Debug, Deserialize, Serialize)]
struct EarningsHistoryResponse {
    meta: EarningsHistoryMeta,
    intervals: Vec<EarningsHistoryInterval>,
}
#[get("/earnings")]
pub async fn get_earnings_data(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<EarningsHistoryParams>,
) -> impl Responder {
    let pagination_params = match QueryParser::new(&query.common) {
        Ok(params) => params,
        Err(response) => return response,
    };
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !EarningsHistoryInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }
    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };
    let interval_str = query.interval.as_deref().unwrap_or("hour");
    match get_earnings_history_data(&mongo_db, pagination_params, &interval_str, sort_by, order)
        .await
    {
        Ok((meta, intervals)) => {
            HttpResponse::Ok().json(EarningsHistoryResponse { meta, intervals })
        }
        Err(error_message) => HttpResponse::InternalServerError().body(error_message),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(get_earnings_data);
}
