#!/bin/bash
# Run all tests for omni-core

set -e

echo "🧪 Running Omni Core Tests"
echo "=========================="

# Backend tests
echo ""
echo "📦 Backend (Rust)"
echo "-----------------"
cd "$(dirname "$0")/../backend" || exit 1

echo "Running cargo fmt check..."
cargo fmt --all -- --check

echo "Running cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "Running cargo test..."
cargo test --all

echo ""
echo "✅ Backend tests passed!"

# Frontend tests (if npm is available and node_modules exists)
echo ""
echo "🌐 Frontend (Next.js)"
echo "---------------------"
cd "$(dirname "$0")/../frontend" || exit 1

if [ -d "node_modules" ]; then
    echo "Running TypeScript check..."
    npx tsc --noEmit
    echo "✅ Frontend type check passed!"
else
    echo "⚠️  node_modules not found. Run 'npm install' in frontend/ first."
fi

echo ""
echo "🎉 All tests completed!"
