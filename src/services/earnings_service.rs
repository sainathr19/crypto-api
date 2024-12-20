use crate::db::connection::MongoDB;
use crate::helpers::query_parser::QueryParser;
use crate::helpers::time_intervals::interval_to_seconds;
use crate::models::earning_history_model::{EarningHistoryInterval, EarningHistoryResponse};
use crate::routes::types::EarningHistoryFlattenMeta;
use actix_web::web;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::AggregateOptions;

pub async fn fetch_earnings_history(
    mongo_db: &web::Data<MongoDB>,
    pagination_params: QueryParser,
    interval_str: &str,
    sort_by: String,
    order: i32,
    pool_name: &str,
) -> Result<(EarningHistoryFlattenMeta, Vec<EarningHistoryInterval>), String> {
    let interval_seconds = interval_to_seconds(interval_str);
    let skip = pagination_params.skip();
    let filter = pagination_params.date_filter();
    let mut sort_doc = doc! {};
    sort_doc.insert(sort_by.clone(), order);
    let pipeline = vec![
        doc! { "$match": filter },
        doc! { "$sort": sort_doc },
        doc! {
            "$project": {
                "startTime": 1,
                "endTime": 1,
                "liquidityFees": 1,
                "blockRewards": 1,
                "earnings": 1,
                "bondingEarnings": 1,
                "liquidityEarnings": 1,
                "avgNodeCount": 1,
                "runePriceUSD": 1,
                "pools": {
                    "$cond": {
                        "if": { "$eq": [pool_name, "all"] },
                        "then": "$pools",
                        "else": {
                            "$filter": {
                                "input": "$pools",
                                "as": "pool",
                                "cond": { "$eq": ["$$pool.pool", pool_name] }
                            }
                        }
                    }
                }
            }
        },
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
                "startTime": { "$first": "$startTime" },
                "endTime": { "$last": "$endTime" },
                "liquidityFees": { "$last": "$liquidityFees" },
                "blockRewards": { "$last": "$blockRewards" },
                "earnings": { "$last": "$earnings" },
                "bondingEarnings": { "$last": "$bondingEarnings" },
                "liquidityEarnings": { "$last": "$liquidityEarnings" },
                "avgNodeCount": { "$last": "$avgNodeCount" },
                "runePriceUSD": { "$last": "$runePriceUSD" },
                "pools": { "$last": "$pools" }
            }
        },
        doc! { "$skip": skip },
        doc! { "$limit": pagination_params.count },
    ];
    let aggregate_options = AggregateOptions::builder().allow_disk_use(true).build();
    match mongo_db
        .earnings_history
        .aggregate(pipeline, aggregate_options)
        .await
    {
        Ok(cursor) => {
            let results: Vec<EarningHistoryInterval> = cursor
                .try_collect::<Vec<Document>>()
                .await
                .unwrap_or_else(|_| Vec::new())
                .into_iter()
                .map(|doc| mongodb::bson::from_document(doc).unwrap())
                .collect();

            if results.is_empty() {
                return Err("No data found for the given parameters.".to_string());
            }

            let meta = EarningHistoryFlattenMeta {
                count: results.len() as i64,
                page: pagination_params.page,
                has_next_page: results.len() as i64 == pagination_params.count,
            };

            Ok((meta, results))
        }
        Err(e) => Err(format!("Error fetching data: {}", e)),
    }
}

pub async fn update_earnings_history(
    mongo_db: MongoDB,
    from: f64,
    to: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if from >= to {
        return Err("Invalid time range: 'from' should be less than 'to'".into());
    }

    let count = 1;
    let url: String = format!(
        "https://midgard.ninerealms.com/v2/history/earnings?interval=hour&count={}&from={}&to={}",
        count, from, to
    );
    println!("Fetching URL: {}", &url);
    match reqwest::get(&url).await {
        Ok(response) => match response.json::<EarningHistoryResponse>().await {
            Ok(resp) => {
                let intervals: Vec<EarningHistoryInterval> = resp.intervals;
                let result = mongo_db
                    .earnings_history
                    .insert_many(intervals, None)
                    .await
                    .map_err(|e| format!("Error Inserting Data into DB: {:?}", e))?;

                println!(
                    "Successfully inserted {} intervals from {} to {}",
                    result.inserted_ids.len(),
                    from,
                    to
                );
                Ok(())
            }
            Err(e) => {
                println!("Failed to deserialize response: {:?}", e);
                Err(e.into())
            }
        },
        Err(e) => {
            println!("Failed to fetch data: {:?}", e);
            Err(e.into())
        }
    }
}
