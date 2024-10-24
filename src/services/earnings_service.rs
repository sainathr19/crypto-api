use actix_web::web;
use futures_util::TryStreamExt;
use mongodb::bson::{doc, Document};

use crate::db::connection::MongoDB;
use crate::helpers::query_parser::QueryParser;
use crate::helpers::time_intervals::interval_to_seconds;
use crate::models::earnings_history::{EarningsHistoryInterval, EarningsHistoryResponse};
use crate::routes::earnings_history::EarningsHistoryMeta;

pub async fn get_earnings_history_data(
    mongo_db: &web::Data<MongoDB>,
    pagination_params: QueryParser,
    interval_str: &str,
    sort_by: String,
    order: i32,
) -> Result<(EarningsHistoryMeta, Vec<EarningsHistoryInterval>), String> {
    let interval_seconds = interval_to_seconds(interval_str);
    let skip = pagination_params.skip();
    let filter = pagination_params.date_filter();
    let mut sort_doc = doc! {};
    sort_doc.insert(sort_by.clone(), order);

    let pipeline = vec![
        doc! { "$match": filter },
        doc! { "$sort": sort_doc },
        doc! { "$skip": skip },
        doc! { "$limit": pagination_params.count },
    ];

    match mongo_db.earnings_history.aggregate(pipeline).await {
        Ok(cursor) => {
            let results: Vec<EarningsHistoryInterval> = cursor
                .try_collect::<Vec<Document>>()
                .await
                .unwrap_or_else(|_| Vec::new())
                .into_iter()
                .map(|doc| mongodb::bson::from_document(doc).unwrap())
                .collect();

            if results.is_empty() {
                return Err("No data found for the given parameters.".to_string());
            }

            let meta = EarningsHistoryMeta {
                count: results.len() as i64,
                page: pagination_params.page,
                hasNextPage: results.len() as i64 == pagination_params.count,
            };

            Ok((meta, results))
        }
        Err(e) => Err(format!("Error fetching data: {}", e)),
    }
}

pub async fn update_earnings_data(
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
        Ok(response) => match response.json::<EarningsHistoryResponse>().await {
            Ok(resp) => {
                let intervals: Vec<EarningsHistoryInterval> = resp.intervals;
                let result = mongo_db
                    .earnings_history
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
                eprintln!("Failed to deserialize response: {:?}", e);
                Err(e.into())
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch data: {:?}", e);
            Err(e.into())
        }
    }
}
