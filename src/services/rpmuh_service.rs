use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};

use crate::db::connection::MongoDB;
use crate::helpers::query_parser::QueryParser;
use crate::helpers::time_intervals::interval_to_seconds;
use crate::models::rptmuh_model::{RpmuHistoryInterval, RpmuHistoryResponse};
use crate::routes::types::RpmuHistoryMeta;

pub async fn fetch_rpmuh_data(
    mongo_db: &MongoDB,
    pagination_params: QueryParser,
    interval_str: &str,
    sort_by: String,
    order: i32,
) -> Result<(RpmuHistoryMeta, Vec<RpmuHistoryInterval>), String> {
    let skip = pagination_params.skip();
    let filter = pagination_params.date_filter();

    let mut sort_doc = doc! {};
    sort_doc.insert(sort_by.clone(), order);
    let interval_seconds = interval_to_seconds(interval_str);
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
                "count": { "$last": "$count" },
                "units": { "$last": "$units" },
                "startTime": { "$first": "$startTime" },
                "endTime": { "$last": "$endTime" }
            }
        },
        doc! { "$skip": skip },
        doc! { "$limit": pagination_params.count },
    ];

    // Fetch the data from MongoDB
    match mongo_db.members_history.aggregate(pipeline).await {
        Ok(cursor) => {
            let results: Vec<RpmuHistoryInterval> = cursor
                .try_collect::<Vec<Document>>()
                .await
                .unwrap_or_else(|_| Vec::new())
                .into_iter()
                .map(|doc| mongodb::bson::from_document(doc).unwrap())
                .collect();

            if results.is_empty() {
                return Err("No data found for the given parameters.".to_string());
            }

            // Calculate the meta values based on the first and last records
            let start_count = results
                .first()
                .map_or("0".to_string(), |r| r.count.to_string());
            let end_count = results
                .last()
                .map_or("0".to_string(), |r| r.count.to_string());
            let start_units = results
                .first()
                .map_or("0".to_string(), |r| r.units.to_string());
            let end_units = results
                .last()
                .map_or("0".to_string(), |r| r.units.to_string());

            let start_time = results
                .first()
                .map_or("0".to_string(), |r| r.start_time.to_string());
            let end_time = results
                .last()
                .map_or("0".to_string(), |r| r.end_time.to_string());

            let has_next_page = results.len() as i64 == pagination_params.count;

            let meta = RpmuHistoryMeta {
                end_count,
                end_time,
                end_units,
                start_count,
                start_time,
                start_units,
                current_page: pagination_params.page,
                count: results.len() as i64,
                has_next_page,
            };

            Ok((meta, results))
        }
        Err(e) => Err(format!("Error fetching data: {}", e)),
    }
}

pub async fn update_rpmuh_data(
    mongo_db: MongoDB,
    from: f64,
    to: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    if from >= to {
        return Err("Invalid time range: 'from' should be less than 'to'".into());
    }

    let count = 1;
    let url: String = format!(
        "https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count={}&from={}&to={}",
        count, from, to
    );
    println!("Fetching URL: {}", &url);

    match reqwest::get(&url).await {
        Ok(response) => match response.json::<RpmuHistoryResponse>().await {
            Ok(resp) => {
                let intervals: Vec<RpmuHistoryInterval> = resp.intervals;
                let result = mongo_db
                    .members_history
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
