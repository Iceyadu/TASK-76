//! HTTP API: `/api/vehicles`
use axum::http::{header, HeaderValue};

use crate::http_helpers::{admin_token_and_csrf, api_server};

#[tokio::test]
async fn api_route_get_vehicles_returns_masked_row() {
    let s = api_server();
    let (token, _) = admin_token_and_csrf(&s).await;

    let res = s
        .get("/api/vehicles")
        .add_query_param("store_id", "store-001")
        .add_header(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        )
        .await;
    res.assert_status_ok();
    let body = res.json::<serde_json::Value>();
    let vehicles = body["vehicles"].as_array().expect("vehicles");
    assert!(!vehicles.is_empty());
    let v0 = &vehicles[0];
    assert_eq!(v0["id"], "v1");
    let vin = v0["vin"].as_str().unwrap();
    assert!(vin.contains('*'), "VIN must be masked in API JSON: {}", vin);
}
