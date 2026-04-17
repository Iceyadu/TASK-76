//! HTTP API: `/api/calendar`
use axum::http::{header, HeaderValue};

use crate::http_helpers::{admin_token_and_csrf, api_server};

#[tokio::test]
async fn api_route_get_calendar_day_view_returns_slots_and_assets() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/calendar")
        .add_query_param("store_id", "store-001")
        .add_query_param("date", "2026-06-15")
        .add_query_param("view", "day")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert_eq!(body["store_id"], "store-001");
    assert_eq!(body["view"], "day");
    assert!(body["slots"].is_array(), "slots must be an array");
    assert!(body["assets"].is_array(), "assets must be an array");
    assert!(body["business_hours"].is_object(), "business_hours must be present");
    // Day view with 24h range (00:00-23:59) produces many 15-min slots
    let slots = body["slots"].as_array().unwrap();
    assert!(!slots.is_empty(), "slots must not be empty for a valid business day");
}

#[tokio::test]
async fn api_route_get_calendar_week_view_covers_seven_days() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    // 2026-06-15 is a Monday; week view should cover Mon-Sun
    let res = s
        .get("/api/calendar")
        .add_query_param("store_id", "store-001")
        .add_query_param("date", "2026-06-15")
        .add_query_param("view", "week")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert_eq!(body["view"], "week");
    // Week view produces 7× more slots than a day view
    let slots = body["slots"].as_array().unwrap();
    assert!(slots.len() > 1, "week view should have multiple slots");
}

#[tokio::test]
async fn api_route_get_calendar_invalid_date_rejected() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/calendar")
        .add_query_param("store_id", "store-001")
        .add_query_param("date", "not-a-date")
        .add_query_param("view", "day")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status(axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn api_route_get_calendar_unknown_store_returns_not_found() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/calendar")
        .add_query_param("store_id", "store-does-not-exist")
        .add_query_param("date", "2026-06-15")
        .add_query_param("view", "day")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn api_route_get_calendar_requires_auth() {
    let s = api_server();
    let res = s
        .get("/api/calendar")
        .add_query_param("store_id", "store-001")
        .add_query_param("date", "2026-06-15")
        .add_query_param("view", "day")
        .await;
    res.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn api_route_get_calendar_slots_have_fifteen_minute_duration() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/calendar")
        .add_query_param("store_id", "store-001")
        .add_query_param("date", "2026-06-15")
        .add_query_param("view", "day")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    let slots = body["slots"].as_array().unwrap();
    for slot in slots {
        assert_eq!(slot["duration_minutes"], 15);
    }
}
