# Key Exchange Flow

## Overview

Omni Core uses X25519 (Curve25519 ECDH) for key exchange and ChaCha20-Poly1305 for symmetric encryption.

## Registration Flow

```
┌──────────┐                              ┌──────────┐
│  Client  │                              │  Server  │
└────┬─────┘                              └────┬─────┘
     │                                         │
     │  1. POST /register/init                 │
     │     { client_id: "device-001" }         │
     │────────────────────────────────────────►│
     │                                         │
     │                    2. Generate X25519   │
     │                       keypair for       │
     │                       this client       │
     │                                         │
     │                    3. Save to           │
     │                       server_keys.yaml  │
     │                                         │
     │  4. { server_public_key: "abc..." }     │
     │◄────────────────────────────────────────│
     │                                         │
     │  5. Generate own                        │
     │     X25519 keypair                      │
     │                                         │
     │  6. Save to localStorage                │
     │                                         │
     │  7. POST /register/complete             │
     │     { client_id, public_key }           │
     │────────────────────────────────────────►│
     │                                         │
     │                    8. Save client key   │
     │                       to client_config  │
     │                                         │
     │  9. { api_key, registered: true }       │
     │◄────────────────────────────────────────│
     │                                         │
```

## Encrypted Communication

After registration, both parties can derive the same shared secret:

```
Client Secret + Server Public = Shared Secret
Server Secret + Client Public = Shared Secret (same!)
```

### Encryption Process

1. **Derive shared secret** using ECDH
2. **Generate random nonce** (12 bytes)
3. **Encrypt** with ChaCha20-Poly1305
4. **Send** nonce + ciphertext

### Message Format

```json
{
  "nonce": "base64_encoded_12_bytes",
  "ciphertext": "base64_encoded_encrypted_data"
}
```

## Key Storage

### Server Side

**server_keys.yaml:**
```yaml
keys:
  device-001:
    client_id: device-001
    public_key: "abc123..."
    secret_key: "def456..."  # Encrypted in production
    created_at: "2024-12-14T22:00:00Z"
```

**client_config.yaml:**
```yaml
clients:
  device-001:
    client_id: device-001
    client_public_key: "xyz789..."
    server_key_id: device-001
    registered_at: "2024-12-14T22:00:00Z"
    last_seen: "2024-12-14T22:30:00Z"
```

### Client Side (localStorage)

```json
{
  "omni_client_keys": [
    {
      "id": "device-001",
      "publicKey": "xyz789...",
      "privateKey": "hidden...",
      "createdAt": "2024-12-14T22:00:00Z"
    }
  ]
}
```

## Security Properties

| Property | Guarantee |
|----------|-----------|
| Forward Secrecy | Per-client keypairs |
| Authentication | AEAD (Poly1305 MAC) |
| Confidentiality | ChaCha20 stream cipher |
| Integrity | Poly1305 authentication |
| Replay Protection | Random nonce per message |
