use crate::helpers::query_parser::QueryParser;
use crate::services::rpmuh_service::fetch_member_data;
use crate::{db::connection::MongoDB, models::rptmuh_model::MembersAndUnitsInterval};
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CommonQueryParams {
    pub page: Option<String>,
    pub count: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MembersHistoryQuery {
    #[serde(flatten)]
    common: CommonQueryParams,
    // interval: Option<String>,
    sort_by: Option<String>,
    order: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MembersAndUnitsMeta {
    pub end_count: String,
    pub end_time: String,
    pub end_units: String,
    pub start_count: String,
    pub start_time: String,
    pub start_units: String,
    pub current_page: i64,
    pub count: i64,
    pub has_next_page: bool,
}

#[derive(Serialize)]
struct MembersResponse {
    meta: MembersAndUnitsMeta,
    intervals: Vec<MembersAndUnitsInterval>,
}

#[get("/history/runepool")]
pub async fn get_member_data(
    mongo_db: web::Data<MongoDB>,
    query: web::Query<MembersHistoryQuery>,
) -> impl Responder {
    let pagination_params = match QueryParser::new(&query.common) {
        Ok(params) => params,
        Err(response) => return response,
    };

    let sort_by = query
        .sort_by
        .clone()
        .unwrap_or_else(|| String::from("startTime"));

    if !MembersAndUnitsInterval::has_field(sort_by.clone()) {
        return HttpResponse::BadRequest().body("Invalid sort_by parameter.");
    }
    let order = match query.order.as_deref() {
        Some("asc") => 1,
        _ => -1,
    };
    match fetch_member_data(&mongo_db, pagination_params, sort_by, order).await {
        Ok((meta, intervals)) => HttpResponse::Ok().json(MembersResponse { meta, intervals }),
        Err(error_message) => HttpResponse::InternalServerError().body(error_message),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(get_member_data);
}