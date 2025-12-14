# Omni Core Integration Guide

This guide helps you integrate Omni Core authentication into your project.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Client Integration](#client-integration)
3. [Server-to-Server Federation](#server-to-server-federation)
4. [Security Best Practices](#security-best-practices)
5. [Example Implementations](#example-implementations)

---

## Quick Start

### Option 1: Docker (Recommended)

```bash
# Clone the repo
git clone https://github.com/zacharyelston/omni-core.git
cd omni-core

# Start with Docker
docker-compose up -d

# Backend: http://localhost:8080
# Frontend: http://localhost:3000
```

### Option 2: Local Development

```bash
# Backend
cd backend
cargo run

# Frontend (separate terminal)
cd frontend
npm install
npm run dev
```

### First Run

On first startup, the server generates:
- **Server keypair** - Unique X25519 keypair for this server
- **Admin key** - Printed to console, save it securely!
- **Server ID** - Derived from public key (`srv_<first16chars>`)

---

## Client Integration

### Step 1: Get Server Public Key

```bash
curl http://localhost:8080/api/v1/server/info
```

Response:
```json
{
  "server_public_key": "77e7ccccabaf5950e0e627c87451c3606824a607794d0eaadaa8c8c20bd49d11",
  "server_name": "Omni Core Server",
  "version": "0.1.0"
}
```

### Step 2: Initialize Registration

```bash
curl -X POST http://localhost:8080/api/v1/register/init \
  -H "Content-Type: application/json" \
  -d '{"client_id": "my-app-001"}'
```

Response:
```json
{
  "client_id": "my-app-001",
  "server_public_key": "abc123...",
  "message": "Registration initiated. Server generated a unique keypair for this client."
}
```

### Step 3: Complete Registration

```bash
curl -X POST http://localhost:8080/api/v1/register/complete \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "my-app-001",
    "encrypted_client_public_key": {
      "nonce": "",
      "ciphertext": "<base64_encoded_hex_public_key>"
    }
  }'
```

Response:
```json
{
  "client_id": "my-app-001",
  "registered": true,
  "api_key": "omni_abc123...",
  "message": "Registration complete"
}
```

### Step 4: Use the API Key

Include the API key in subsequent requests:

```bash
curl -X POST http://localhost:8080/api/v1/auth/verify \
  -H "Content-Type: application/json" \
  -d '{"api_key": "omni_abc123..."}'
```

---

## Client SDK Examples

### JavaScript/TypeScript

```typescript
class OmniClient {
  private baseUrl: string;
  private clientId: string;
  private apiKey: string | null = null;

  constructor(baseUrl: string, clientId: string) {
    this.baseUrl = baseUrl;
    this.clientId = clientId;
  }

  async getServerInfo() {
    const res = await fetch(`${this.baseUrl}/api/v1/server/info`);
    return res.json();
  }

  async register() {
    // Step 1: Init registration
    const initRes = await fetch(`${this.baseUrl}/api/v1/register/init`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ client_id: this.clientId })
    });
    const initData = await initRes.json();
    
    // Step 2: Generate client keypair (use a crypto library)
    // const clientKeypair = generateX25519Keypair();
    
    // Step 3: Complete registration
    const completeRes = await fetch(`${this.baseUrl}/api/v1/register/complete`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        client_id: this.clientId,
        encrypted_client_public_key: {
          nonce: '',
          ciphertext: btoa(clientPublicKeyHex) // base64 encode
        }
      })
    });
    const completeData = await completeRes.json();
    this.apiKey = completeData.api_key;
    return completeData;
  }

  async verifySession() {
    if (!this.apiKey) throw new Error('Not registered');
    const res = await fetch(`${this.baseUrl}/api/v1/auth/verify`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ api_key: this.apiKey })
    });
    return res.json();
  }
}

// Usage
const client = new OmniClient('http://localhost:8080', 'my-app');
await client.register();
const session = await client.verifySession();
```

### Rust

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RegisterInit {
    client_id: String,
}

#[derive(Deserialize)]
struct RegisterInitResponse {
    client_id: String,
    server_public_key: String,
}

#[derive(Serialize)]
struct RegisterComplete {
    client_id: String,
    encrypted_client_public_key: EncryptedKey,
}

#[derive(Serialize)]
struct EncryptedKey {
    nonce: String,
    ciphertext: String,
}

#[derive(Deserialize)]
struct RegisterCompleteResponse {
    client_id: String,
    registered: bool,
    api_key: String,
}

pub struct OmniClient {
    base_url: String,
    client_id: String,
    http: Client,
    api_key: Option<String>,
}

impl OmniClient {
    pub fn new(base_url: &str, client_id: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client_id: client_id.to_string(),
            http: Client::new(),
            api_key: None,
        }
    }

    pub async fn register(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // Init
        let init_res: RegisterInitResponse = self.http
            .post(format!("{}/api/v1/register/init", self.base_url))
            .json(&RegisterInit { client_id: self.client_id.clone() })
            .send()
            .await?
            .json()
            .await?;

        // Generate keypair and complete registration
        // ... (use x25519-dalek for key generation)

        let complete_res: RegisterCompleteResponse = self.http
            .post(format!("{}/api/v1/register/complete", self.base_url))
            .json(&RegisterComplete {
                client_id: self.client_id.clone(),
                encrypted_client_public_key: EncryptedKey {
                    nonce: String::new(),
                    ciphertext: base64_encoded_public_key,
                },
            })
            .send()
            .await?
            .json()
            .await?;

        self.api_key = Some(complete_res.api_key.clone());
        Ok(complete_res.api_key)
    }
}
```

### Python

```python
import requests
import base64

class OmniClient:
    def __init__(self, base_url: str, client_id: str):
        self.base_url = base_url
        self.client_id = client_id
        self.api_key = None

    def get_server_info(self):
        res = requests.get(f"{self.base_url}/api/v1/server/info")
        return res.json()

    def register(self):
        # Step 1: Init
        init_res = requests.post(
            f"{self.base_url}/api/v1/register/init",
            json={"client_id": self.client_id}
        )
        init_data = init_res.json()
        server_public_key = init_data["server_public_key"]

        # Step 2: Generate client keypair
        # from cryptography.hazmat.primitives.asymmetric.x25519 import X25519PrivateKey
        # private_key = X25519PrivateKey.generate()
        # public_key = private_key.public_key()

        # Step 3: Complete registration
        complete_res = requests.post(
            f"{self.base_url}/api/v1/register/complete",
            json={
                "client_id": self.client_id,
                "encrypted_client_public_key": {
                    "nonce": "",
                    "ciphertext": base64.b64encode(public_key_hex.encode()).decode()
                }
            }
        )
        complete_data = complete_res.json()
        self.api_key = complete_data["api_key"]
        return complete_data

    def verify_session(self):
        res = requests.post(
            f"{self.base_url}/api/v1/auth/verify",
            json={"api_key": self.api_key}
        )
        return res.json()

# Usage
client = OmniClient("http://localhost:8080", "my-python-app")
client.register()
print(client.verify_session())
```

---

## Server-to-Server Federation

Omni Core servers can discover and communicate with each other.

### Register Your Server with Another

```bash
# Get your server's info
MY_INFO=$(curl -s http://localhost:8080/api/v1/server/info)
MY_KEY=$(echo $MY_INFO | jq -r '.server_public_key')

# Register with remote server
curl -X POST http://remote-server:8080/api/v1/servers/register \
  -H "Content-Type: application/json" \
  -d "{
    \"server_id\": \"my-server-id\",
    \"name\": \"My Server\",
    \"description\": \"My Omni Core instance\",
    \"public_url\": \"http://my-server:8080\",
    \"public_key\": \"$MY_KEY\",
    \"is_public\": true
  }"
```

### List Known Servers

```bash
# Public servers (no auth)
curl http://localhost:8080/api/v1/servers/public

# All servers (admin)
curl http://localhost:8080/api/v1/servers/all

# Server stats
curl http://localhost:8080/api/v1/servers/stats
```

### Automatic Sync

Servers automatically sync their known server lists hourly with authenticated peers. Configure with:

```bash
# Set sync interval (in seconds)
export SYNC_INTERVAL_SECS=1800  # 30 minutes
```

---

## Security Best Practices

### 1. Protect the Admin Key

The admin key is generated on first run and printed to the console. Store it securely:

```bash
# Bad: Don't put in code or version control
ADMIN_KEY="admin_abc123..."  # NO!

# Good: Use environment variables or secrets manager
export OMNI_ADMIN_KEY=$(cat /run/secrets/admin_key)
```

### 2. Use HTTPS in Production

```yaml
# docker-compose.prod.yml
services:
  backend:
    environment:
      - HTTPS_ONLY=true
```

### 3. Rotate Keys Periodically

```bash
# Generate new server keypair (requires restart)
rm data/config.d/admin.yaml
# Restart server - new keys will be generated
```

### 4. Validate Client Public Keys

Always verify the client's public key format before accepting:

```rust
// Server-side validation
fn validate_public_key(hex: &str) -> bool {
    hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit())
}
```

### 5. Use Per-Client Keypairs

Each client gets a unique server keypair. This means:
- Compromising one client doesn't affect others
- You can revoke individual clients
- Better audit trail

---

## Data Storage

### Directory Structure

```
data/
├── config.d/
│   ├── server-config.yaml    # Main server config
│   └── admin.yaml            # Admin key and server ID
├── clients.d/
│   └── {client_id}.yaml      # One file per client
└── servers.d/
    └── {server_id}.yaml      # One file per known server
```

### Client Config Format

```yaml
# data/clients.d/my-app-001.yaml
client_id: my-app-001
public_key: "abc123..."
server_secret_key: "def456..."  # Server's secret for this client
registered_at: "2024-12-14T22:00:00Z"
last_seen: "2024-12-14T23:00:00Z"
registration_complete: true
```

### Server Config Format

```yaml
# data/servers.d/server-2.yaml
server_id: server-2
name: "Remote Server"
public_url: "https://remote.example.com"
public_key: "abc123..."
is_public: true
is_authenticated: false
discovered_at: "2024-12-14T22:00:00Z"
trust_level: 50
```

---

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HOST` | `0.0.0.0` | Server bind address |
| `PORT` | `8080` | Server port |
| `RUST_LOG` | `info` | Log level |
| `SYNC_INTERVAL_SECS` | `3600` | Server sync interval |
| `SERVER_NAME` | `Omni Core Server` | Display name |

---

## Troubleshooting

### "Client already registered"

The client ID is already in use. Either:
- Use a different client ID
- Delete `data/clients.d/{client_id}.yaml` to reset

### "No pending registration"

Registration must be completed within the session. Start over with `/register/init`.

### "Server not authenticated"

For sync operations, servers must be mutually authenticated. Register your server with the remote server first.

### Connection refused

Check that:
1. Server is running (`curl http://localhost:8080/api/v1/health`)
2. Firewall allows the port
3. Docker network is configured correctly

---

## Next Steps

1. **Read the API Reference** - `docs/API-Reference.md`
2. **Understand the Architecture** - `docs/Architecture.md`
3. **Review Key Exchange Flow** - `docs/Key-Exchange-Flow.md`
4. **Deploy to Production** - `docs/Deployment.md`

---

## Support

- **GitHub Issues**: [github.com/zacharyelston/omni-core/issues](https://github.com/zacharyelston/omni-core/issues)
- **Documentation**: `docs/` folder
