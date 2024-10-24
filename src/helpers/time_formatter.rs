use actix_web::HttpResponse;
use chrono::NaiveDateTime;

pub fn parse_date(date_str: &str) -> Result<i64, HttpResponse> {
    match NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S") {
        Ok(datetime) => Ok(datetime.and_utc().timestamp()),
        Err(_) => {
            Err(HttpResponse::BadRequest().body("Invalid date format. Use 'YYYY-MM-DDTHH:MM:SS'."))
        }
    }
}
