use dotenv::dotenv;
use mongodb::{error::Error, Client, Collection};
use std::env;

use crate::models::{
    depth_history_model::DepthHistoryInterval, earning_history_model::EarningHistoryInterval,
    rptmuh_model::RpmuHistoryInterval, swap_history_model::SwapHistoryInterval,
};

#[derive(Clone)]
pub struct MongoDB {
    pub depths_history: Collection<DepthHistoryInterval>,
    pub members_history: Collection<RpmuHistoryInterval>,
    pub swaps_history: Collection<SwapHistoryInterval>,
    pub earnings_history: Collection<EarningHistoryInterval>,
}
impl MongoDB {
    pub async fn init() -> Result<Self, Error> {
        dotenv().ok();
        let mongo_uri: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client: Client = Client::with_uri_str(mongo_uri)
            .await
            .expect("Unable to connect with MongoDB");
        let db = client.database("masterdb");
        let depths_history: Collection<DepthHistoryInterval> = db.collection("depths_history");
        let members_history: Collection<RpmuHistoryInterval> = db.collection("members_history");
        let swaps_history: Collection<SwapHistoryInterval> = db.collection("swaps_history");
        let earnings_history: Collection<EarningHistoryInterval> =
            db.collection("earnings_history");
        Ok(MongoDB {
            depths_history,
            members_history,
            swaps_history,
            earnings_history,
        })
    }
}
