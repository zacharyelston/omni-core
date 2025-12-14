# Omni Core - Architecture & Design Standards

## Overview

This document defines the architecture, design standards, and conventions for Omni Core. It serves as the authoritative reference for AI agents and developers building this system.

---

## ⚠️ FIRST PRINCIPLES - READ THIS FIRST

These rules take precedence over all other guidelines in this document.

### 1. Modularity Above All

```
RULE: Every file must have a single, clear responsibility.
      Use the filesystem as your organizational structure.
      If you're unsure where code belongs, create a new module.
```

### 2. File Size Limit

```
RULE: Maximum 500 lines per source file.
      If a file exceeds 500 lines, split it into smaller modules.
      No exceptions.
```

**Why?**
- Easier to understand, review, and test
- Faster compilation (Rust)
- Better git diffs and merge conflict resolution
- Forces good separation of concerns

**How to split:**
```
# Before (bad): page.tsx with 800 lines
page.tsx

# After (good): components/ directory with focused modules
components/
├── tabs/
│   ├── HomeTab.tsx
│   ├── RegisterTab.tsx
│   ├── KeysTab.tsx
│   ├── ServerTab.tsx
│   └── SettingsTab.tsx
├── common/
│   ├── QRDisplay.tsx
│   └── KeyDisplay.tsx
└── forms/
    ├── AdminLoginForm.tsx
    └── SettingsForm.tsx
```

### 3. Reduce Complexity

```
RULE: Prefer simple, readable code over clever code.
      If a function is hard to explain, refactor it.
      Cyclomatic complexity should stay low.
```

**Guidelines:**
- Functions: max 50 lines, ideally under 25
- Nesting: max 3 levels deep
- Parameters: max 5, use structs for more
- Dependencies: minimize coupling between modules

### 4. Commit and Test Often

```
RULE: Commit frequently with atomic, logical changes.
      Run tests before every commit.
      Each commit should leave the codebase in a working state.
```

**Commit workflow:**
```bash
# 1. Make small, focused change
# 2. Run tests
cargo test
npm test

# 3. Commit with descriptive message
git commit -m "feat(settings): add federation toggle to settings UI"

# 4. Repeat
```

### 5. Push Only When It Works

```
RULE: Never push broken code to shared branches.
      All tests must pass before pushing.
      CI should never be red on main/develop.
```

**Push workflow:**
```bash
# 1. Run full test suite
cargo test --all
cd frontend && npm test && npm run build

# 2. Run lints
cargo clippy -- -D warnings
npm run lint

# 3. Only then push
git push
```

### Quick Reference

| Rule | Limit | Action if Exceeded |
|------|-------|-------------------|
| File size | 500 lines | Split into modules |
| Function size | 50 lines | Extract helper functions |
| Nesting depth | 3 levels | Refactor or early return |
| Parameters | 5 params | Use config/options struct |
| Commit frequency | Every logical change | Commit more often |
| Push condition | All tests pass | Fix before pushing |

---

## Technology Stack

### Core Languages

| Component | Language | Runtime/Framework | Rationale |
|-----------|----------|-------------------|-----------|
| **Backend API** | Rust | Axum + Tokio | Performance, memory safety, single binary |
| **Core Library** | Rust | - | Shared logic between server and clients |
| **Frontend** | TypeScript | Next.js 14 | PWA, React ecosystem |
| **Config** | YAML | serde_yaml | Human-readable, easy to edit |

### Infrastructure

| Component | Technology | Notes |
|-----------|------------|-------|
| Storage | YAML files | `config.d/`, `clients.d/`, `servers.d/` |
| Crypto | X25519 + ChaCha20Poly1305 | Key exchange + encryption |
| Container | Docker | Single-container deployment |
| CI/CD | GitHub Actions | Automated testing and releases |

---

## Project Structure

```
omni-core/
├── .github/
│   └── workflows/
│       ├── backend.yml         # Rust CI
│       ├── frontend.yml        # TypeScript CI
│       ├── docker.yml          # Docker builds
│       └── release.yml         # Release automation
│
├── backend/                    # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs           # Configuration
│   │   ├── api/                # Route handlers
│   │   │   ├── mod.rs
│   │   │   ├── admin.rs
│   │   │   ├── auth.rs
│   │   │   ├── health.rs
│   │   │   ├── keys.rs
│   │   │   ├── register.rs
│   │   │   ├── servers.rs
│   │   │   └── settings.rs
│   │   └── services/           # Business logic
│   │       ├── mod.rs
│   │       ├── admin.rs
│   │       ├── client_store.rs
│   │       ├── crypto.rs
│   │       ├── keystore.rs
│   │       ├── server_config.rs
│   │       ├── server_registry.rs
│   │       ├── session.rs
│   │       └── sync.rs
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── tests/
│
├── frontend/                   # TypeScript frontend
│   ├── src/
│   │   ├── app/               # Next.js app router
│   │   ├── components/        # React components (TODO: extract)
│   │   ├── hooks/             # Custom hooks (TODO: create)
│   │   ├── lib/               # Utilities (TODO: create)
│   │   └── types/             # TypeScript types (TODO: create)
│   ├── public/
│   ├── package.json
│   ├── tsconfig.json
│   └── Dockerfile
│
├── data/                       # Runtime data
│   ├── config.d/              # Server configuration
│   │   ├── admin.yaml
│   │   └── server-config.yaml
│   ├── clients.d/             # Client registrations
│   └── servers.d/             # Known servers
│
├── scripts/                    # Development scripts
│   ├── setup-hooks.sh
│   ├── test.sh
│   ├── lint.sh
│   └── docker-test.sh
│
├── docs/                       # Documentation
│   ├── Home.md
│   ├── Architecture.md
│   ├── API-Reference.md
│   ├── Quick-Start.md
│   ├── Integration-Guide.md
│   ├── Project-Review.md
│   └── RELEASE_SCHEDULE.md
│
├── Cargo.toml                  # Rust workspace root
├── Cargo.lock
├── Makefile
├── docker-compose.yml
├── docker-compose.test.yml
├── ARCHITECTURE.md             # This file
├── CONTRIBUTING.md             # Contribution guidelines
├── README.md
└── LICENSE
```

---

## Design Principles

### 1. Separation of Concerns

```
┌─────────────────────────────────────────────────────────────┐
│                      Presentation Layer                      │
│                   (Frontend, API Routes)                     │
├─────────────────────────────────────────────────────────────┤
│                      Application Layer                       │
│                   (Services, Use Cases)                      │
├─────────────────────────────────────────────────────────────┤
│                       Domain Layer                           │
│              (Core Types, Business Logic)                    │
├─────────────────────────────────────────────────────────────┤
│                    Infrastructure Layer                      │
│            (Storage, External APIs, Crypto)                  │
└─────────────────────────────────────────────────────────────┘
```

### 2. Error Handling

All errors must be:
- **Typed**: Use `thiserror` for Rust, custom error types for TypeScript
- **Traceable**: Include context for debugging
- **User-friendly**: Provide actionable messages

```rust
// Rust error example
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid public key format: expected 64 hex characters")]
    InvalidPublicKey,
    
    #[error("Key derivation failed: {reason}")]
    KeyDerivationFailed { reason: String },
    
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
}
```

### 3. Configuration

All configuration via YAML files with sensible defaults:

```yaml
# data/config.d/server-config.yaml
server:
  id: "omni-server-001"
  name: "My Omni Server"
  description: "Personal authentication server"
  version: "0.1.0"

network:
  host: "0.0.0.0"
  port: 8080
  public_url: ""

auth:
  session_ttl_secs: 3600
  admin_session_multiplier: 24

federation:
  enabled: true
  public: true
  sync_interval_secs: 3600
  max_known_servers: 1000
```

### 4. API Design

RESTful with consistent patterns:

```
GET    /api/v1/resources          # List
POST   /api/v1/resources          # Create
GET    /api/v1/resources/:id      # Read
PUT    /api/v1/resources/:id      # Update (full)
DELETE /api/v1/resources/:id      # Delete
```

Response format:
```json
{
  "success": true,
  "data": { ... }
}
```

Error format:
```json
{
  "success": false,
  "error": "Human readable message"
}
```

---

## Testing Standards

### Test Pyramid

```
        ┌─────────┐
        │   E2E   │  10% - Critical user journeys
        ├─────────┤
        │ Integr. │  20% - API, storage, external services
        ├─────────┤
        │  Unit   │  70% - Functions, modules, components
        └─────────┘
```

### Rust Testing

```rust
// Unit test - same file
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_session_creation() {
        let store = SessionStore::new();
        let session = store.create("client-001");
        assert!(!session.api_key.is_empty());
    }
}

// Integration test - tests/ directory
#[tokio::test]
async fn test_registration_flow() {
    let app = spawn_test_app().await;
    let client = reqwest::Client::new();
    
    // Init registration
    let response = client
        .post(&format!("{}/api/v1/register/init", app.address))
        .json(&json!({ "client_id": "test-client" }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 200);
}
```

### TypeScript Testing

```typescript
// Unit test - Vitest
import { describe, it, expect } from 'vitest';
import { api } from './api';

describe('API Client', () => {
  it('should fetch server info', async () => {
    const info = await api.getServerInfo();
    expect(info.server_name).toBeDefined();
  });
});
```

---

## Code Style

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` with default settings
- All public items must have doc comments
- Prefer `thiserror` for error types
- Use `tracing` for logging

```rust
/// Manages client sessions with automatic expiry.
/// 
/// # Example
/// ```
/// let store = SessionStore::new();
/// let session = store.create("client-001");
/// assert!(store.validate(&session.api_key).is_some());
/// ```
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}
```

### TypeScript

- Use ESLint with `@typescript-eslint`
- Use Prettier for formatting
- Strict TypeScript (`strict: true`)
- Prefer `type` over `interface` for consistency
- Use barrel exports (`index.ts`)

```typescript
// types/server.ts
export type ServerConfig = {
  server: ServerSettings;
  network: NetworkSettings;
  auth: AuthSettings;
  federation: FederationSettings;
};

// types/index.ts
export * from './server';
export * from './client';
```

---

## Security Standards

### Authentication

- API keys are generated with cryptographic randomness
- Keys never logged or exposed in errors
- Session expiry enforced server-side
- Admin keys have extended TTL

### Cryptography

- X25519 for key exchange
- ChaCha20Poly1305 for symmetric encryption
- Per-client server keypairs
- Shared secrets derived via Diffie-Hellman

### Data Protection

- HTTPS required in production
- Sensitive config via environment variables
- No secrets in code or logs
- Keys stored in separate files per client

---

## Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(federation): add server-to-server sync

Implements hourly synchronization of public server lists
between federated Omni Core instances.

Closes #45
```

---

## AI Agent Instructions

When implementing features:

1. **Read this document first** - Understand the architecture
2. **Check RELEASE_SCHEDULE.md** - Follow the phase order
3. **Write tests first** - TDD approach
4. **Follow code style** - Run formatters and linters
5. **Update documentation** - Keep docs in sync
6. **Small commits** - One logical change per commit
7. **Run CI locally** - `make test`

### File Size Check

Before committing, verify no file exceeds 500 lines:
```bash
find backend/src -name "*.rs" -exec wc -l {} \; | awk '$1 > 500 {print "WARNING: " $2 " has " $1 " lines"}'
find frontend/src -name "*.tsx" -exec wc -l {} \; | awk '$1 > 500 {print "WARNING: " $2 " has " $1 " lines"}'
```
