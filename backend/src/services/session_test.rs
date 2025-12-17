//! Tests for session module

#[cfg(test)]
mod tests {
    use crate::services::session::*;

    #[test]
    fn test_session_creation() {
        let session = Session::new(3600);

        assert!(!session.api_key.is_empty());
        assert!(session.api_key.starts_with("omni_"));
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_store_create_and_get() {
        let store = SessionStore::new();
        let session = store.create(3600);

        let retrieved = store.get(&session.api_key);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, session.id);
    }

    #[test]
    fn test_session_store_validate() {
        let store = SessionStore::new();
        let session = store.create(3600);

        let validated = store.validate(&session.api_key);
        assert!(validated.is_some());
    }

    #[test]
    fn test_session_store_validate_invalid_key() {
        let store = SessionStore::new();

        let validated = store.validate("invalid_key");
        assert!(validated.is_none());
    }

    #[test]
    fn test_session_store_revoke() {
        let store = SessionStore::new();
        let session = store.create(3600);

        let revoked = store.revoke(&session.api_key);
        assert!(revoked);

        let retrieved = store.get(&session.api_key);
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_session_store_revoke_nonexistent() {
        let store = SessionStore::new();

        let revoked = store.revoke("nonexistent_key");
        assert!(!revoked);
    }

    #[test]
    fn test_session_expiry() {
        let session = Session::new(0); // 0 second TTL

        // Should be expired immediately or very soon
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_touch_updates_last_seen() {
        let mut session = Session::new(3600);
        let original_last_seen = session.last_seen;

        std::thread::sleep(std::time::Duration::from_millis(10));
        session.touch();

        assert!(session.last_seen > original_last_seen);
    }

    #[test]
    fn test_cleanup_expired_sessions() {
        let store = SessionStore::new();

        // Create expired session
        let _expired = store.create(0);
        std::thread::sleep(std::time::Duration::from_millis(10));

        // Create valid session
        let valid = store.create(3600);

        let cleaned = store.cleanup_expired();
        assert_eq!(cleaned, 1);

        // Valid session should still exist
        assert!(store.get(&valid.api_key).is_some());
    }
}
