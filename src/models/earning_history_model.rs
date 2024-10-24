use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningHistoryInterval {
    pub start_time: f64,
    pub end_time: f64,
    pub liquidity_fees: f64,
    pub block_rewards: f64,
    pub earnings: f64,
    pub bonding_earnings: f64,
    pub liquidity_earnings: f64,
    pub avg_node_count: f64,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    pub pools: Vec<EarningHistoryPool>,
}
impl EarningHistoryInterval {
    pub fn field_names() -> Vec<&'static str> {
        vec![
            "startTime",
            "endTime",
            "liquidityFees",
            "blockRewards",
            "earnings",
            "bondingEarnings",
            "liquidityEarnings",
            "avgNodeCount",
            "runePriceUSD",
            "pools",
        ]
    }

    pub fn has_field(field: String) -> bool {
        Self::field_names().contains(&field.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningHistoryPool {
    pub pool: String,
    pub asset_liquidity_fees: f64,
    pub rune_liquidity_fees: f64,
    pub total_liquidity_fees_rune: f64,
    pub saver_earning: f64,
    pub rewards: f64,
    pub earnings: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningHistoryMeta {
    pub start_time: f64,
    pub end_time: f64,
    pub liquidity_fees: f64,
    pub block_rewards: f64,
    pub earnings: f64,
    pub bonding_earnings: f64,
    pub liquidity_earnings: f64,
    pub avg_node_count: f64,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    pub pools: Vec<EarningHistoryPool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningHistoryResponse {
    pub meta: EarningHistoryMeta,
    pub intervals: Vec<EarningHistoryInterval>,
}
