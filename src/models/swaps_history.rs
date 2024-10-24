use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistoryInterval {
    pub average_slip: f64,
    pub end_time: f64,
    pub from_trade_average_slip: f64,
    pub from_trade_count: f64,
    pub from_trade_fees: f64,
    pub from_trade_volume: f64,
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: f64,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    pub start_time: i64,
    pub synth_mint_average_slip: f64,
    pub synth_mint_count: f64,
    pub synth_mint_fees: f64,
    pub synth_mint_volume: f64,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: f64,
    pub synth_redeem_average_slip: f64,
    pub synth_redeem_count: f64,
    pub synth_redeem_fees: f64,
    pub synth_redeem_volume: f64,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,
    pub to_asset_average_slip: f64,
    pub to_asset_count: f64,
    pub to_asset_fees: f64,
    pub to_asset_volume: f64,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,
    pub to_rune_average_slip: f64,
    pub to_rune_count: f64,
    pub to_rune_fees: f64,
    pub to_rune_volume: f64,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,
    pub to_trade_average_slip: f64,
    pub to_trade_count: f64,
    pub to_trade_fees: f64,
    pub to_trade_volume: f64,
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: f64,
    pub total_count: f64,
    pub total_fees: f64,
    pub total_volume: f64,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: f64,
}

impl SwapsHistoryInterval {
    pub fn field_names() -> Vec<&'static str> {
        vec![
            "averageSlip",
            "endTime",
            "fromTradeAverageSlip",
            "fromTradeCount",
            "fromTradeFees",
            "fromTradeVolume",
            "fromTradeVolumeUSD",
            "runePriceUSD",
            "startTime",
            "synthMintAverageSlip",
            "synthMintCount",
            "synthMintFees",
            "synthMintVolume",
            "synthMintVolumeUSD",
            "synthRedeemAverageSlip",
            "synthRedeemCount",
            "synthRedeemFees",
            "synthRedeemVolume",
            "synthRedeemVolumeUSD",
            "toAssetAverageSlip",
            "toAssetCount",
            "toAssetFees",
            "toAssetVolume",
            "toAssetVolumeUSD",
            "toRuneAverageSlip",
            "toRuneCount",
            "toRuneFees",
            "toRuneVolume",
            "toRuneVolumeUSD",
            "toTradeAverageSlip",
            "toTradeCount",
            "toTradeFees",
            "toTradeVolume",
            "toTradeVolumeUSD",
            "totalCount",
            "totalFees",
            "totalVolume",
            "totalVolumeUSD",
        ]
    }

    pub fn has_field(field: String) -> bool {
        Self::field_names().contains(&field.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapsHistoryMeta {
    pub average_slip: f64,
    pub end_time: f64,
    pub from_trade_average_slip: f64,
    pub from_trade_count: f64,
    pub from_trade_fees: f64,
    pub from_trade_volume: f64,
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: f64,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: f64,
    pub start_time: i64,
    pub synth_mint_average_slip: f64,
    pub synth_mint_count: f64,
    pub synth_mint_fees: f64,
    pub synth_mint_volume: f64,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: f64,
    pub synth_redeem_average_slip: f64,
    pub synth_redeem_count: f64,
    pub synth_redeem_fees: f64,
    pub synth_redeem_volume: f64,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: f64,
    pub to_asset_average_slip: f64,
    pub to_asset_count: f64,
    pub to_asset_fees: f64,
    pub to_asset_volume: f64,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: f64,
    pub to_rune_average_slip: f64,
    pub to_rune_count: f64,
    pub to_rune_fees: f64,
    pub to_rune_volume: f64,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: f64,
    pub to_trade_average_slip: f64,
    pub to_trade_count: f64,
    pub to_trade_fees: f64,
    pub to_trade_volume: f64,
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: f64,
    pub total_count: f64,
    pub total_fees: f64,
    pub total_volume: f64,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapsHistoryResponse {
    pub intervals: Vec<SwapsHistoryInterval>,
    pub meta: SwapsHistoryMeta,
}
