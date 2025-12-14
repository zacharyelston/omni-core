#!/usr/bin/env node

/**
 * Create GitHub Issues for Omni Core Release Schedule
 * 
 * Usage:
 *   GITHUB_TOKEN=your_token node scripts/create-github-issues.js
 * 
 * Or with GitHub CLI:
 *   gh auth login
 *   node scripts/create-github-issues.js
 */

const { execSync } = require('child_process');

const OWNER = 'zacharyelston';
const REPO = 'omni-core';

// Phase 2: Frontend Refactor
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

## Phase
Phase 2: Frontend Refactor (v0.2.0)`,
    labels: ["enhancement", "frontend", "phase-2"],
    milestone: "M2: Frontend Refactor"
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
- \`useClientKeys()\` - Local storage keys

## Acceptance Criteria
- [ ] Each hook handles loading/error states
- [ ] Hooks are typed with TypeScript
- [ ] Components use hooks instead of inline fetch
- [ ] Unit tests for each hook

## Phase
Phase 2: Frontend Refactor (v0.2.0)`,
    labels: ["enhancement", "frontend", "phase-2"],
    milestone: "M2: Frontend Refactor"
  },
  {
    title: "[Frontend] Create centralized API client",
    body: `## Description
Create a typed API client for all backend calls.

## File
\`lib/api.ts\`

## Methods
\`\`\`typescript
class OmniAPI {
  getServerInfo(): Promise<ServerInfo>
  getSettings(): Promise<ServerConfig>
  updateSettings(config: ServerConfig): Promise<void>
  registerInit(clientId: string): Promise<RegisterInitResponse>
  registerComplete(clientId: string, publicKey: string): Promise<RegisterCompleteResponse>
  adminLogin(key: string): Promise<void>
  getKnownServers(): Promise<KnownServer[]>
}
\`\`\`

## Acceptance Criteria
- [ ] All API calls go through client
- [ ] Proper error handling with typed errors
- [ ] TypeScript types for all responses
- [ ] Base URL configurable

## Phase
Phase 2: Frontend Refactor (v0.2.0)`,
    labels: ["enhancement", "frontend", "phase-2"],
    milestone: "M2: Frontend Refactor"
  },
  {
    title: "[Frontend] Create shared TypeScript types",
    body: `## Description
Extract and centralize TypeScript type definitions.

## File
\`types/index.ts\`

## Types to Define
\`\`\`typescript
// Server types
export type ServerInfo = { ... }
export type ServerConfig = { ... }
export type KnownServer = { ... }

// Client types
export type ClientKey = { ... }
export type ServerKey = { ... }
export type Session = { ... }

// API types
export type ApiResponse<T> = { ... }
export type ApiError = { ... }
\`\`\`

## Acceptance Criteria
- [ ] All types in one location
- [ ] Types match backend API
- [ ] No duplicate type definitions
- [ ] Exported via barrel file

## Phase
Phase 2: Frontend Refactor (v0.2.0)`,
    labels: ["enhancement", "frontend", "phase-2"],
    milestone: "M2: Frontend Refactor"
  },
  {
    title: "[Frontend] Add Vitest unit tests",
    body: `## Description
Set up Vitest and add unit tests for hooks and components.

## Setup
- Install vitest, @testing-library/react
- Configure vitest.config.ts
- Add test scripts to package.json

## Tests to Write
- [ ] useServerInfo hook
- [ ] useSettings hook
- [ ] useAuth hook
- [ ] API client methods
- [ ] Tab components render

## Acceptance Criteria
- [ ] \`npm test\` runs all tests
- [ ] 70%+ coverage on hooks
- [ ] 50%+ coverage on components
- [ ] Tests run in CI

## Phase
Phase 2: Frontend Refactor (v0.2.0)`,
    labels: ["enhancement", "frontend", "phase-2", "testing"],
    milestone: "M2: Frontend Refactor"
  }
];

// Phase 4: Federation
const phase4Issues = [
  {
    title: "[Backend] Add authenticated server-to-server sync",
    body: `## Description
Verify server identity during federation sync using public keys.

## Current State
- Servers can register and sync
- No verification of server identity

## Required Changes
1. Sign sync requests with server private key
2. Verify signatures on incoming sync requests
3. Reject unverified servers

## Acceptance Criteria
- [ ] Sync requests are signed
- [ ] Signatures are verified
- [ ] Invalid signatures rejected
- [ ] Unit tests for signing/verification

## Phase
Phase 4: Federation (v0.4.0)`,
    labels: ["enhancement", "backend", "phase-4", "security"],
    milestone: "M4: Federation"
  },
  {
    title: "[Backend] Add mutual TLS for server federation",
    body: `## Description
Implement mutual TLS for server-to-server communication.

## Requirements
- Generate server certificates
- Configure TLS for sync endpoints
- Verify client certificates

## Acceptance Criteria
- [ ] Server generates self-signed cert on startup
- [ ] Sync uses mTLS when available
- [ ] Fallback to unsigned sync for public servers
- [ ] Certificate rotation supported

## Phase
Phase 4: Federation (v0.4.0)`,
    labels: ["enhancement", "backend", "phase-4", "security"],
    milestone: "M4: Federation"
  }
];

// Phase 5: Plugins
const phase5Issues = [
  {
    title: "[Backend] Design plugin trait and loader",
    body: `## Description
Create a plugin system for extending Omni Core functionality.

## Plugin Trait
\`\`\`rust
#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    async fn on_startup(&self, state: &AppState) -> Result<()>;
    async fn on_shutdown(&self, state: &AppState) -> Result<()>;
    
    fn routes(&self) -> Option<Router<AppState>> { None }
}
\`\`\`

## Acceptance Criteria
- [ ] Plugin trait defined
- [ ] Plugin loader implemented
- [ ] Example plugin created
- [ ] Documentation written

## Phase
Phase 5: Plugins (v1.1.0)`,
    labels: ["enhancement", "backend", "phase-5"],
    milestone: "M5: Plugins"
  },
  {
    title: "[Backend] Add Prometheus metrics plugin",
    body: `## Description
Create a metrics plugin that exposes Prometheus-compatible metrics.

## Metrics to Expose
- \`omni_http_requests_total\`
- \`omni_http_request_duration_seconds\`
- \`omni_active_sessions\`
- \`omni_registered_clients\`
- \`omni_known_servers\`

## Endpoint
\`GET /metrics\`

## Acceptance Criteria
- [ ] Metrics endpoint works
- [ ] All metrics exposed
- [ ] Prometheus can scrape
- [ ] Grafana dashboard template

## Phase
Phase 5: Plugins (v1.1.0)`,
    labels: ["enhancement", "backend", "phase-5"],
    milestone: "M5: Plugins"
  },
  {
    title: "[Backend] Add OpenAPI/Swagger plugin",
    body: `## Description
Auto-generate OpenAPI documentation from code.

## Implementation
Use \`utoipa\` crate for Rust OpenAPI generation.

## Endpoints
- \`GET /api-docs/openapi.json\` - OpenAPI spec
- \`GET /swagger-ui\` - Swagger UI

## Acceptance Criteria
- [ ] OpenAPI spec generated
- [ ] Swagger UI accessible
- [ ] All endpoints documented
- [ ] Types documented

## Phase
Phase 5: Plugins (v1.1.0)`,
    labels: ["enhancement", "backend", "phase-5", "documentation"],
    milestone: "M5: Plugins"
  }
];

// CI/CD Issues
const cicdIssues = [
  {
    title: "[CI] Add Docker image publishing workflow",
    body: `## Description
Automatically build and publish Docker images to GHCR.

## Workflow
\`\`\`yaml
name: Docker

on:
  push:
    branches: [main]
    tags: ['v*']

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: docker/build-push-action@v5
        with:
          push: true
          tags: ghcr.io/zacharyelston/omni-core:latest
\`\`\`

## Acceptance Criteria
- [ ] Images built on push to main
- [ ] Images tagged with version on release
- [ ] Both backend and frontend images
- [ ] Multi-arch support (amd64, arm64)

## Priority
High - Enables easy deployment`,
    labels: ["enhancement", "ci-cd", "priority-high"],
    milestone: "M1: Core Backend"
  },
  {
    title: "[CI] Add release automation workflow",
    body: `## Description
Automatically create GitHub releases when tags are pushed.

## Workflow
- Trigger on \`v*\` tags
- Build release binaries
- Create GitHub release
- Upload artifacts
- Generate changelog

## Artifacts
- \`omni-server-linux-amd64\`
- \`omni-server-darwin-amd64\`
- \`omni-server-darwin-arm64\`

## Acceptance Criteria
- [ ] Release created on tag push
- [ ] Binaries attached
- [ ] Changelog generated
- [ ] Docker images tagged

## Priority
High - Enables versioned releases`,
    labels: ["enhancement", "ci-cd", "priority-high"],
    milestone: "M1: Core Backend"
  },
  {
    title: "[CI] Add integration test workflow",
    body: `## Description
Run integration tests using Docker Compose in CI.

## Workflow
- Start two Omni Core instances
- Test registration flow
- Test federation sync
- Test settings API

## Acceptance Criteria
- [ ] Integration tests run in CI
- [ ] Tests use Docker Compose
- [ ] Federation tested between instances
- [ ] Results reported

## Priority
Medium - Improves confidence`,
    labels: ["enhancement", "ci-cd", "testing"],
    milestone: "M2: Frontend Refactor"
  },
  {
    title: "[CI] Add Dependabot configuration",
    body: `## Description
Configure Dependabot for automated dependency updates.

## File
\`.github/dependabot.yml\`

## Configuration
\`\`\`yaml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
  - package-ecosystem: "npm"
    directory: "/frontend"
    schedule:
      interval: "weekly"
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
\`\`\`

## Acceptance Criteria
- [ ] Dependabot configured
- [ ] PRs created for updates
- [ ] Security updates prioritized

## Priority
Low - Nice to have`,
    labels: ["enhancement", "ci-cd"],
    milestone: "M1: Core Backend"
  }
];

// All issues
const allIssues = [
  ...phase2Issues,
  ...phase4Issues,
  ...phase5Issues,
  ...cicdIssues
];

// Create issues using GitHub CLI
function createIssue(issue) {
  const labels = issue.labels.join(',');
  const cmd = `gh issue create --repo ${OWNER}/${REPO} --title "${issue.title}" --body "${issue.body.replace(/"/g, '\\"').replace(/\n/g, '\\n')}" --label "${labels}"`;
  
  try {
    const result = execSync(cmd, { encoding: 'utf-8' });
    console.log(`✅ Created: ${issue.title}`);
    console.log(`   ${result.trim()}`);
    return true;
  } catch (error) {
    console.error(`❌ Failed: ${issue.title}`);
    console.error(`   ${error.message}`);
    return false;
  }
}

// Main
async function main() {
  console.log('🚀 Creating GitHub Issues for Omni Core\n');
  console.log(`Repository: ${OWNER}/${REPO}`);
  console.log(`Total issues: ${allIssues.length}\n`);

  // Check if gh CLI is available
  try {
    execSync('gh --version', { encoding: 'utf-8' });
  } catch {
    console.error('❌ GitHub CLI (gh) not found. Install it from https://cli.github.com/');
    console.error('   Or set GITHUB_TOKEN environment variable.');
    process.exit(1);
  }

  // Check authentication
  try {
    execSync('gh auth status', { encoding: 'utf-8' });
  } catch {
    console.error('❌ Not authenticated. Run: gh auth login');
    process.exit(1);
  }

  let created = 0;
  let failed = 0;

  for (const issue of allIssues) {
    if (createIssue(issue)) {
      created++;
    } else {
      failed++;
    }
    // Rate limit
    await new Promise(resolve => setTimeout(resolve, 1000));
  }

  console.log(`\n📊 Summary: ${created} created, ${failed} failed`);
}

// Run if called directly
if (require.main === module) {
  main().catch(console.error);
}

module.exports = { allIssues, createIssue };
