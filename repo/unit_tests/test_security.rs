use fleetreserve_backend::audit::chain::{append_audit_log, verify_chain_integrity};
use fleetreserve_backend::security::masking::{mask_license_plate, mask_vin};
use rusqlite::Connection;

#[test]
fn unit_security_masking() {
    assert_eq!(mask_vin("1HGCM82633A123456"), "*************3456");
    assert_eq!(mask_license_plate("ABC1234"), "*****34");
}

#[test]
fn unit_security_audit_chain() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(include_str!("../backend/migrations/001_initial_schema.sql")).unwrap();
    append_audit_log(&conn, "u1", "alice", "CREATE", "x", "1", &serde_json::json!({})).unwrap();
    append_audit_log(&conn, "u1", "alice", "UPDATE", "x", "1", &serde_json::json!({"a":1})).unwrap();
    assert!(verify_chain_integrity(&conn).unwrap());
}
