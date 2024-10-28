use crate::db::connection::MongoDB;
use crate::helpers::query_parser::QueryParser;
use crate::helpers::time_intervals::interval_to_seconds;
use crate::models::swap_history_model::{SwapHistoryInterval, SwapHistoryResponse};
use crate::routes::types::SwapHistoryMeta;

use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::AggregateOptions;
use reqwest;
use std::time::Duration;
use tokio::time::sleep;

pub async fn fetch_swaps_history(
    mongo_db: &MongoDB,
    pagination_params: QueryParser,
    interval_str: &str,
    sort_by: String,
    order: i32,
) -> Result<(SwapHistoryMeta, Vec<SwapHistoryInterval>), String> {
    let skip = pagination_params.skip();
    let filter = pagination_params.date_filter();
    let interval_seconds = interval_to_seconds(interval_str);
    let mut sort_doc = doc! {};
    sort_doc.insert(sort_by, order);
    let pipeline = vec![
        doc! { "$match": filter },
        doc! { "$sort": sort_doc },
        doc! {
            "$group": {
                "_id": {
                    "$toDate": {
                        "$subtract": [
                            "$startTime",
                            { "$mod": ["$startTime", interval_seconds] }
                        ]
                    }
                },
                "toAssetCount": { "$last": "$toAssetCount" },
                "toRuneCount": { "$last": "$toRuneCount" },
                "toTradeCount": { "$last": "$toTradeCount" },
                "fromTradeCount": { "$last": "$fromTradeCount" },
                "synthMintCount": { "$last": "$synthMintCount" },
                "synthRedeemCount": { "$last": "$synthRedeemCount" },
                "totalCount": { "$last": "$totalCount" },
                "toAssetVolume": { "$last": "$toAssetVolume" },
                "toRuneVolume": { "$last": "$toRuneVolume" },
                "toTradeVolume": { "$last": "$toTradeVolume" },
                "fromTradeVolume": { "$last": "$fromTradeVolume" },
                "synthMintVolume": { "$last": "$synthMintVolume" },
                "synthRedeemVolume": { "$last": "$synthRedeemVolume" },
                "totalVolume": { "$last": "$totalVolume" },
                "toAssetVolumeUSD": { "$last": "$toAssetVolumeUSD" },
                "toRuneVolumeUSD": { "$last": "$toRuneVolumeUSD" },
                "toTradeVolumeUSD": { "$last": "$toTradeVolumeUSD" },
                "fromTradeVolumeUSD": { "$last": "$fromTradeVolumeUSD" },
                "synthMintVolumeUSD": { "$last": "$synthMintVolumeUSD" },
                "synthRedeemVolumeUSD": { "$last": "$synthRedeemVolumeUSD" },
                "totalVolumeUSD": { "$last": "$totalVolumeUSD" },
                "toAssetFees": { "$last": "$toAssetFees" },
                "toRuneFees": { "$last": "$toRuneFees" },
                "toTradeFees": { "$last": "$toTradeFees" },
                "fromTradeFees": { "$last": "$fromTradeFees" },
                "synthMintFees": { "$last": "$synthMintFees" },
                "synthRedeemFees": { "$last": "$synthRedeemFees" },
                "totalFees": { "$last": "$totalFees" },
                "toAssetAverageSlip": { "$last": "$toAssetAverageSlip" },
                "toRuneAverageSlip": { "$last": "$toRuneAverageSlip" },
                "toTradeAverageSlip": { "$last": "$toTradeAverageSlip" },
                "fromTradeAverageSlip": { "$last": "$fromTradeAverageSlip" },
                "synthMintAverageSlip": { "$last": "$synthMintAverageSlip" },
                "synthRedeemAverageSlip": { "$last": "$synthRedeemAverageSlip" },
                "averageSlip": { "$last": "$averageSlip" },
                "runePriceUSD": { "$last": "$runePriceUSD" },
                "startTime": { "$first": "$startTime" },
                "endTime": { "$last": "$endTime" }
            }
        },
        doc! { "$skip": skip },
        doc! { "$limit": pagination_params.count },
    ];
    let aggregate_options = AggregateOptions::builder().allow_disk_use(true).build();
    match mongo_db
        .swaps_history
        .aggregate(pipeline, aggregate_options)
        .await
    {
        Ok(cursor) => {
            let results: Vec<SwapHistoryInterval> = cursor
                .try_collect::<Vec<Document>>()
                .await
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(|doc| mongodb::bson::from_document(doc).unwrap())
                .collect();
            let has_next_page = results.len() as i64 == pagination_params.count;
            let meta = SwapHistoryMeta {
                current_page: pagination_params.page,
                count: results.len() as i64,
                has_next_page,
            };

            Ok((meta, results))
        }
        Err(e) => Err(format!("Error fetching data: {}", e)),
    }
}

pub async fn update_swaps_history(
    mongo_db: MongoDB,
    from: f64,
    to: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if from >= to {
        return Err("Invalid time range: 'from' should be less than 'to'".into());
    }

    let count = 400;
    let mut start_time = from;

    while start_time < to {
        let url: String = format!(
            "https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count={}&from={}",
            count, start_time
        );
        println!("Fetching URL: {}", &url);

        match reqwest::get(&url).await {
            Ok(response) => match response.json::<SwapHistoryResponse>().await {
                Ok(resp) => {
                    start_time = resp.meta.end_time;
                    if start_time >= to {
                        println!("Reached the specified end time, stopping fetch.");
                        break;
                    }

                    let intervals: Vec<SwapHistoryInterval> = resp.intervals;
                    let result = mongo_db
                        .swaps_history
                        .insert_many(intervals, None)
                        .await
                        .map_err(|e| format!("Error Inserting Data into DB: {:?}", e))?;

                    println!(
                        "Successfully inserted {} intervals, now starting from {}",
                        result.inserted_ids.len(),
                        start_time
                    );
                }
                Err(e) => {
                    println!("Failed to deserialize response: {:?}", e);
                    return Err(e.into());
                }
            },
            Err(e) => {
                println!("Failed to fetch data: {:?}", e);
                return Err(e.into());
            }
        }

        sleep(Duration::from_secs(3)).await;
    }

    Ok(())
}
