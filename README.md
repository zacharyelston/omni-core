# Omni Core

Minimal authentication and session management server for mobile-first applications.

## Features

- **Simple API Key Authentication** - Join to get a key, use it for all requests
- **In-Memory Sessions** - Fast, no database required
- **Mobile-First Design** - Optimized for mobile clients
- **Rust Backend** - Fast, safe, and reliable

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/health` | Health check |
| POST | `/api/v1/auth/join` | Create new session, get API key |
| POST | `/api/v1/auth/verify` | Verify API key is valid |
| POST | `/api/v1/auth/logout` | Invalidate session |

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

# Join (get API key)
curl -X POST http://localhost:8080/api/v1/auth/join

# Verify key
curl -X POST http://localhost:8080/api/v1/auth/verify \
  -H "Content-Type: application/json" \
  -d '{"api_key": "omni_YOUR_KEY_HERE"}'

# Logout
curl -X POST http://localhost:8080/api/v1/auth/logout \
  -H "Content-Type: application/json" \
  -d '{"api_key": "omni_YOUR_KEY_HERE"}'
```

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
