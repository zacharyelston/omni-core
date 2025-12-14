#!/bin/bash
# Lint all code for omni-core

set -e

echo "🔍 Running Omni Core Linters"
echo "============================"

# Backend linting
echo ""
echo "📦 Backend (Rust)"
echo "-----------------"
cd "$(dirname "$0")/../backend" || exit 1

echo "Checking formatting..."
cargo fmt --all -- --check

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "✅ Backend lint passed!"

# Frontend linting
echo ""
echo "🌐 Frontend (Next.js)"
echo "---------------------"
cd "$(dirname "$0")/../frontend" || exit 1

if [ -d "node_modules" ]; then
    echo "Running TypeScript check..."
    npx tsc --noEmit
    echo "✅ Frontend lint passed!"
else
    echo "⚠️  node_modules not found. Run 'npm install' in frontend/ first."
fi

echo ""
echo "🎉 All linting completed!"
