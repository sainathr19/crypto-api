use crate::helpers::query_parser::QueryParser;
use crate::routes::types::{MembersHistoryQuery, MembersResponse};
use crate::services::rpmuh_service::fetch_member_data;
use crate::{db::connection::MongoDB, models::rptmuh_model::MembersAndUnitsInterval};
use actix_web::{get, web, HttpResponse, Responder};

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

    let interval_str = query.interval.as_ref().unwrap().as_str();
    match fetch_member_data(&mongo_db, pagination_params, &interval_str, sort_by, order).await {
        Ok((meta, intervals)) => HttpResponse::Ok().json(MembersResponse { meta, intervals }),
        Err(error_message) => HttpResponse::InternalServerError().body(error_message),
    }
}

pub fn init(config: &mut web::ServiceConfig) {
    config.service(get_member_data);
}
