use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub encryption_key: String,
    pub hmac_secret: String,
    pub upload_dir: String,
    /// Server-side CSRF token store: maps user_id -> active CSRF token
    pub csrf_tokens: Arc<Mutex<HashMap<String, String>>>,
}
