use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpmuHistoryInterval {
    pub count: f64,
    pub end_time: f64,
    pub start_time: f64,
    pub units: f64,
}
impl RpmuHistoryInterval {
    pub fn field_names() -> Vec<&'static str> {
        vec!["count", "endTime", "startTime", "units"]
    }

    pub fn has_field(field: String) -> bool {
        Self::field_names().contains(&field.as_str())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpmuHistoryMeta {
    pub end_count: f64,
    pub end_time: f64,
    pub end_units: f64,
    pub start_count: f64,
    pub start_time: f64,
    pub start_units: f64,
}

#[derive(Serialize, Deserialize)]
pub struct RpmuHistoryResponse {
    pub intervals: Vec<RpmuHistoryInterval>,
    pub meta: RpmuHistoryMeta,
}
