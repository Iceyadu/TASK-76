//! HTTP API: `/api/exports`
use axum::http::{header, HeaderValue};

use crate::http_helpers::{admin_token_and_csrf, api_server};

#[tokio::test]
async fn api_route_get_exports_returns_export_envelope() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/exports")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    assert!(body["reservations"].is_array(), "export must include reservations");
    assert!(body["vehicles"].is_array(), "export must include vehicles");
    assert!(body["export_type"].as_str().is_some());
    assert!(body["exported_at"].as_str().is_some());
}

#[tokio::test]
async fn api_route_get_exports_filtered_by_store() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/exports")
        .add_query_param("store_id", "store-001")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    // All returned vehicles should belong to store-001
    for v in body["vehicles"].as_array().unwrap() {
        assert_eq!(v["store_id"], "store-001");
    }
}

#[tokio::test]
async fn api_route_get_exports_vehicles_omit_sensitive_fields() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/exports")
        .add_query_param("store_id", "store-001")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    for v in body["vehicles"].as_array().unwrap() {
        assert!(v.get("vin").is_none(), "VIN must not appear in export");
        assert!(v.get("vin_encrypted").is_none(), "vin_encrypted must not appear in export");
        assert!(v.get("license_plate").is_none(), "license_plate must not appear in export");
    }
}

#[tokio::test]
async fn api_route_get_exports_requires_auth() {
    let s = api_server();
    let res = s.get("/api/exports").await;
    res.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}
