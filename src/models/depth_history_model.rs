use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistoryInterval {
    pub asset_depth: f64,
    pub asset_price: f64,
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: f64,
    pub end_time: f64,
    pub liquidity_units: f64,
    pub luvi: f64,
    pub members_count: f64,
    pub rune_depth: f64,
    pub start_time: f64,
    pub synth_supply: f64,
    pub synth_units: f64,
    pub units: f64,
}
impl DepthHistoryInterval {
    pub fn get_feilds() -> Vec<&'static str> {
        vec![
            "assetDepth",
            "assetPrice",
            "assetPriceUSD",
            "endTime",
            "liquidityUnits",
            "luvi",
            "membersCount",
            "runeDepth",
            "startTime",
            "synthSupply",
            "synthUnits",
            "units",
        ]
    }
    pub fn has_field(field: String) -> bool {
        Self::get_feilds().contains(&field.as_str())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistoryMeta {
    pub end_asset_depth: f64,
    pub end_lp_units: f64,
    pub end_member_count: f64,
    pub end_rune_depth: f64,
    pub end_synth_units: f64,
    pub end_time: f64,
    pub luvi_increase: f64,
    pub price_shift_loss: f64,
    pub start_asset_depth: f64,
    pub start_lp_units: f64,
    pub start_member_count: f64,
    pub start_rune_depth: f64,
    pub start_synth_units: f64,
    pub start_time: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthHistoryResponse {
    pub intervals: Vec<DepthHistoryInterval>,
    pub meta: DepthHistoryMeta,
}
