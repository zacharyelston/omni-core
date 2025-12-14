# Omni Core

Welcome to the Omni Core documentation.

## Overview

Omni Core is a lightweight authentication and session management system built with:
- **Backend**: Rust + Axum
- **Frontend**: Next.js + React + TailwindCSS
- **Encryption**: X25519 key exchange + ChaCha20-Poly1305
- **Federation**: Server-to-server discovery and sync

## Features

- Session-based authentication
- Bi-directional encryption with X25519 key exchange
- Per-client server keypairs
- Server-to-server federation (DNS-like discovery)
- QR code key sharing
- Mobile-first responsive UI
- Docker support

## Quick Start

```bash
# Clone the repo
git clone https://github.com/zacharyelston/omni-core.git
cd omni-core

# Option 1: Docker (recommended)
docker-compose up -d

# Option 2: Local development
cargo run                    # Backend
cd frontend && npm run dev   # Frontend
```

## Documentation

- **[Quick Start](Quick-Start.md)** - Get running in 5 minutes
- **[Integration Guide](Integration-Guide.md)** - Add Omni Core to your project
- [Architecture](Architecture.md) - System design and components
- [API Reference](API-Reference.md) - REST API endpoints
- [Key Exchange Flow](Key-Exchange-Flow.md) - Encryption protocol
- [Backend Guide](Backend-Guide.md) - Rust backend details
- [Frontend Guide](Frontend-Guide.md) - React frontend details
- [Deployment](Deployment.md) - Production deployment

## Project Structure

```
omni-core/
├── backend/          # Rust Axum server
│   └── src/
│       ├── api/      # REST endpoints
│       ├── services/ # Business logic
│       └── config.rs # Configuration
├── frontend/         # Next.js React app
│   └── src/
│       └── app/      # App router pages
├── data/             # Runtime data
│   ├── config.d/     # Server configuration
│   ├── clients.d/    # Per-client configs
│   └── servers.d/    # Known servers
├── docker-compose.yml      # Production setup
└── docker-compose.test.yml # Multi-server testing
```

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /health` | Health check |
| `GET /server/info` | Server public key and info |
| `POST /register/init` | Start client registration |
| `POST /register/complete` | Complete registration |
| `POST /auth/verify` | Verify API key |
| `GET /servers/public` | List public servers |
| `POST /servers/register` | Register a server |

See [API Reference](API-Reference.md) for full documentation.

## License

MIT
