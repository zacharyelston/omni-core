//! Application services

mod admin;
mod client_store;
mod crypto;
mod keystore;
mod server_registry;
mod session;
mod sync;

#[cfg(test)]
mod crypto_test;
#[cfg(test)]
mod keystore_test;
#[cfg(test)]
mod session_test;

use crate::config::Config;
use std::sync::Arc;

pub use admin::AdminAuth;
pub use client_store::{ClientConfig, ClientStore};
pub use crypto::{parse_public_key, CryptoError, EncryptedMessage, ServerKeyPair};
pub use keystore::{ClientEntry, KeyStoreManager, ServerKeyEntry};
pub use server_registry::{ServerEntry, ServerRegistry};
pub use session::{Session, SessionStore};
pub use sync::spawn_sync_service;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub sessions: SessionStore,
    pub server_keypair: Arc<ServerKeyPair>,
    pub keystore: KeyStoreManager,
    pub client_store: ClientStore,
    pub server_registry: ServerRegistry,
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
            client_store: ClientStore::new(),
            server_registry: ServerRegistry::new(),
            admin,
        }
    }
}
