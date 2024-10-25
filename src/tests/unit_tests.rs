#[cfg(test)]
mod tests {
    use mongodb::bson::doc;

    use crate::{helpers::query_parser::QueryParser, routes::types::CommonQueryParams};

    #[test]
    fn test_valid_query() {
        let query = CommonQueryParams {
            count: Some("10".to_string()),
            page: Some("1".to_string()),
            from: Some("2022-04-01T00:00:00".to_string()),
            to: Some("2023-04-01T00:00:00".to_string()),
        };
        let parser = QueryParser::new(&query, 400).unwrap();
        assert_eq!(parser.count, 10);
        assert_eq!(parser.page, 1);
    }

    #[test]
    fn test_invalid_count() {
        let query = CommonQueryParams {
            count: Some("500".to_string()),
            page: Some("1".to_string()),
            from: None,
            to: None,
        };
        let result = QueryParser::new(&query, 400);
        assert!(result.is_err());
    }
    #[test]
    fn test_date_filter() {
        let parser = QueryParser {
            page: 1,
            count: 10,
            from: 1648771200,
            to: 1670304000,
        };
        let filter = parser.date_filter();
        let expected = doc! {
            "startTime": { "$gte": 1648771200f64 },
            "endTime": { "$lte": 1670304000f64 }
        };
        assert_eq!(filter, expected);
    }
}
