//! Session management for authenticated clients.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub client_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

impl Session {
    /// Create a new session
    pub fn new(client_id: &str, ttl_secs: i64) -> Self {
        let now = chrono::Utc::now();
        Self {
            session_id: Uuid::new_v4().to_string(),
            client_id: client_id.to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::seconds(ttl_secs),
            metadata: HashMap::new(),
        }
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }

    /// Extend session expiration
    pub fn extend(&mut self, ttl_secs: i64) {
        self.expires_at = chrono::Utc::now() + chrono::Duration::seconds(ttl_secs);
    }
}

/// Session store for managing active sessions
#[derive(Clone)]
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    default_ttl_secs: i64,
}

impl SessionStore {
    /// Create a new session store
    pub fn new() -> Self {
        Self::with_ttl(3600) // 1 hour default
    }

    /// Create with custom TTL
    pub fn with_ttl(ttl_secs: i64) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_ttl_secs: ttl_secs,
        }
    }

    /// Create a new session for a client
    pub fn create(&self, client_id: &str) -> Session {
        let session = Session::new(client_id, self.default_ttl_secs);
        self.sessions
            .write()
            .unwrap()
            .insert(session.session_id.clone(), session.clone());
        session
    }

    /// Get a session by ID
    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).and_then(|s| {
            if s.is_expired() {
                None
            } else {
                Some(s.clone())
            }
        })
    }

    /// Validate and get session
    pub fn validate(&self, session_id: &str) -> Option<Session> {
        self.get(session_id)
    }

    /// Extend a session
    pub fn extend(&self, session_id: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            if !session.is_expired() {
                session.extend(self.default_ttl_secs);
                return Some(session.clone());
            }
        }
        None
    }

    /// Revoke a session
    pub fn revoke(&self, session_id: &str) -> bool {
        self.sessions.write().unwrap().remove(session_id).is_some()
    }

    /// Revoke all sessions for a client
    pub fn revoke_all_for_client(&self, client_id: &str) {
        self.sessions
            .write()
            .unwrap()
            .retain(|_, s| s.client_id != client_id);
    }

    /// Clean up expired sessions
    pub fn cleanup_expired(&self) {
        self.sessions
            .write()
            .unwrap()
            .retain(|_, s| !s.is_expired());
    }

    /// Get active session count
    pub fn active_count(&self) -> usize {
        self.sessions
            .read()
            .unwrap()
            .values()
            .filter(|s| !s.is_expired())
            .count()
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let store = SessionStore::new();
        let session = store.create("client-1");

        assert_eq!(session.client_id, "client-1");
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_validation() {
        let store = SessionStore::new();
        let session = store.create("client-1");

        let validated = store.validate(&session.session_id);
        assert!(validated.is_some());
    }

    #[test]
    fn test_session_revocation() {
        let store = SessionStore::new();
        let session = store.create("client-1");

        assert!(store.revoke(&session.session_id));
        assert!(store.get(&session.session_id).is_none());
    }
}
