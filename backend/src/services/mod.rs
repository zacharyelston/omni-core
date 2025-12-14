//! Application services

mod admin;
mod crypto;
mod keystore;
mod session;

#[cfg(test)]
mod crypto_test;
#[cfg(test)]
mod keystore_test;
#[cfg(test)]
mod session_test;

use crate::config::Config;
use std::sync::Arc;

pub use admin::AdminAuth;
pub use crypto::{parse_public_key, CryptoError, EncryptedMessage, ServerKeyPair};
pub use keystore::{ClientEntry, KeyStoreManager, ServerKeyEntry};
pub use session::{Session, SessionStore};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub sessions: SessionStore,
    pub server_keypair: Arc<ServerKeyPair>,
    pub keystore: KeyStoreManager,
    pub admin: AdminAuth,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let server_keypair = Arc::new(ServerKeyPair::generate());
        let admin = AdminAuth::new(&server_keypair.public_key_hex());
        
        Self {
            config: Arc::new(config),
            sessions: SessionStore::new(),
            server_keypair,
            keystore: KeyStoreManager::new(),
            admin,
        }
    }
}
