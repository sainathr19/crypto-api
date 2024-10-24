// Helper function to convert the interval string into seconds
pub fn interval_to_seconds(interval: &str) -> i64 {
    match interval {
        "hour" => 3600,
        "day" => 86400,
        "week" => 604800,
        "month" => 2629800,
        "quarter" => 7889400,
        "year" => 31557600,
        _ => 86400,
    }
}
