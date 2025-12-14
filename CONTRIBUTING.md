# Contributing to Omni Core

Thank you for your interest in contributing to Omni Core! This document provides guidelines and standards for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)

---

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow

---

## Getting Started

### Prerequisites

- Rust 1.70+ (`rustup update stable`)
- Node.js 18+ (`nvm use 18`)
- Docker (for integration tests)

### Setup

```bash
# Clone the repository
git clone https://github.com/zacharyelston/omni-core.git
cd omni-core

# Install git hooks
make setup-hooks

# Install frontend dependencies
cd frontend && npm install && cd ..

# Verify setup
make test
```

---

## Development Workflow

### Branch Strategy

```
main
  └── feature/your-feature-name
  └── fix/bug-description
  └── docs/documentation-update
```

### Workflow

1. **Create a branch** from `main`
   ```bash
   git checkout -b feature/add-metrics-endpoint
   ```

2. **Make changes** following code standards

3. **Run tests** before committing
   ```bash
   make test
   ```

4. **Commit** with conventional commit messages
   ```bash
   git commit -m "feat(api): add metrics endpoint"
   ```

5. **Push** and create a PR
   ```bash
   git push -u origin feature/add-metrics-endpoint
   ```

---

## Code Standards

### First Principles

These rules are **non-negotiable**:

| Rule | Limit | Action if Exceeded |
|------|-------|-------------------|
| **File size** | 500 lines | Split into modules |
| **Function size** | 50 lines | Extract helpers |
| **Nesting depth** | 3 levels | Refactor |
| **Parameters** | 5 params | Use struct |

### Rust Standards

```rust
// ✅ Good: Clear, documented, tested
/// Creates a new session for the given client.
///
/// # Arguments
/// * `client_id` - Unique identifier for the client
///
/// # Returns
/// A new `Session` with a generated API key
pub fn create_session(client_id: &str) -> Session {
    Session {
        client_id: client_id.to_string(),
        api_key: generate_api_key(),
        created_at: Utc::now(),
    }
}

// ❌ Bad: No docs, unclear purpose
pub fn cs(c: &str) -> Session {
    Session { client_id: c.to_string(), api_key: gen(), created_at: Utc::now() }
}
```

**Rust Checklist:**
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` has no warnings
- [ ] All public items have doc comments
- [ ] Error types use `thiserror`
- [ ] Logging uses `tracing`

### TypeScript Standards

```typescript
// ✅ Good: Typed, clear, reusable
export type ServerConfig = {
  server: ServerSettings;
  network: NetworkSettings;
  auth: AuthSettings;
  federation: FederationSettings;
};

export async function fetchSettings(): Promise<ServerConfig> {
  const response = await fetch('/api/v1/settings');
  if (!response.ok) {
    throw new Error(`Failed to fetch settings: ${response.status}`);
  }
  return response.json();
}

// ❌ Bad: Untyped, inline, no error handling
const getSettings = () => fetch('/api/v1/settings').then(r => r.json());
```

**TypeScript Checklist:**
- [ ] `npm run lint` passes
- [ ] `npm run type-check` passes
- [ ] All functions have explicit return types
- [ ] No `any` types (use `unknown` if needed)
- [ ] Components are under 200 lines

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Formatting (no code change)
- `refactor` - Code restructuring
- `test` - Adding tests
- `chore` - Maintenance

**Examples:**
```bash
# Feature
git commit -m "feat(federation): add server discovery endpoint"

# Bug fix
git commit -m "fix(session): handle expired session cleanup"

# Documentation
git commit -m "docs(api): add settings endpoint documentation"

# With body and footer
git commit -m "feat(settings): add federation toggle

Allows admins to enable/disable server federation
from the Settings UI.

Closes #42"
```

---

## Testing Requirements

### Test Coverage

| Component | Minimum Coverage |
|-----------|-----------------|
| Backend services | 80% |
| API handlers | 70% |
| Frontend hooks | 70% |
| Frontend components | 50% |

### Running Tests

```bash
# All tests
make test

# Backend only
cd backend && cargo test

# Frontend only
cd frontend && npm test

# With coverage
cd backend && cargo tarpaulin
cd frontend && npm run test:coverage
```

### Writing Tests

**Rust:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let store = SessionStore::new();
        let session = store.create("client-001");
        
        assert!(!session.api_key.is_empty());
        assert_eq!(session.client_id, "client-001");
    }

    #[test]
    fn test_session_expiry() {
        let store = SessionStore::new();
        let session = store.create("client-001");
        
        // Fast-forward time
        store.cleanup_expired();
        
        assert!(store.get(&session.api_key).is_none());
    }
}
```

**TypeScript:**
```typescript
import { describe, it, expect, vi } from 'vitest';
import { useSettings } from './useSettings';
import { renderHook, waitFor } from '@testing-library/react';

describe('useSettings', () => {
  it('should fetch settings on mount', async () => {
    const { result } = renderHook(() => useSettings());
    
    await waitFor(() => {
      expect(result.current.config).not.toBeNull();
    });
  });

  it('should handle fetch errors', async () => {
    vi.spyOn(global, 'fetch').mockRejectedValueOnce(new Error('Network error'));
    
    const { result } = renderHook(() => useSettings());
    
    await waitFor(() => {
      expect(result.current.error).toBe('Network error');
    });
  });
});
```

---

## Pull Request Process

### Before Opening a PR

- [ ] Branch is up to date with `main`
- [ ] All tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation is updated
- [ ] Commit messages follow convention

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings

## Related Issues
Closes #XX
```

### Review Process

1. **Automated checks** must pass (CI)
2. **Code review** by at least one maintainer
3. **Approval** required before merge
4. **Squash merge** to keep history clean

---

## Issue Guidelines

### Bug Reports

```markdown
## Bug Description
Clear description of the bug

## Steps to Reproduce
1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., macOS 14.0]
- Browser: [e.g., Chrome 120]
- Version: [e.g., v0.1.0]

## Screenshots
If applicable
```

### Feature Requests

```markdown
## Feature Description
Clear description of the feature

## Use Case
Why is this feature needed?

## Proposed Solution
How should it work?

## Alternatives Considered
Other approaches considered

## Additional Context
Any other information
```

### Issue Labels

| Label | Description |
|-------|-------------|
| `bug` | Something isn't working |
| `enhancement` | New feature request |
| `documentation` | Documentation updates |
| `good first issue` | Good for newcomers |
| `help wanted` | Extra attention needed |
| `priority-high` | Critical path |
| `backend` | Rust backend |
| `frontend` | TypeScript frontend |

---

## Project Structure

```
omni-core/
├── backend/src/
│   ├── api/           # HTTP handlers (one file per resource)
│   ├── services/      # Business logic (one file per service)
│   ├── config.rs      # Configuration
│   └── main.rs        # Entry point
├── frontend/src/
│   ├── app/           # Next.js pages
│   ├── components/    # React components
│   ├── hooks/         # Custom hooks
│   ├── lib/           # Utilities
│   └── types/         # TypeScript types
├── docs/              # Documentation
├── scripts/           # Development scripts
└── data/              # Runtime data (gitignored)
```

---

## Getting Help

- **Documentation**: Check `docs/` folder
- **Issues**: Search existing issues first
- **Discussions**: Use GitHub Discussions for questions

---

## Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Project README

Thank you for contributing! 🎉
