use crate::helpers::query_parser::QueryParser;
use crate::models::earning_history_model::EarningHistoryInterval;
use crate::routes::types::{EarningHistoryParams, EarningHistoryResponse};
use crate::{db::connection::MongoDB, services::earnings_service::fetch_earnings_history};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/earnings")]
pub async fn handle_earnings_history(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<EarningHistoryParams>,
) -> impl Responder {
    let query_params = match QueryParser::new(&query.common, 100) {
        Ok(params) => params,
        Err(response) => return response,
    };
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !EarningHistoryInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }
    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };
    let interval_str = query.interval.as_deref().unwrap_or("hour");
    match fetch_earnings_history(&mongo_db, query_params, &interval_str, sort_by, order).await {
        Ok((meta, intervals)) => {
            HttpResponse::Ok().json(EarningHistoryResponse { meta, intervals })
        }
        Err(error_message) => HttpResponse::InternalServerError().body(error_message),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(handle_earnings_history);
}
