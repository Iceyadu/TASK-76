use fleetreserve_backend::audit::chain::{append_audit_log, verify_chain_integrity};
use rusqlite::Connection;

fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(include_str!("../backend/migrations/001_initial_schema.sql"))
        .unwrap();
    conn
}

#[test]
fn api_audit_chain_integrity_happy_path() {
    let conn = setup_db();
    append_audit_log(
        &conn,
        "user-1",
        "alice",
        "CREATE",
        "reservation",
        "res-1",
        &serde_json::json!({"k":"v"}),
    )
    .unwrap();
    append_audit_log(
        &conn,
        "user-1",
        "alice",
        "UPDATE",
        "reservation",
        "res-1",
        &serde_json::json!({"status":"confirmed"}),
    )
    .unwrap();
    assert!(verify_chain_integrity(&conn).unwrap());
}
