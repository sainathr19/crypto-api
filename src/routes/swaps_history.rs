use crate::helpers::query_parser::QueryParser;
use crate::services::swaps_service::fetch_swaps_data;
use crate::{db::connection::MongoDB, models::swaps_history::SwapsHistoryInterval};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use super::members_history::CommonQueryParams;

#[derive(Debug, Deserialize)]
struct SwapsHistoryParams {
    #[serde(flatten)]
    common: CommonQueryParams,
    // interval: Option<String>,
    sort_by: Option<String>,
    order: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistoryMeta {
    pub current_page: i64,
    pub count: i64,
    pub has_next_page: bool,
}

#[derive(Serialize)]
struct SwapsHistoryResponse {
    meta: SwapsHistoryMeta,
    intervals: Vec<SwapsHistoryInterval>,
}

#[get("/swaps")]
pub async fn get_swaps_data(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<SwapsHistoryParams>,
) -> impl Responder {
    let pagination_params = match QueryParser::new(&query.common) {
        Ok(params) => params,
        Err(response) => return response,
    };

    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !SwapsHistoryInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }

    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };
    match fetch_swaps_data(&mongo_db, pagination_params, sort_by, order).await {
        Ok((meta, intervals)) => HttpResponse::Ok().json(SwapsHistoryResponse { meta, intervals }),
        Err(error_message) => HttpResponse::InternalServerError().body(error_message),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(get_swaps_data);
}
