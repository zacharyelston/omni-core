#!/bin/bash
# Setup git hooks for omni-core

set -e

ROOT_DIR="$(git rev-parse --show-toplevel)"

echo "🔧 Setting up git hooks..."

# Configure git to use our hooks directory
git config core.hooksPath .githooks

# Make hooks executable
chmod +x "$ROOT_DIR/.githooks/"*

echo "✅ Git hooks configured!"
echo ""
echo "Hooks will run automatically on commit."
echo "To skip hooks temporarily: git commit --no-verify"
echo ""
echo "Available hooks:"
echo "  - pre-commit: Runs fmt, clippy, and tests on staged files"
