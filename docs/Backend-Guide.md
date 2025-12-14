# Backend Guide

## Technology Stack

- **Language**: Rust 2021 Edition
- **Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Crypto**: x25519-dalek, chacha20poly1305

## Project Structure

```
backend/
├── Cargo.toml
└── src/
    ├── main.rs           # Entry point, server setup
    ├── config.rs         # Environment configuration
    ├── api/
    │   ├── mod.rs        # Route definitions
    │   ├── auth.rs       # Join/verify/logout
    │   ├── health.rs     # Health check
    │   ├── keys.rs       # Legacy key exchange
    │   └── register.rs   # Per-client registration
    └── services/
        ├── mod.rs        # AppState definition
        ├── crypto.rs     # X25519 + ChaCha20
        ├── keystore.rs   # YAML key storage
        └── session.rs    # In-memory sessions
```

## Running

```bash
cd backend
cargo run
```

With logging:
```bash
RUST_LOG=debug cargo run
```

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | Server port |
| `SECRET_KEY` | change-me | Secret for signing |
| `SESSION_TTL` | 3600 | Session lifetime (seconds) |
| `RUST_LOG` | info | Log level |

## Key Components

### AppState

Shared state across all handlers:

```rust
pub struct AppState {
    pub config: Arc<Config>,
    pub sessions: SessionStore,
    pub server_keypair: Arc<ServerKeyPair>,
    pub keystore: KeyStoreManager,
}
```

### KeyStoreManager

Manages YAML-based key storage:

```rust
// Generate server key for client
let key = keystore.generate_server_key_for_client("device-001");

// Register client with their public key
keystore.register_client("device-001", "abc123...");

// Derive shared secret
let secret = keystore.derive_shared_secret("device-001");
```

### Crypto Module

```rust
// Server keypair
let server = ServerKeyPair::generate();
let public_hex = server.public_key_hex();

// Derive shared secret
let shared = server.derive_shared_secret(&client_public_bytes);

// Encrypt
let encrypted = EncryptedMessage::encrypt(plaintext, &shared)?;

// Decrypt
let plaintext = encrypted.decrypt(&shared)?;
```

## Adding New Endpoints

1. Create handler in `api/` directory
2. Add route in `api/mod.rs`
3. Use `State<AppState>` extractor

Example:
```rust
pub async fn my_handler(
    State(state): State<AppState>,
    Json(req): Json<MyRequest>,
) -> Result<Json<MyResponse>, (StatusCode, String)> {
    // Implementation
}
```

## Testing

```bash
cargo test
```

Run specific test:
```bash
cargo test test_key_exchange
```

## Building for Production

```bash
cargo build --release
./target/release/omni-server
```
