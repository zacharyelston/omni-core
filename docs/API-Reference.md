# API Reference

Base URL: `http://localhost:8080/api/v1`

## Health

### GET /health
Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2024-12-14T22:00:00Z"
}
```

---

## Authentication

### POST /auth/join
Create a new session and receive an API key.

**Response:**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "api_key": "omni_abc123...",
  "expires_at": "2024-12-14T23:00:00Z"
}
```

### POST /auth/verify
Verify an API key is valid.

**Request:**
```json
{
  "api_key": "omni_abc123..."
}
```

**Response:**
```json
{
  "valid": true,
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "expires_at": "2024-12-14T23:00:00Z"
}
```

### POST /auth/logout
Invalidate a session.

**Request:**
```json
{
  "api_key": "omni_abc123..."
}
```

**Response:**
```json
{
  "success": true
}
```

---

## Key Exchange (Legacy)

### GET /keys/public
Get server's static public key.

**Response:**
```json
{
  "public_key": "abc123def456..."
}
```

### POST /keys/exchange
Exchange keys and create session.

**Request:**
```json
{
  "client_public_key": "abc123def456..."
}
```

**Response:**
```json
{
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "api_key": "omni_abc123...",
  "expires_at": "2024-12-14T23:00:00Z",
  "server_public_key": "def456abc123..."
}
```

### POST /keys/send
Send encrypted message.

**Request:**
```json
{
  "client_public_key": "abc123def456...",
  "payload": {
    "nonce": "base64_nonce",
    "ciphertext": "base64_ciphertext"
  }
}
```

**Response:**
```json
{
  "payload": {
    "nonce": "base64_nonce",
    "ciphertext": "base64_ciphertext"
  }
}
```

---

## Registration (Per-Client Keys)

### POST /register/init
Start registration with client ID. Server generates a unique keypair for this client.

**Request:**
```json
{
  "client_id": "my-device-001"
}
```

**Response:**
```json
{
  "client_id": "my-device-001",
  "server_public_key": "abc123def456...",
  "message": "Encrypt your public key with this server key..."
}
```

**Errors:**
- `409 Conflict` - Client already registered

### POST /register/complete
Complete registration with client's public key.

**Request:**
```json
{
  "client_id": "my-device-001",
  "encrypted_client_public_key": {
    "nonce": "",
    "ciphertext": "base64_encoded_hex_public_key"
  }
}
```

**Response:**
```json
{
  "client_id": "my-device-001",
  "registered": true,
  "api_key": "omni_abc123...",
  "message": "Registration complete..."
}
```

**Errors:**
- `404 Not Found` - No pending registration
- `400 Bad Request` - Invalid public key format

### GET /register/clients
List all registered clients.

**Response:**
```json
{
  "clients": [
    {
      "client_id": "my-device-001",
      "registered_at": "2024-12-14T22:00:00Z",
      "last_seen": "2024-12-14T22:30:00Z"
    }
  ]
}
```

### GET /register/keys
List all server public keys (one per client).

**Response:**
```json
{
  "keys": [
    {
      "client_id": "my-device-001",
      "public_key": "abc123def456..."
    }
  ]
}
```

---

## Server Info & Admin

### GET /server/info
Get public server information (no auth required).

**Response:**
```json
{
  "server_public_key": "abc123def456...",
  "server_name": "Omni Core Server",
  "version": "0.1.0"
}
```

### POST /admin/login
Authenticate as admin.

**Request:**
```json
{
  "admin_key": "admin_abc123..."
}
```

**Response:**
```json
{
  "authenticated": true,
  "message": "Admin session created. API key: omni_..."
}
```

### GET /admin/dashboard
Get admin dashboard data.

**Response:**
```json
{
  "total_clients": 5,
  "total_server_keys": 5,
  "server_public_key": "abc123def456..."
}
```

---

## Server Federation

Servers can register with each other and share their known server lists, creating a DNS-like discovery mechanism.

### GET /servers/public
List all public servers (no auth required).

**Response:**
```json
{
  "servers": [
    {
      "server_id": "srv_abc123...",
      "name": "Remote Server",
      "description": "A remote Omni Core server",
      "public_url": "https://remote.example.com",
      "public_key": "def456...",
      "version": "0.1.0"
    }
  ],
  "total": 1
}
```

### POST /servers/register
Register a new server.

**Request:**
```json
{
  "server_id": "srv_abc123...",
  "name": "My Server",
  "description": "My Omni Core server",
  "public_url": "https://my.example.com",
  "public_key": "abc123...",
  "is_public": true
}
```

**Response:**
```json
{
  "success": true,
  "message": "Server registered successfully",
  "our_server_id": "srv_def456...",
  "our_public_key": "def456..."
}
```

### POST /servers/sync
Sync server lists with an authenticated server.

**Request:**
```json
{
  "requesting_server_id": "srv_abc123...",
  "requesting_server_key": "abc123..."
}
```

**Response:**
```json
{
  "servers": [...],
  "total": 5
}
```

**Errors:**
- `401 Unauthorized` - Server not authenticated

### GET /servers/stats
Get server statistics.

**Response:**
```json
{
  "total_servers": 10,
  "public_servers": 8,
  "authenticated_servers": 3
}
```

### GET /servers/all
List all known servers (admin only).

**Response:**
```json
[
  {
    "server_id": "srv_abc123...",
    "name": "Remote Server",
    "public_url": "https://remote.example.com",
    "public_key": "def456...",
    "is_public": true,
    "is_authenticated": false,
    "discovered_at": "2024-12-14T22:00:00Z",
    "last_seen": null,
    "last_sync": null,
    "version": "0.1.0",
    "trust_level": 50
  }
]
```
