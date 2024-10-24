use std::error::Error;

use chrono::{Duration, Utc};

use crate::{
    db::connection::MongoDB,
    services::{
        depths_service::update_depths_data, earnings_service::update_earnings_history,
        rpmuh_service::update_rpmuh_data, swaps_service::update_swaps_history,
    },
};

pub async fn start_scheduler(mongo_db: MongoDB) -> Result<(), Box<dyn Error>> {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));

    loop {
        interval.tick().await;
        println!("Fetching Latest Data");
        if let Err(e) = pull_latest_data(mongo_db.clone()).await {
            eprintln!("Error pulling latest data: {}", e);
        }
    }
}

async fn pull_latest_data(mongo_db: MongoDB) -> Result<(), Box<dyn Error>> {
    let to = Utc::now().timestamp() as f64;
    let from = (Utc::now() - Duration::hours(1)).timestamp() as f64;

    if let Err(e) = update_depths_data(mongo_db.clone(), String::from("BTC.BTC"), from, to).await {
        eprintln!("Error fetching depth history: {:?}", e);
    }

    if let Err(e) = update_earnings_history(mongo_db.clone(), from, to).await {
        eprintln!("Error fetching earnings history: {:?}", e);
    }

    if let Err(e) = update_rpmuh_data(mongo_db.clone(), from, to).await {
        eprintln!("Error fetching members history: {:?}", e);
    }

    if let Err(e) = update_swaps_history(mongo_db.clone(), from, to).await {
        eprintln!("Error fetching swap history: {:?}", e);
    }

    Ok(())
}
