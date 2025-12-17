#!/bin/bash
# Run docker-compose tests for omni-core

set -e

echo "🐳 Omni Core Docker Test Suite"
echo "=============================="

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

cd "$(dirname "$0")/.."

# Clean up any existing containers
echo ""
echo "🧹 Cleaning up existing containers..."
docker-compose -f docker-compose.test.yml down -v 2>/dev/null || true

# Build images
echo ""
echo "🔨 Building Docker images..."
docker-compose -f docker-compose.test.yml build

# Start services
echo ""
echo "🚀 Starting test services..."
docker-compose -f docker-compose.test.yml up -d server1 server2 server3

# Wait for services to be healthy
echo ""
echo "⏳ Waiting for services to start..."
sleep 15

# Run health checks
echo ""
echo "🏥 Running health checks..."

check_health() {
    local name=$1
    local port=$2
    local response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:$port/api/v1/health 2>/dev/null || echo "000")
    if [ "$response" = "200" ]; then
        echo "  ✅ $name (port $port) - healthy"
        return 0
    else
        echo "  ❌ $name (port $port) - unhealthy (HTTP $response)"
        return 1
    fi
}

check_health "Server 1" 9081
check_health "Server 2" 9082
check_health "Server 3" 9083

# Run API tests
echo ""
echo "🧪 Running API tests..."

# Test server info
echo ""
echo "📋 Server Info:"
echo "  Server 1:"
curl -s http://localhost:9081/api/v1/server/info | python3 -m json.tool 2>/dev/null || curl -s http://localhost:9081/api/v1/server/info
echo ""
echo "  Server 2:"
curl -s http://localhost:9082/api/v1/server/info | python3 -m json.tool 2>/dev/null || curl -s http://localhost:9082/api/v1/server/info

# Test registration flow
echo ""
echo "📝 Testing registration flow on Server 1..."
INIT_RESPONSE=$(curl -s -X POST http://localhost:9081/api/v1/register/init \
    -H "Content-Type: application/json" \
    -d '{"client_id": "test-client-001"}')
echo "  Init response: $INIT_RESPONSE"

# Test server federation
echo ""
echo "🌐 Testing server federation..."
SERVER2_INFO=$(curl -s http://localhost:9082/api/v1/server/info)
SERVER2_KEY=$(echo $SERVER2_INFO | python3 -c "import sys, json; print(json.load(sys.stdin)['server_public_key'])" 2>/dev/null || echo "unknown")

echo "  Registering Server 2 with Server 1..."
REGISTER_RESPONSE=$(curl -s -X POST http://localhost:9081/api/v1/servers/register \
    -H "Content-Type: application/json" \
    -d "{\"server_id\": \"server-2\", \"name\": \"Server Beta\", \"public_url\": \"http://server2:8080\", \"public_key\": \"$SERVER2_KEY\", \"is_public\": true}")
echo "  Register response: $REGISTER_RESPONSE"

# Check known servers
echo ""
echo "📡 Known servers on Server 1:"
curl -s http://localhost:9081/api/v1/servers/all | python3 -m json.tool 2>/dev/null || curl -s http://localhost:9081/api/v1/servers/all

# Server stats
echo ""
echo "📊 Server stats:"
echo "  Server 1: $(curl -s http://localhost:9081/api/v1/servers/stats)"
echo "  Server 2: $(curl -s http://localhost:9082/api/v1/servers/stats)"

# Cleanup
echo ""
echo "🧹 Cleaning up..."
docker-compose -f docker-compose.test.yml down -v

echo ""
echo "✅ Docker tests complete!"
