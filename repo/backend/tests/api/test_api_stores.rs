//! HTTP API: `/api/stores`
use axum::http::{header, HeaderValue};

use crate::http_helpers::{admin_token_and_csrf, api_server};

#[tokio::test]
async fn api_route_get_stores_lists_active_stores() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/stores")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    let stores = body["stores"].as_array().expect("stores");
    assert!(!stores.is_empty());
}
