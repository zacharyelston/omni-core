# Omni Core

**Minimal authentication and session management server with bi-directional encryption.**

## Overview

Omni Core provides a secure foundation for mobile-first applications requiring:
- X25519 key exchange (Elliptic Curve Diffie-Hellman)
- ChaCha20-Poly1305 authenticated encryption
- Per-client server keypairs
- Session management with API keys

## Quick Links

- [[Architecture]] - System design and components
- [[API Reference]] - Complete endpoint documentation
- [[Key Exchange Flow]] - How encryption works
- [[Frontend Guide]] - Mobile-first React UI
- [[Backend Guide]] - Rust Axum server
- [[Deployment]] - Replit and production setup

## Repository Structure

```
omni-core/
├── backend/           # Rust Axum server
│   └── src/
│       ├── main.rs    # Entry point
│       ├── config.rs  # Configuration
│       ├── api/       # HTTP endpoints
│       └── services/  # Business logic & crypto
├── frontend/          # Next.js mobile-first PWA
│   └── src/app/
│       ├── layout.tsx
│       └── page.tsx   # Tabbed UI
├── data/              # Runtime data (gitignored)
│   ├── server_keys.yaml
│   └── client_config.yaml
├── .replit            # Replit config
└── replit.nix         # Nix dependencies
```

## Getting Started

### Backend
```bash
cd backend
cargo run
# Server starts at http://localhost:8080
```

### Frontend
```bash
cd frontend
npm install
npm run dev
# UI at http://localhost:5000
```

## License

MIT
