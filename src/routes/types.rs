use serde::{Deserialize, Serialize};

use crate::models::{
    depths_history::{DepthsHistoryInterval, DepthsHistoryMeta},
    earnings_history::EarningsHistoryInterval,
    rptmuh_model::MembersAndUnitsInterval,
    swaps_history::SwapsHistoryInterval,
};

#[derive(Debug, Deserialize)]
pub struct SwapsHistoryParams {
    #[serde(flatten)]
    pub common: CommonQueryParams,
    //pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistoryMeta {
    pub current_page: i64,
    pub count: i64,
    pub has_next_page: bool,
}

#[derive(Serialize)]
pub struct SwapsHistoryResponse {
    pub meta: SwapsHistoryMeta,
    pub intervals: Vec<SwapsHistoryInterval>,
}

#[derive(Deserialize, Debug)]
pub struct CommonQueryParams {
    pub page: Option<String>,
    pub count: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MembersHistoryQuery {
    #[serde(flatten)]
    pub common: CommonQueryParams,
    //pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
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
pub struct MembersResponse {
    pub meta: MembersAndUnitsMeta,
    pub intervals: Vec<MembersAndUnitsInterval>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EarningsHistoryMeta {
    pub count: i64,
    pub page: i64,
    pub hasNextPage: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EarningsHistoryResponse {
    pub meta: EarningsHistoryMeta,
    pub intervals: Vec<EarningsHistoryInterval>,
}

#[derive(Debug, Deserialize)]
pub struct DepthsHistoryParams {
    #[serde(flatten)]
    pub common: CommonQueryParams,
    pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub min_depth: Option<f64>,
    pub max_depth: Option<f64>,
    pub liquidity_gt: Option<f64>,
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
pub struct Response {
    pub meta: Meta,
    pub intervals: Vec<DepthsHistoryInterval>,
}
