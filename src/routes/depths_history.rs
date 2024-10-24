use crate::helpers::query_parser::QueryParser;
use crate::models::depths_history::DepthsHistoryMeta;
use crate::routes::members_history::CommonQueryParams;
use crate::services::depths_service::get_depths_history_data;
use crate::{db::connection::MongoDB, models::depths_history::DepthsHistoryInterval};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct DepthsHistoryParams {
    #[serde(flatten)]
    common: CommonQueryParams,
    interval: Option<String>,
    sort_by: Option<String>,
    order: Option<String>,
    min_depth: Option<f64>,
    max_depth: Option<f64>,
    liquidity_gt: Option<f64>,
}

#[get("/depths")]
pub async fn fetch_depths_history(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<DepthsHistoryParams>,
) -> impl Responder {
    let pagination_params = match QueryParser::new(&query.common) {
        Ok(params) => params,
        Err(response) => return response,
    };

    let interval_str = query.interval.as_deref().unwrap_or("hour");
    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !DepthsHistoryInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }

    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };

    let max_depth: Option<f64> = query.max_depth;
    let min_depth: Option<f64> = query.min_depth;
    let liquidity_gt: Option<f64> = query.liquidity_gt;

    match get_depths_history_data(
        &mongo_db,
        pagination_params,
        interval_str,
        sort_by,
        order,
        max_depth,
        min_depth,
        liquidity_gt,
    )
    .await
    {
        Ok((meta, intervals)) => HttpResponse::Ok().json(Response { meta, intervals }),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(fetch_depths_history);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(flatten)]
    pub meta: DepthsHistoryMeta,
    pub current_page: i64,
    pub count: i64,
    pub has_next_page: bool,
}
#[derive(Serialize)]
struct Response {
    meta: Meta,
    intervals: Vec<DepthsHistoryInterval>,
}
