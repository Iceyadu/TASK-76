//! Shared `AppState` + DB seeding for HTTP API route tests (`axum-test`).
use fleetreserve_backend::app::state::AppState;
use fleetreserve_backend::auth::password;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tempfile::NamedTempFile;

/// Known password for the seeded admin user (see `prepare_test_database`).
pub const TEST_ADMIN_PASSWORD: &str = "FleetReserveHttpTest#2026";

fn test_secrets() -> (String, String) {
    (
        "x".repeat(32),
        "y".repeat(32),
    )
}

fn prepare_test_database(conn: &Connection) {
    let schema = include_str!("../../migrations/001_initial_schema.sql");
    let seed = include_str!("../../migrations/002_seed_data.sql");
    conn.execute_batch(schema).expect("schema");
    conn.execute_batch(seed).expect("seed");
    conn.execute_batch(
        "UPDATE stores SET business_hours_start = '00:00', business_hours_end = '23:59' WHERE id IN ('store-001','store-002');",
    )
    .expect("relax hours");
    let hash = password::hash_password(TEST_ADMIN_PASSWORD).expect("hash admin password");
    conn.execute(
        "UPDATE users SET active = 1, password_hash = ?1 WHERE id = 'user-admin-001'",
        [&hash],
    )
    .expect("activate admin with known password");
    conn.execute(
        "INSERT INTO vehicles (id, vin_encrypted, vin_hash, license_plate_encrypted, license_plate_hash, make, model, store_id, status, insurance_expiry, version) VALUES ('v1', 'enc', 'h', 'enc', 'h', 'T', 'V', 'store-001', 'available', '2100-01-01T00:00:00', 1)",
        [],
    )
    .expect("seed vehicle");
}

pub fn test_app_state() -> AppState {
    let tmp = NamedTempFile::new().expect("temp db");
    let conn = Connection::open(tmp.path()).expect("open db");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("pragma");
    prepare_test_database(&conn);
    let upload_dir = tempfile::tempdir().expect("upload dir");
    let upload_path = upload_dir.path().to_string_lossy().into_owned();
    // Keep the directory on disk: `AppState` only stores `String`, so dropping `TempDir` here would delete it.
    std::mem::forget(upload_dir);
    let (encryption_key, hmac_secret) = test_secrets();
    AppState {
        db: Arc::new(Mutex::new(conn)),
        encryption_key,
        hmac_secret,
        upload_dir: upload_path,
        csrf_tokens: Arc::new(Mutex::new(HashMap::new())),
    }
}

