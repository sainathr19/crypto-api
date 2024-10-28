use serde::{Deserialize, Serialize};

use crate::models::{
    depth_history_model::{DepthHistoryInterval, DepthHistoryMeta},
    earning_history_model::EarningHistoryInterval,
    rptmuh_model::RpmuHistoryInterval,
    swap_history_model::SwapHistoryInterval,
};

#[derive(Deserialize)]
pub struct SwapHistoryParams {
    #[serde(flatten)]
    pub common: CommonQueryParams,
    pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapHistoryMeta {
    pub current_page: i64,
    pub count: i64,
    pub has_next_page: bool,
}

#[derive(Serialize)]
pub struct SwapHistoryResponse {
    pub meta: SwapHistoryMeta,
    pub intervals: Vec<SwapHistoryInterval>,
}

#[derive(Deserialize)]
pub struct CommonQueryParams {
    pub page: Option<String>,
    pub count: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Deserialize)]
pub struct RpmuHistoryQuery {
    #[serde(flatten)]
    pub common: CommonQueryParams,
    pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RpmuHistoryMeta {
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
pub struct RpmuHistoryResponse {
    pub meta: RpmuHistoryMeta,
    pub intervals: Vec<RpmuHistoryInterval>,
}
#[derive(Deserialize)]
pub struct EarningHistoryParams {
    #[serde(flatten)]
    pub common: CommonQueryParams,
    pub interval: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub pool: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct EarningHistoryFlattenMeta {
    pub count: i64,
    pub page: i64,
    pub has_next_page: bool,
}
#[derive(Deserialize, Serialize)]
pub struct EarningHistoryResponse {
    pub meta: EarningHistoryFlattenMeta,
    pub intervals: Vec<EarningHistoryInterval>,
}

#[derive(Deserialize)]
pub struct DepthHistoryParams {
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
pub struct DepthsHistoryMeta {
    #[serde(flatten)]
    pub meta: DepthHistoryMeta,
    pub current_page: i64,
    pub count: i64,
    pub has_next_page: bool,
}
#[derive(Serialize)]
pub struct DepthHistoryResponse {
    pub meta: DepthsHistoryMeta,
    pub intervals: Vec<DepthHistoryInterval>,
}
