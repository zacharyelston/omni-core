//! Omni Core Library
//!
//! Reusable components for authentication, cryptography, and server federation.
//!
//! # Modules
//!
//! - **crypto** - X25519 key exchange and ChaCha20Poly1305 encryption
//! - **keystore** - Persistent key storage for server and client keys
//! - **registry** - Server registry for federation
//! - **sync** - Background sync service for server discovery
//! - **session** - Session management
//!
//! # Example
//!
//! ```rust,ignore
//! use omni_core::{ServerKeyPair, ClientKeyPair, EncryptedMessage};
//!
//! let server = ServerKeyPair::generate();
//! let client = ClientKeyPair::generate();
//! let shared = server.derive_shared_secret_hex(&client.public_key_hex()).unwrap();
//! let encrypted = EncryptedMessage::encrypt(b"Hello", &shared).unwrap();
//! ```

pub mod crypto;
pub mod keystore;
pub mod registry;
pub mod session;
pub mod sync;

// Re-export main types
pub use crypto::{parse_public_key, ClientKeyPair, CryptoError, EncryptedMessage, ServerKeyPair};
pub use keystore::{ClientEntry, KeyStoreManager, ServerKeyEntry};
pub use registry::{ServerEntry, ServerRegistry};
pub use session::{Session, SessionStore};
pub use sync::{SyncConfig, SyncService};
