# Omni Core

Minimal authentication and session management server with **bi-directional encryption**.

## Features

- **X25519 Key Exchange** - Elliptic curve Diffie-Hellman for secure key agreement
- **ChaCha20-Poly1305 Encryption** - Fast, secure authenticated encryption
- **Simple API Key Authentication** - Join to get a key, use it for all requests
- **In-Memory Sessions** - Fast, no database required
- **Mobile-First Design** - Optimized for mobile clients
- **Rust Backend** - Fast, safe, and reliable

## API Endpoints

### Authentication
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/health` | Health check |
| POST | `/api/v1/auth/join` | Create new session, get API key |
| POST | `/api/v1/auth/verify` | Verify API key is valid |
| POST | `/api/v1/auth/logout` | Invalidate session |

### Key Exchange & Encryption
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/keys/public` | Get server's X25519 public key |
| POST | `/api/v1/keys/exchange` | Exchange keys and create encrypted session |
| POST | `/api/v1/keys/send` | Send encrypted message to server |

## Quick Start

### Local Development

```bash
cd backend
cargo run
```

Server starts at `http://localhost:8080`

### Test the API

```bash
# Health check
curl http://localhost:8080/api/v1/health

# Get server's public key
curl http://localhost:8080/api/v1/keys/public

# Join (get API key)
curl -X POST http://localhost:8080/api/v1/auth/join

# Key exchange (provide your X25519 public key)
curl -X POST http://localhost:8080/api/v1/keys/exchange \
  -H "Content-Type: application/json" \
  -d '{"client_public_key": "YOUR_HEX_PUBLIC_KEY"}'

# Verify key
curl -X POST http://localhost:8080/api/v1/auth/verify \
  -H "Content-Type: application/json" \
  -d '{"api_key": "omni_YOUR_KEY_HERE"}'

# Logout
curl -X POST http://localhost:8080/api/v1/auth/logout \
  -H "Content-Type: application/json" \
  -d '{"api_key": "omni_YOUR_KEY_HERE"}'
```

## Encryption Flow

1. **Client** generates ephemeral X25519 keypair
2. **Client** fetches server's public key from `/keys/public`
3. **Client** sends its public key to `/keys/exchange`
4. **Both** derive the same shared secret using ECDH
5. **Messages** are encrypted with ChaCha20-Poly1305 using the shared secret
6. **Each message** includes a random 12-byte nonce

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8080 | Server port |
| `SECRET_KEY` | change-me | Secret for signing |
| `SESSION_TTL` | 3600 | Session lifetime in seconds |

## Replit Deployment

This repo is configured for Replit. Import from GitHub and it will auto-detect Rust.

## Architecture

```
omni-core/
├── backend/           # Rust Axum server
│   └── src/
│       ├── main.rs    # Entry point
│       ├── config.rs  # Configuration
│       ├── api/       # HTTP endpoints
│       └── services/  # Business logic
├── .replit            # Replit config
└── replit.nix         # Nix dependencies
```

## License

MIT
