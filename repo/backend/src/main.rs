use std::sync::{Arc, Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use fleetreserve_backend::app::state::AppState;
use fleetreserve_backend::routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting FleetReserve Operations Suite backend");

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "fleetreserve.db".to_string());
    let conn = rusqlite::Connection::open(&db_url).expect("Failed to open database");
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("Failed to set pragmas");

    run_migrations(&conn);

    let encryption_key = std::env::var("ENCRYPTION_KEY")
        .expect("ENCRYPTION_KEY is required");
    let hmac_secret = std::env::var("HMAC_SECRET")
        .expect("HMAC_SECRET is required");
    if encryption_key.len() < 32 {
        panic!("ENCRYPTION_KEY must be at least 32 characters");
    }
    if hmac_secret.len() < 32 {
        panic!("HMAC_SECRET must be at least 32 characters");
    }
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());

    std::fs::create_dir_all(&upload_dir).expect("Failed to create upload directory");

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        encryption_key,
        hmac_secret,
        upload_dir,
        csrf_tokens: Arc::new(Mutex::new(std::collections::HashMap::new())),
    };

    let app = routes::build_router(state);

    let addr = "0.0.0.0:3001";
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn run_migrations(conn: &rusqlite::Connection) {
    let schema = include_str!("../migrations/001_initial_schema.sql");
    conn.execute_batch(schema).expect("Failed to run schema migration");

    let seed = include_str!("../migrations/002_seed_data.sql");
    conn.execute_batch(seed).expect("Failed to run seed migration");

    tracing::info!("Database migrations complete");
}
