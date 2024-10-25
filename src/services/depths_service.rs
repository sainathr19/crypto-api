use crate::db::connection::MongoDB;
use crate::helpers::query_parser::QueryParser;
use crate::helpers::time_intervals::interval_to_seconds;
use crate::models::depth_history_model::{
    DepthHistoryInterval, DepthHistoryMeta, DepthHistoryResponse,
};
use crate::routes::types::DepthsHistoryMeta;
use actix_web::web;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};

pub async fn fetch_depths_history(
    mongo_db: &web::Data<MongoDB>,
    pagination_params: QueryParser,
    interval_str: &str,
    sort_by: String,
    order: i32,
    max_depth: Option<f64>,
    min_depth: Option<f64>,
    liquidity_gt: Option<f64>,
) -> Result<(DepthsHistoryMeta, Vec<DepthHistoryInterval>), String> {
    let interval_seconds = interval_to_seconds(interval_str);
    let skip = pagination_params.skip();
    let mut filter = pagination_params.date_filter();
    let mut sort_doc = doc! {};
    sort_doc.insert(sort_by.clone(), order);

    if let Some(min_depth) = min_depth {
        filter.insert("assetDepth", doc! { "$gte": min_depth });
    }

    if let Some(max_depth) = max_depth {
        filter.insert("assetDepth", doc! { "$lte": max_depth });
    }

    if let Some(liquidity_gt) = liquidity_gt {
        filter.insert("liquidityUnits", doc! { "$gte": liquidity_gt });
    }
    let pipeline = vec![
        doc! { "$match": filter },
        doc! { "$group": {
            "_id": {
                "$toDate": {
                    "$subtract": ["$startTime", { "$mod": ["$startTime", interval_seconds] }]
                }
            },
            "assetDepth": { "$last": "$assetDepth" },
            "runeDepth": { "$last": "$runeDepth" },
            "assetPrice": { "$last": "$assetPrice" },
            "assetPriceUSD": { "$last": "$assetPriceUSD" },
            "liquidityUnits": { "$last": "$liquidityUnits" },
            "membersCount": { "$last": "$membersCount" },
            "synthUnits": { "$last": "$synthUnits" },
            "synthSupply": { "$last": "$synthSupply" },
            "units": { "$last": "$units" },
            "luvi": { "$last": "$luvi" },
            "startTime": { "$first": "$startTime" },
            "endTime": { "$last": "$endTime" }
        }},
        doc! { "$project": {
            "_id": 0,
            "startTime": 1,
            "endTime": 1,
            "assetDepth": 1,
            "runeDepth": 1,
            "assetPrice": 1,
            "assetPriceUSD": 1,
            "liquidityUnits": 1,
            "membersCount": 1,
            "synthUnits": 1,
            "synthSupply": 1,
            "units": 1,
            "luvi": 1
        }},
        doc! { "$sort": sort_doc },
        doc! { "$skip": skip },
        doc! { "$limit": pagination_params.count },
    ];
    match mongo_db.depths_history.aggregate(pipeline).await {
        Ok(cursor) => {
            let results: Vec<DepthHistoryInterval> = cursor
                .try_collect::<Vec<Document>>()
                .await
                .unwrap_or_else(|_| Vec::new())
                .into_iter()
                .map(|doc| mongodb::bson::from_document(doc).unwrap())
                .collect();

            if results.is_empty() {
                return Err("No data found for the given parameters.".to_string());
            }

            let start = &results.first().unwrap();
            let end = &results.last().unwrap();
            let depths_meta = DepthHistoryMeta {
                end_asset_depth: end.asset_depth,
                end_lp_units: end.liquidity_units,
                end_member_count: end.members_count,
                end_rune_depth: end.rune_depth,
                end_synth_units: end.synth_units,
                end_time: end.end_time,
                luvi_increase: 0.0,
                price_shift_loss: 0.0,
                start_asset_depth: start.asset_depth,
                start_lp_units: start.liquidity_units,
                start_member_count: start.members_count,
                start_rune_depth: start.rune_depth,
                start_synth_units: start.synth_units,
                start_time: start.start_time,
            };
            let meta = DepthsHistoryMeta {
                meta: depths_meta,
                current_page: pagination_params.page,
                count: results.len() as i64,
                has_next_page: results.len() as i64 == pagination_params.count,
            };

            Ok((meta, results))
        }
        Err(e) => Err(format!("Error fetching data: {}", e)),
    }
}

pub async fn update_depths_data(
    mongo_db: MongoDB,
    pool_name: String,
    from: f64,
    to: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if from >= to {
        return Err("Invalid time range: 'from' should be less than 'to'".into());
    }

    let count = 400;

    let url: String = format!(
        "https://midgard.ninerealms.com/v2/history/depths/{}?interval=hour&count={}&from={}&to={}",
        pool_name, count, from, to
    );
    println!("Fetching URL: {}", &url);
    match reqwest::get(&url).await {
        Ok(response) => match response.json::<DepthHistoryResponse>().await {
            Ok(resp) => {
                let intervals: Vec<DepthHistoryInterval> = resp.intervals;
                let result = mongo_db
                    .depths_history
                    .insert_many(intervals)
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
