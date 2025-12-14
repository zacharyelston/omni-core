# Deployment

## Replit Setup

### Backend Replit

1. Create new Replit → Import from GitHub
2. Select `zacharyelston/omni-core`
3. Replit auto-detects Rust from `.replit`
4. Click Run - server starts on port 8080

**Environment Variables:**
```
PORT=8080
RUST_LOG=info
SECRET_KEY=your-production-secret
SESSION_TTL=3600
```

### Frontend Replit

1. Create new Replit → Import from GitHub
2. Select `zacharyelston/omni-core`
3. In Shell: `cd frontend && npm install`
4. Set run command: `cd frontend && npm run dev`
5. Set `BACKEND_URL` environment variable

**Environment Variables:**
```
BACKEND_URL=https://your-backend.replit.app
```

## Local Development

### Backend
```bash
cd backend
cargo run
# http://localhost:8080
```

### Frontend
```bash
cd frontend
npm install
npm run dev
# http://localhost:5000
```

### Both Together
```bash
# Terminal 1
cd backend && cargo run

# Terminal 2
cd frontend && npm run dev
```

## Production Considerations

### Security Checklist

- [ ] Change `SECRET_KEY` from default
- [ ] Enable HTTPS
- [ ] Set appropriate `SESSION_TTL`
- [ ] Consider encrypting `server_keys.yaml` secret keys
- [ ] Add rate limiting
- [ ] Add request logging

### Data Persistence

The `data/` directory contains:
- `server_keys.yaml` - Server keypairs (CRITICAL)
- `client_config.yaml` - Client registrations

**Backup these files regularly!**

### Scaling

Current architecture uses in-memory sessions. For horizontal scaling:
- Use Redis for session storage
- Use PostgreSQL for key storage
- Add load balancer with sticky sessions

## Docker (Future)

```dockerfile
# backend/Dockerfile
FROM rust:1.75-slim
WORKDIR /app
COPY . .
RUN cargo build --release
EXPOSE 8080
CMD ["./target/release/omni-server"]
```

```dockerfile
# frontend/Dockerfile
FROM node:20-slim
WORKDIR /app
COPY frontend/ .
RUN npm install && npm run build
EXPOSE 5000
CMD ["npm", "start"]
```
