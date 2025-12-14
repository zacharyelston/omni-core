//! Application services

mod crypto;
mod session;

use crate::config::Config;
use std::sync::Arc;

pub use crypto::{parse_public_key, CryptoError, EncryptedMessage, ServerKeyPair};
pub use session::{Session, SessionStore};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub sessions: SessionStore,
    pub server_keypair: Arc<ServerKeyPair>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            sessions: SessionStore::new(),
            server_keypair: Arc::new(ServerKeyPair::generate()),
        }
    }
}
