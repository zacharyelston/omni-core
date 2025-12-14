.PHONY: all test lint build clean setup-hooks backend frontend

# Default target
all: lint test build

# Setup git hooks
setup-hooks:
	@./scripts/setup-hooks.sh

# Run all tests
test:
	@./scripts/test.sh

# Run all linters
lint:
	@./scripts/lint.sh

# Build everything
build: build-backend build-frontend

build-backend:
	@echo "📦 Building backend..."
	@cd backend && cargo build --release

build-frontend:
	@echo "🌐 Building frontend..."
	@cd frontend && npm run build

# Run backend
run-backend:
	@cd backend && cargo run

# Run frontend dev server
run-frontend:
	@cd frontend && npm run dev

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	@cd backend && cargo clean
	@rm -rf frontend/.next frontend/node_modules/.cache

# Install dependencies
install:
	@echo "📥 Installing dependencies..."
	@cd frontend && npm install

# Format code
fmt:
	@echo "✨ Formatting code..."
	@cd backend && cargo fmt --all

# Quick check (no tests)
check:
	@echo "🔍 Quick check..."
	@cd backend && cargo check
	@cd frontend && npx tsc --noEmit 2>/dev/null || echo "⚠️  Run 'make install' first"

# Help
help:
	@echo "Omni Core Makefile"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Run lint, test, and build"
	@echo "  setup-hooks  - Configure git pre-commit hooks"
	@echo "  test         - Run all tests"
	@echo "  lint         - Run all linters"
	@echo "  build        - Build backend and frontend"
	@echo "  run-backend  - Start the backend server"
	@echo "  run-frontend - Start the frontend dev server"
	@echo "  install      - Install frontend dependencies"
	@echo "  fmt          - Format all code"
	@echo "  check        - Quick syntax check (no tests)"
	@echo "  clean        - Remove build artifacts"
	@echo "  help         - Show this help"
