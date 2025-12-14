# Architecture

## System Overview

```
┌─────────────────┐         ┌─────────────────┐
│   Frontend      │  HTTPS  │    Backend      │
│   (Next.js)     │◄───────►│    (Axum)       │
│   Port 5000     │         │    Port 8080    │
└─────────────────┘         └─────────────────┘
        │                           │
        ▼                           ▼
┌─────────────────┐         ┌─────────────────┐
│  localStorage   │         │  YAML Files     │
│  - Client keys  │         │  - server_keys  │
│  - API keys     │         │  - client_config│
└─────────────────┘         └─────────────────┘
```

## Components

### Backend (Rust/Axum)

| Module | Purpose |
|--------|---------|
| `api/auth.rs` | Basic join/verify/logout |
| `api/keys.rs` | Legacy key exchange |
| `api/register.rs` | Per-client registration |
| `api/health.rs` | Health check |
| `services/crypto.rs` | X25519 + ChaCha20 |
| `services/keystore.rs` | YAML key storage |
| `services/session.rs` | In-memory sessions |

### Frontend (Next.js)

| Component | Purpose |
|-----------|---------|
| Register Tab | Two-step registration flow |
| My Keys Tab | View client keypairs |
| Server Keys Tab | View known server keys |

## Data Flow

### Registration
1. Client sends `client_id` to `/register/init`
2. Server generates X25519 keypair for client
3. Server saves to `server_keys.yaml`
4. Server returns public key
5. Client generates its own keypair
6. Client sends public key to `/register/complete`
7. Server saves to `client_config.yaml`
8. Server returns API key

### Encrypted Communication
1. Both parties have each other's public keys
2. Derive shared secret via ECDH
3. Encrypt with ChaCha20-Poly1305
4. Include random 12-byte nonce per message

## Security Model

- **Key Exchange**: X25519 (Curve25519 ECDH)
- **Encryption**: ChaCha20-Poly1305 (AEAD)
- **Key Storage**: YAML files (server-side)
- **Session Tokens**: Random 32-byte base64

### Threat Considerations

| Threat | Mitigation |
|--------|------------|
| MITM | ECDH key exchange |
| Replay | Random nonce per message |
| Key theft | Per-client keypairs |
| Session hijack | Short TTL, API key rotation |
