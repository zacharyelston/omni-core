# Release Schedule

This document outlines the phased release plan for Omni Core, including infrastructure components and feature milestones.

## Semantic Versioning

We follow [Semantic Versioning](https://semver.org/): `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Version Roadmap

```
v0.1.0 ──► v0.2.0 ──► v0.3.0 ──► v0.4.0 ──► v1.0.0 ──► v1.1.0 ──► v1.2.0
  │          │          │          │          │          │          │
Phase 1   Phase 2   Phase 3   Phase 4      GA      Phase 5   Phase 6
 Core     Frontend   Config   Federation RELEASE   Plugins   Multi-
Backend   Refactor    UI       Sync                          Tenant
```

| Version | Phase | Target | Description |
|---------|-------|--------|-------------|
| **v0.1.0** | 1 | Dec 2024 | Core backend API (auth, sessions, keys) |
| **v0.2.0** | 2 | Jan 2025 | Frontend refactoring (components, hooks) |
| **v0.3.0** | 3 | Jan 2025 | Settings UI + Config management |
| **v0.4.0** | 4 | Feb 2025 | Federation sync + Server discovery |
| **v1.0.0** | 4 | Feb 2025 | **🎉 GA Release** - Production ready |
| **v1.1.0** | 5 | Mar 2025 | Plugin system + Metrics |
| **v1.2.0** | 6 | Apr 2025 | Multi-tenant support |

### What v1.0.0 Means

**v1.0.0** is the first **General Availability (GA)** release with:

✅ **Backend**
- Client registration with X25519 key exchange
- Session management with expiry
- Admin authentication
- Server federation (discovery + sync)
- Settings API for runtime configuration

✅ **Frontend**
- Mobile-responsive PWA
- Tabbed UI (Home, Register, Keys, Server, Settings)
- QR code display for keys
- Admin dashboard

✅ **Infrastructure**
- Docker Compose deployment
- GitHub Actions CI/CD
- Automated releases
- Documentation

---

## GitHub Integration

### Project Board

**Project URL**: https://github.com/zacharyelston/omni-core/projects/1

| Column | Description |
|--------|-------------|
| **Backlog** | All planned work |
| **To Do** | Ready for current sprint |
| **In Progress** | Actively being worked on |
| **Review** | PRs awaiting review |
| **Done** | Completed and merged |

### Issue Labels

| Label | Color | Description |
|-------|-------|-------------|
| `phase-1` | `#0052CC` | Core backend |
| `phase-2` | `#0052CC` | Frontend refactor |
| `phase-3` | `#0052CC` | Config UI |
| `phase-4` | `#0052CC` | Federation |
| `phase-5` | `#0052CC` | Plugins |
| `phase-6` | `#0052CC` | Multi-tenant |
| `bug` | `#d73a4a` | Something isn't working |
| `enhancement` | `#a2eeef` | New feature |
| `documentation` | `#0075ca` | Documentation updates |
| `backend` | `#5319e7` | Rust backend |
| `frontend` | `#1d76db` | TypeScript frontend |
| `priority-high` | `#b60205` | Critical path |
| `priority-medium` | `#fbca04` | Important |
| `priority-low` | `#0e8a16` | Nice to have |

### Milestones

| Milestone | Version | Due Date | Issues |
|-----------|---------|----------|--------|
| M1: Core Backend | v0.1.0 | Dec 31, 2024 | #1-#10 |
| M2: Frontend Refactor | v0.2.0 | Jan 15, 2025 | #11-#20 |
| M3: Config UI | v0.3.0 | Jan 31, 2025 | #21-#30 |
| M4: Federation | v0.4.0 | Feb 15, 2025 | #31-#40 |
| GA Release | v1.0.0 | Feb 28, 2025 | - |
| M5: Plugins | v1.1.0 | Mar 31, 2025 | #41-#50 |
| M6: Multi-Tenant | v1.2.0 | Apr 30, 2025 | #51-#60 |

---

## Release Phases

### Phase 1: Core Backend (v0.1.0) ✅ CURRENT
**Target: December 2024**

| Feature | Status | Issue | Description |
|---------|--------|-------|-------------|
| Health Endpoint | ✅ Done | - | `/health` with version |
| Client Registration | ✅ Done | - | Two-step key exchange |
| Session Management | ✅ Done | - | Create, validate, expire |
| Admin Auth | ✅ Done | - | Generated admin key |
| Server Keypairs | ✅ Done | - | Per-client X25519 keys |
| Unit Tests | ✅ Done | - | 30 tests passing |
| CI Pipeline | ✅ Done | - | GitHub Actions |

**Deliverables:**
- Working Rust backend with auth
- YAML-based storage
- Pre-commit hooks

---

### Phase 2: Frontend Refactor (v0.2.0)
**Target: January 2025**

| Feature | Status | Issue | Description |
|---------|--------|-------|-------------|
| Component Extraction | 🔲 Pending | #11 | Split 817-line page.tsx |
| Custom Hooks | 🔲 Pending | #12 | useSettings, useAuth, etc. |
| API Client Layer | 🔲 Pending | #13 | Centralized API calls |
| Type Definitions | 🔲 Pending | #14 | Shared TypeScript types |
| Unit Tests | 🔲 Pending | #15 | Vitest for components |

**File Structure After Refactor:**
```
frontend/src/
├── app/
│   └── page.tsx              # ~100 lines (layout only)
├── components/
│   ├── tabs/
│   │   ├── HomeTab.tsx       # ~150 lines
│   │   ├── RegisterTab.tsx   # ~100 lines
│   │   ├── KeysTab.tsx       # ~100 lines
│   │   ├── ServerTab.tsx     # ~100 lines
│   │   └── SettingsTab.tsx   # ~150 lines
│   └── common/
│       ├── QRDisplay.tsx     # ~50 lines
│       └── TabButton.tsx     # ~30 lines
├── hooks/
│   ├── useServerInfo.ts
│   ├── useSettings.ts
│   ├── useAuth.ts
│   └── useKnownServers.ts
├── lib/
│   └── api.ts                # API client
└── types/
    └── index.ts              # Shared types
```

---

### Phase 3: Config UI (v0.3.0) ✅ DONE
**Target: January 2025**

| Feature | Status | Issue | Description |
|---------|--------|-------|-------------|
| ConfigManager Service | ✅ Done | - | Thread-safe config access |
| Settings API | ✅ Done | - | GET/PUT endpoints |
| Settings Tab UI | ✅ Done | - | Admin-only config editor |
| Server Identity | ✅ Done | - | Name, description |
| Network Settings | ✅ Done | - | Host, port, public URL |
| Auth Settings | ✅ Done | - | Session TTL |
| Federation Settings | ✅ Done | - | Enable/disable, sync interval |

---

### Phase 4: Federation (v0.4.0)
**Target: February 2025**

| Feature | Status | Issue | Description |
|---------|--------|-------|-------------|
| Server Registry | ✅ Done | - | Store known servers |
| Server Discovery | ✅ Done | - | `/servers/public` endpoint |
| Sync Service | ✅ Done | - | Hourly background sync |
| Server Dashboard | ✅ Done | - | UI for known servers |
| Authenticated Sync | 🔲 Pending | #31 | Verify server identity |
| Mutual TLS | 🔲 Pending | #32 | Server-to-server auth |

---

### Phase 5: Plugins (v1.1.0)
**Target: March 2025**

| Feature | Status | Issue | Description |
|---------|--------|-------|-------------|
| Plugin Trait | 🔲 Pending | #41 | Plugin interface |
| Plugin Loader | 🔲 Pending | #42 | Dynamic loading |
| Metrics Plugin | 🔲 Pending | #43 | Prometheus metrics |
| OpenAPI Plugin | 🔲 Pending | #44 | Swagger UI |
| Webhook Plugin | 🔲 Pending | #45 | Event notifications |

---

### Phase 6: Multi-Tenant (v1.2.0)
**Target: April 2025**

| Feature | Status | Issue | Description |
|---------|--------|-------|-------------|
| Tenant Isolation | 🔲 Pending | #51 | Separate data per tenant |
| Tenant Admin | 🔲 Pending | #52 | Per-tenant admin keys |
| Tenant Quotas | 🔲 Pending | #53 | Rate limits per tenant |
| Database Migration | 🔲 Pending | #54 | SQLite → PostgreSQL |

---

## GitHub Issues to Create

### Phase 2 Issues (Frontend Refactor)

```javascript
const phase2Issues = [
  {
    title: "[Frontend] Extract tab components from page.tsx",
    body: `## Description
Split the 817-line page.tsx into focused tab components.

## Files to Create
- \`components/tabs/HomeTab.tsx\`
- \`components/tabs/RegisterTab.tsx\`
- \`components/tabs/KeysTab.tsx\`
- \`components/tabs/ServerTab.tsx\`
- \`components/tabs/SettingsTab.tsx\`

## Acceptance Criteria
- [ ] Each tab is a separate component
- [ ] page.tsx is under 150 lines
- [ ] No functionality changes
- [ ] All tabs still work

## Labels
enhancement, frontend, phase-2`,
    labels: ["enhancement", "frontend", "phase-2"]
  },
  {
    title: "[Frontend] Create custom hooks for data fetching",
    body: `## Description
Extract data fetching logic into reusable hooks.

## Hooks to Create
- \`useServerInfo()\` - Server public key and info
- \`useSettings()\` - Config get/save
- \`useAuth()\` - Admin login state
- \`useKnownServers()\` - Federation servers

## Acceptance Criteria
- [ ] Each hook handles loading/error states
- [ ] Hooks are typed with TypeScript
- [ ] Components use hooks instead of inline fetch

## Labels
enhancement, frontend, phase-2`,
    labels: ["enhancement", "frontend", "phase-2"]
  },
  {
    title: "[Frontend] Create centralized API client",
    body: `## Description
Create a typed API client for all backend calls.

## File
\`lib/api.ts\`

## Methods
- \`getServerInfo()\`
- \`getSettings()\` / \`updateSettings()\`
- \`registerInit()\` / \`registerComplete()\`
- \`adminLogin()\`
- \`getKnownServers()\`

## Acceptance Criteria
- [ ] All API calls go through client
- [ ] Proper error handling
- [ ] TypeScript types for all responses

## Labels
enhancement, frontend, phase-2`,
    labels: ["enhancement", "frontend", "phase-2"]
  }
];
```

---

## CI/CD Workflows

### Current Workflows

| Workflow | Trigger | Actions |
|----------|---------|---------|
| `backend.yml` | Push to backend/ | fmt, clippy, test, build |
| `frontend.yml` | Push to frontend/ | lint, type-check, build |

### Workflows to Add

| Workflow | Trigger | Actions |
|----------|---------|---------|
| `docker.yml` | Push to main, tags | Build + push to GHCR |
| `release.yml` | Push tag v* | Create GitHub release |
| `integration.yml` | PR to main | Docker-based integration tests |

---

## Release Checklist

### Before Tagging a Release

- [ ] All tests pass (`make test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Version bumped in package.json

### Release Process

```bash
# 1. Update version
vim Cargo.toml  # Update version
vim frontend/package.json  # Update version

# 2. Update changelog
vim CHANGELOG.md

# 3. Commit
git add -A
git commit -m "chore: bump version to v0.2.0"

# 4. Tag
git tag -a v0.2.0 -m "Release v0.2.0 - Frontend Refactor"

# 5. Push
git push origin main --tags
```

### Post-Release

- [ ] Verify GitHub release created
- [ ] Verify Docker image published
- [ ] Update project board
- [ ] Close milestone
- [ ] Announce release

---

## Metrics & Success Criteria

### v1.0.0 GA Criteria

Before tagging v1.0.0, all of the following must be true:

- [ ] Backend builds and passes all tests
- [ ] Frontend builds and passes all tests
- [ ] Client registration works end-to-end
- [ ] Admin login works
- [ ] Settings can be viewed and saved
- [ ] Server federation discovers peers
- [ ] Docker Compose deploys full stack
- [ ] API documentation is complete
- [ ] No critical or high-severity bugs open
- [ ] 80%+ code coverage (backend)

### Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| API response (p95) | < 50ms | Tracing |
| Registration flow | < 500ms | E2E test |
| Settings save | < 100ms | Tracing |
| Memory usage | < 128MB | Docker stats |
| Binary size | < 20MB | CI artifact |
| Frontend bundle | < 200KB | Lighthouse |

---

## Getting Started

```bash
# Development
make run-backend   # Start backend on :8080
make run-frontend  # Start frontend on :3000

# Testing
make test          # Run all tests
make lint          # Run linters

# Docker
docker-compose up  # Full stack

# Release
git tag -a v0.2.0 -m "Release v0.2.0"
git push --tags
```
