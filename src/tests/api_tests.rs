use crate::{db::connection::MongoDB, routes};
use actix_web::{http::StatusCode, test, web, App};

#[actix_web::test]
async fn test_get_runepool_history() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::rpmuh_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/history/runepool?page=1&count=10&sort_by=units")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_runepool_history_invalid_sort() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::rpmuh_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/history/runepool?page=1&count=10&sort_by=invalid_field")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_get_earnings_history() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::earnings_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/earnings?interval=day&from=2023-10-23T00:00:00&to=2023-10-30T00:00:00")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_earnings_history_invalid_sort() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::earnings_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/earnings?interval=day&from=2023-10-23T00:00:00&to=2023-10-30T00:00:00&sort_by=iinvalid")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// Tests for /swaps
#[actix_web::test]
async fn test_get_swaps_history() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::swaps_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/swaps?sort_by=totalFees&count=3&page=3")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_swaps_history_invalid_sort() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::swaps_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/swaps?count=3&page=3&sort_by=invalid")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

// Tests for /depth
#[actix_web::test]
async fn test_get_depth_data() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::depths_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/depths?interval=day&from=2023-01-01T00:00:00&to=2023-12-31T00:00:00")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_get_depth_data_invalid_sort() {
    let mongo_db = MongoDB::init().await.expect("Failed to initialize MongoDB");

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(mongo_db))
            .configure(routes::depths_history::init),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/depths?sort_by=invalid_field")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
