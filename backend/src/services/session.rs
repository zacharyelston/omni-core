//! In-memory session store

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

impl Session {
    pub fn new(ttl_secs: u64) -> Self {
        let now = Utc::now();
        let api_key = generate_api_key();
        Self {
            id: Uuid::new_v4(),
            api_key,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(ttl_secs as i64),
            last_seen: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn touch(&mut self) {
        self.last_seen = Utc::now();
    }
}

fn generate_api_key() -> String {
    use base64::Engine;
    use rand::RngCore;

    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    format!("omni_{}", base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes))
}

#[derive(Clone, Default)]
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&self, ttl_secs: u64) -> Session {
        let session = Session::new(ttl_secs);
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session.api_key.clone(), session.clone());
        session
    }

    pub fn get(&self, api_key: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(api_key).cloned()
    }

    pub fn validate(&self, api_key: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(api_key) {
            if session.is_expired() {
                sessions.remove(api_key);
                return None;
            }
            session.touch();
            return Some(session.clone());
        }
        None
    }

    pub fn revoke(&self, api_key: &str) -> bool {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(api_key).is_some()
    }

    pub fn cleanup_expired(&self) -> usize {
        let mut sessions = self.sessions.write().unwrap();
        let before = sessions.len();
        sessions.retain(|_, s| !s.is_expired());
        before - sessions.len()
    }
}
