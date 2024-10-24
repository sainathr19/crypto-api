use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembersAndUnitsInterval {
    pub count: f64,
    pub end_time: f64,
    pub start_time: f64,
    pub units: f64,
}
impl MembersAndUnitsInterval {
    pub fn field_names() -> Vec<&'static str> {
        vec!["count", "endTime", "startTime", "units"]
    }

    pub fn has_field(field: String) -> bool {
        Self::field_names().contains(&field.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MembersAndUnitsMeta {
    pub end_count: f64,
    pub end_time: f64,
    pub end_units: f64,
    pub start_count: f64,
    pub start_time: f64,
    pub start_units: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembersAndUnitsResponse {
    pub intervals: Vec<MembersAndUnitsInterval>,
    pub meta: MembersAndUnitsMeta,
}
