# Omni Core Project Review & Recommendations

## Executive Summary

Omni Core is a well-structured authentication and session management system. This review identifies areas for improvement in modularity, code organization, and automation to make the project more maintainable and extensible.

---

## Current Architecture Assessment

### Strengths ✅

1. **Clean Separation of Concerns**
   - `api/` - HTTP handlers
   - `services/` - Business logic
   - `config.rs` - Configuration
   - Clear module boundaries

2. **Good Testing Foundation**
   - 30 unit tests passing
   - Test files co-located with source (`*_test.rs`)
   - Pre-commit hooks for quality gates

3. **Modern Stack**
   - Rust + Axum (performant, type-safe)
   - Next.js + React (modern frontend)
   - Docker support for deployment

4. **Documentation**
   - 9 docs covering architecture, API, deployment
   - Integration guide for new projects

### Areas for Improvement 🔧

---

## 1. Backend Modularity

### Current Issues

**Problem: Monolithic `page.tsx` (817 lines)**
```
frontend/src/app/page.tsx - Single file with all UI logic
```

**Problem: Overlapping Services**
- `keystore.rs` and `client_store.rs` have similar functionality
- `server_config.rs` duplicates some `config.rs` patterns

**Problem: AppState Growing**
```rust
pub struct AppState {
    pub config: Arc<Config>,
    pub sessions: SessionStore,
    pub server_keypair: Arc<ServerKeyPair>,
    pub keystore: KeyStoreManager,
    pub client_store: ClientStore,      // Similar to keystore
    pub server_registry: ServerRegistry,
    pub admin: AdminAuth,
    pub server_config: ConfigManager,   // Similar to config
}
```

### Recommendations

#### 1.1 Extract Crates for Reusability

```toml
# Cargo.toml - Proposed workspace structure
[workspace]
members = [
    "crates/omni-core",      # Core types and traits
    "crates/omni-crypto",    # Crypto primitives
    "crates/omni-storage",   # Storage abstractions
    "crates/omni-server",    # HTTP server
    "crates/omni-client",    # Client SDK
]
```

**Benefits:**
- Other projects can depend on `omni-core` or `omni-crypto` without the full server
- Clearer dependency boundaries
- Faster incremental builds

#### 1.2 Introduce Traits for Storage

```rust
// crates/omni-storage/src/lib.rs
pub trait EntityStore<T> {
    fn get(&self, id: &str) -> Option<T>;
    fn save(&self, entity: &T) -> Result<(), StorageError>;
    fn delete(&self, id: &str) -> Result<(), StorageError>;
    fn list(&self) -> Vec<T>;
}

// Implementations
pub struct YamlStore<T> { ... }
pub struct SqliteStore<T> { ... }  // Future
pub struct PostgresStore<T> { ... } // Future
```

#### 1.3 Consolidate Config Management

```rust
// Merge config.rs and server_config.rs
pub struct Config {
    // Runtime config (env vars)
    pub host: String,
    pub port: u16,
    
    // Persistent config (YAML)
    #[serde(flatten)]
    pub server: ServerConfig,
}
```

---

## 2. Frontend Modularity

### Current Issues

**Problem: Single 817-line component**
- All state in one component
- Hard to test individual features
- No code splitting

### Recommendations

#### 2.1 Component Extraction

```
frontend/src/
├── app/
│   └── page.tsx              # Just layout + routing
├── components/
│   ├── tabs/
│   │   ├── HomeTab.tsx
│   │   ├── RegisterTab.tsx
│   │   ├── KeysTab.tsx
│   │   ├── ServerTab.tsx
│   │   └── SettingsTab.tsx
│   ├── common/
│   │   ├── QRDisplay.tsx
│   │   ├── KeyDisplay.tsx
│   │   └── StatusBadge.tsx
│   └── forms/
│       ├── AdminLoginForm.tsx
│       └── SettingsForm.tsx
├── hooks/
│   ├── useServerInfo.ts
│   ├── useSettings.ts
│   ├── useAuth.ts
│   └── useKnownServers.ts
├── lib/
│   ├── api.ts               # API client
│   └── crypto.ts            # Client-side crypto
└── types/
    └── index.ts             # Shared types
```

#### 2.2 State Management

```typescript
// hooks/useSettings.ts
export function useSettings() {
  const [config, setConfig] = useState<ServerConfig | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetch = async () => { ... };
  const save = async (config: ServerConfig) => { ... };

  return { config, loading, error, fetch, save };
}
```

#### 2.3 API Client Layer

```typescript
// lib/api.ts
class OmniAPI {
  private baseUrl: string;

  constructor(baseUrl = '/api/v1') {
    this.baseUrl = baseUrl;
  }

  async getServerInfo(): Promise<ServerInfo> { ... }
  async getSettings(): Promise<ServerConfig> { ... }
  async updateSettings(config: ServerConfig): Promise<void> { ... }
  async registerInit(clientId: string): Promise<RegisterInitResponse> { ... }
  // ... etc
}

export const api = new OmniAPI();
```

---

## 3. Automated Feature Building

### Current CI/CD

```yaml
# .github/workflows/backend.yml
- Check formatting
- Clippy
- Build
- Test
- Upload artifact
```

### Recommendations

#### 3.1 Add Integration Tests Workflow

```yaml
# .github/workflows/integration.yml
name: Integration Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  integration:
    runs-on: ubuntu-latest
    services:
      server1:
        image: ghcr.io/${{ github.repository }}/omni-server:latest
        ports:
          - 8081:8080
      server2:
        image: ghcr.io/${{ github.repository }}/omni-server:latest
        ports:
          - 8082:8080

    steps:
      - uses: actions/checkout@v4
      
      - name: Wait for servers
        run: |
          for i in {1..30}; do
            curl -s http://localhost:8081/api/v1/health && break
            sleep 1
          done

      - name: Run integration tests
        run: ./scripts/integration-test.sh
```

#### 3.2 Add Docker Image Publishing

```yaml
# .github/workflows/docker.yml
name: Docker Build

on:
  push:
    tags: ['v*']
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Login to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Build and push backend
        uses: docker/build-push-action@v5
        with:
          context: .
          file: backend/Dockerfile
          push: true
          tags: |
            ghcr.io/${{ github.repository }}/omni-server:latest
            ghcr.io/${{ github.repository }}/omni-server:${{ github.sha }}
```

#### 3.3 Add Release Automation

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Build binaries
        run: |
          cargo build --release --target x86_64-unknown-linux-gnu
          cargo build --release --target x86_64-apple-darwin
          cargo build --release --target aarch64-apple-darwin

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            target/x86_64-unknown-linux-gnu/release/omni-server
            target/x86_64-apple-darwin/release/omni-server
            target/aarch64-apple-darwin/release/omni-server
```

#### 3.4 Add Dependabot

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    groups:
      rust-dependencies:
        patterns:
          - "*"

  - package-ecosystem: "npm"
    directory: "/frontend"
    schedule:
      interval: "weekly"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

---

## 4. Feature Flag System

### Recommendation: Add Feature Flags

```rust
// crates/omni-core/src/features.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub federation_enabled: bool,
    pub public_registration: bool,
    pub admin_ui_enabled: bool,
    pub sync_enabled: bool,
    pub experimental_features: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            federation_enabled: true,
            public_registration: true,
            admin_ui_enabled: true,
            sync_enabled: true,
            experimental_features: false,
        }
    }
}
```

**Usage in API:**
```rust
pub async fn register_server(
    State(state): State<AppState>,
    Json(req): Json<RegisterServerRequest>,
) -> Result<Json<Response>, StatusCode> {
    if !state.features.federation_enabled {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    // ...
}
```

---

## 5. Plugin/Extension System

### Recommendation: Add Plugin Architecture

```rust
// crates/omni-core/src/plugin.rs
#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    async fn on_startup(&self, state: &AppState) -> Result<()>;
    async fn on_shutdown(&self, state: &AppState) -> Result<()>;
    
    fn routes(&self) -> Option<Router<AppState>> { None }
    fn middleware(&self) -> Option<BoxLayer> { None }
}

// Example plugin
pub struct MetricsPlugin;

#[async_trait]
impl Plugin for MetricsPlugin {
    fn name(&self) -> &str { "metrics" }
    fn version(&self) -> &str { "0.1.0" }
    
    fn routes(&self) -> Option<Router<AppState>> {
        Some(Router::new()
            .route("/metrics", get(prometheus_metrics)))
    }
}
```

---

## 6. OpenAPI/Swagger Generation

### Recommendation: Add Auto-Generated API Docs

```toml
# backend/Cargo.toml
[dependencies]
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "6", features = ["axum"] }
```

```rust
// backend/src/api/mod.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        admin::get_server_info,
        settings::get_settings,
        // ... all endpoints
    ),
    components(schemas(
        ServerInfo,
        ServerConfig,
        // ... all types
    ))
)]
pub struct ApiDoc;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // ... existing routes
}
```

---

## 7. Observability

### Recommendation: Add Metrics & Tracing

```rust
// backend/src/observability.rs
use metrics::{counter, histogram};
use tracing::instrument;

#[instrument(skip(state))]
pub async fn register_init(
    State(state): State<AppState>,
    Json(req): Json<RegisterInitRequest>,
) -> Result<Json<Response>, StatusCode> {
    counter!("omni_registrations_initiated").increment(1);
    
    let start = std::time::Instant::now();
    let result = do_registration(&state, &req).await;
    histogram!("omni_registration_duration_ms").record(start.elapsed().as_millis() as f64);
    
    result
}
```

```yaml
# docker-compose.yml addition
services:
  prometheus:
    image: prom/prometheus
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana
    ports:
      - "3001:3000"
```

---

## 8. Database Migration Path

### Current: YAML Files
### Future: SQLite → PostgreSQL

```rust
// crates/omni-storage/src/migrations.rs
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
```

```sql
-- migrations/001_initial.sql
CREATE TABLE clients (
    id TEXT PRIMARY KEY,
    public_key TEXT NOT NULL,
    server_secret_key TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL,
    last_seen TIMESTAMPTZ
);

CREATE TABLE servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    public_url TEXT NOT NULL,
    public_key TEXT NOT NULL,
    is_public BOOLEAN DEFAULT true,
    is_authenticated BOOLEAN DEFAULT false,
    discovered_at TIMESTAMPTZ NOT NULL
);
```

---

## Implementation Priority

| Priority | Item | Effort | Impact |
|----------|------|--------|--------|
| 🔴 High | Frontend component extraction | Medium | High |
| 🔴 High | API client layer | Low | High |
| 🟡 Medium | Crate extraction | High | High |
| 🟡 Medium | Docker publishing workflow | Low | Medium |
| 🟡 Medium | Integration tests | Medium | High |
| 🟢 Low | OpenAPI generation | Low | Medium |
| 🟢 Low | Plugin system | High | Medium |
| 🟢 Low | Database migration | High | Low (for now) |

---

## Quick Wins (Can Do Now)

1. **Add Dependabot** - 5 minutes
2. **Add Docker publish workflow** - 15 minutes
3. **Extract API client in frontend** - 30 minutes
4. **Add `/metrics` endpoint** - 30 minutes
5. **Add OpenAPI docs** - 1 hour

---

## Conclusion

Omni Core has a solid foundation. The main improvements needed are:

1. **Frontend refactoring** - Break up the monolithic component
2. **Crate extraction** - Enable reuse of core functionality
3. **CI/CD enhancement** - Add integration tests, Docker publishing, releases
4. **Observability** - Add metrics and better tracing

These changes will make the project more maintainable, testable, and ready for production use.
