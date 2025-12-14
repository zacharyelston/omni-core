# Omni Core Quick Start

Get up and running in 5 minutes.

## Prerequisites

- Docker and Docker Compose, OR
- Rust 1.75+ and Node.js 20+

## Option 1: Docker (Fastest)

```bash
git clone https://github.com/zacharyelston/omni-core.git
cd omni-core
docker-compose up -d
```

**That's it!** 
- Backend: http://localhost:8080
- Frontend: http://localhost:3000

## Option 2: Local Development

```bash
git clone https://github.com/zacharyelston/omni-core.git
cd omni-core

# Terminal 1: Backend
cargo run

# Terminal 2: Frontend
cd frontend && npm install && npm run dev
```

## First Steps

### 1. Check Health

```bash
curl http://localhost:8080/api/v1/health
```

### 2. Get Server Info

```bash
curl http://localhost:8080/api/v1/server/info
```

### 3. Register a Client

```bash
# Initialize
curl -X POST http://localhost:8080/api/v1/register/init \
  -H "Content-Type: application/json" \
  -d '{"client_id": "my-first-client"}'

# Complete (simplified - see Integration Guide for full flow)
curl -X POST http://localhost:8080/api/v1/register/complete \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "my-first-client",
    "encrypted_client_public_key": {
      "nonce": "",
      "ciphertext": "dGVzdA=="
    }
  }'
```

### 4. Open the Dashboard

Visit http://localhost:3000 to see:
- Server public key (with QR code)
- Registered clients
- Known servers

## What's Next?

- **[Integration Guide](Integration-Guide.md)** - Full client SDK examples
- **[API Reference](API-Reference.md)** - All endpoints documented
- **[Architecture](Architecture.md)** - How it works under the hood

## Common Commands

```bash
# Run tests
make test

# Check code
make lint

# Build for production
make build

# Docker test suite (3 servers)
make docker-test
```

## Need Help?

1. Check `docs/` for detailed guides
2. Open an issue on GitHub
3. Review the API at `/api/v1/health`
