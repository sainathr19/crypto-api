use crate::helpers::query_parser::QueryParser;
use crate::routes::types::{DepthHistoryParams, DepthHistoryResponse};
use crate::services::depths_service::fetch_depths_history;
use crate::{db::connection::MongoDB, models::depth_history_model::DepthHistoryInterval};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/depths")]
pub async fn handle_depths_history(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<DepthHistoryParams>,
) -> impl Responder {
    let query_params = match QueryParser::new(&query.common) {
        Ok(params) => params,
        Err(response) => return response,
    };

    let interval_str = query.interval.as_deref().unwrap_or("hour");
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !DepthHistoryInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }

    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };

    let max_depth: Option<f64> = query.max_depth;
    let min_depth: Option<f64> = query.min_depth;
    let liquidity_gt: Option<f64> = query.liquidity_gt;

    match fetch_depths_history(
        &mongo_db,
        query_params,
        interval_str,
        sort_by,
        order,
        max_depth,
        min_depth,
        liquidity_gt,
    )
    .await
    {
        Ok((meta, intervals)) => HttpResponse::Ok().json(DepthHistoryResponse { meta, intervals }),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(handle_depths_history);
}
