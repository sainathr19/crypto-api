use dotenv::dotenv;
use mongodb::{error::Error, Client, Collection};
use std::env;

use crate::models::{
    depths_history::DepthsHistoryInterval, earnings_history::EarningsHistoryInterval,
    rptmuh_model::MembersAndUnitsInterval, swaps_history::SwapsHistoryInterval,
};

#[derive(Clone)]
pub struct MongoDB {
    pub depths_history: Collection<DepthsHistoryInterval>,
    pub members_history: Collection<MembersAndUnitsInterval>,
    pub swaps_history: Collection<SwapsHistoryInterval>,
    pub earnings_history: Collection<EarningsHistoryInterval>,
}
impl MongoDB {
    pub async fn init() -> Result<Self, Error> {
        dotenv().ok();
        let mongo_uri: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let client: Client = Client::with_uri_str(mongo_uri)
            .await
            .expect("Unable to connect with MongoDB");
        let db = client.database("masterdb");
        let depths_history: Collection<DepthsHistoryInterval> = db.collection("depths_history");
        let members_history: Collection<MembersAndUnitsInterval> = db.collection("members_history");
        let swaps_history: Collection<SwapsHistoryInterval> = db.collection("swaps_history");
        let earnings_history: Collection<EarningsHistoryInterval> =
            db.collection("earnings_history");
        Ok(MongoDB {
            depths_history,
            members_history,
            swaps_history,
            earnings_history,
        })
    }
}
