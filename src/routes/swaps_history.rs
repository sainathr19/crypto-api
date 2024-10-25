use crate::helpers::query_parser::QueryParser;
use crate::routes::types::{SwapHistoryParams, SwapHistoryResponse};
use crate::services::swaps_service::fetch_swaps_history;
use crate::{db::connection::MongoDB, models::swap_history_model::SwapHistoryInterval};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/swaps")]
pub async fn handle_swaps_history(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<SwapHistoryParams>,
) -> impl Responder {
    let pagination_params = match QueryParser::new(&query.common, 400) {
        Ok(params) => params,
        Err(response) => return response,
    };

    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !SwapHistoryInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }

    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };
    let interval_str = query.interval.as_deref().unwrap_or("hour");
    match fetch_swaps_history(&mongo_db, pagination_params, interval_str, sort_by, order).await {
        Ok((meta, intervals)) => HttpResponse::Ok().json(SwapHistoryResponse { meta, intervals }),
        Err(error_message) => HttpResponse::InternalServerError().body(error_message),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(handle_swaps_history);
}
