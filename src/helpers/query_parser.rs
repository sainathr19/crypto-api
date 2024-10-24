use crate::helpers::time_formatter::parse_date;
use crate::routes::members_history::CommonQueryParams;
use actix_web::HttpResponse;
use chrono::Utc;
use mongodb::bson::doc;

#[derive(Debug)]
pub struct QueryParser {
    pub page: i64,
    pub count: i64,
    pub from: i64,
    pub to: i64,
}

impl QueryParser {
    pub fn new(query: &CommonQueryParams) -> Result<Self, HttpResponse> {
        let count = query
            .count
            .as_ref()
            .map(|c| c.parse::<i64>())
            .transpose()
            .map_err(|_| HttpResponse::BadRequest().body("Count must be a valid number."))?
            .unwrap_or(400);

        if count < 1 || count > 400 {
            return Err(HttpResponse::BadRequest().body("Count must be between 1 and 400."));
        }

        let page = query
            .page
            .as_ref()
            .map(|p| p.parse::<i64>())
            .transpose()
            .map_err(|_| HttpResponse::BadRequest().body("Page must be a valid number."))?
            .unwrap_or(1);

        if page < 1 {
            return Err(HttpResponse::BadRequest().body("Page must be greater than or equal to 1."));
        }

        let from = if let Some(from_str) = &query.from {
            parse_date(from_str)?
        } else {
            1648771200 // Default to April 1, 2022
        };

        let to = if let Some(to_str) = &query.to {
            parse_date(to_str)?
        } else {
            Utc::now().timestamp() // Default to current time
        };

        if from > to {
            return Err(HttpResponse::BadRequest().body("'from' cannot be greater than 'to'."));
        }

        Ok(Self {
            page,
            count,
            from,
            to,
        })
    }

    pub fn skip(&self) -> i64 {
        (self.page - 1).max(0) * self.count
    }

    pub fn date_filter(&self) -> mongodb::bson::Document {
        doc! {
            "startTime": { "$gte": self.from as f64 },
            "endTime": { "$lte": self.to as f64 }
        }
    }
}
