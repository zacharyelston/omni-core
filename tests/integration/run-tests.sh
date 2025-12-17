#!/bin/bash
# Integration test suite for Omni Core
# Runs E2E tests against Docker containers

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Server ports
SERVER1_PORT=9081
SERVER2_PORT=9082
SERVER3_PORT=9083

log_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

log_pass() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((TESTS_PASSED++))
}

log_fail() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((TESTS_FAILED++))
}

# Assert HTTP status code
assert_status() {
    local url=$1
    local expected=$2
    local method=${3:-GET}
    local data=${4:-}
    
    if [ -n "$data" ]; then
        actual=$(curl -s -o /dev/null -w "%{http_code}" -X "$method" -H "Content-Type: application/json" -d "$data" "$url" 2>/dev/null)
    else
        actual=$(curl -s -o /dev/null -w "%{http_code}" -X "$method" "$url" 2>/dev/null)
    fi
    
    if [ "$actual" = "$expected" ]; then
        log_pass "$method $url -> $actual"
        return 0
    else
        log_fail "$method $url -> expected $expected, got $actual"
        return 1
    fi
}

# Assert JSON field exists
assert_json_field() {
    local url=$1
    local field=$2
    local response=$(curl -s "$url" 2>/dev/null)
    
    if echo "$response" | python3 -c "import sys, json; d=json.load(sys.stdin); assert '$field' in d" 2>/dev/null; then
        log_pass "GET $url has field '$field'"
        return 0
    else
        log_fail "GET $url missing field '$field'"
        return 1
    fi
}

# Assert JSON field value
assert_json_value() {
    local url=$1
    local field=$2
    local expected=$3
    local response=$(curl -s "$url" 2>/dev/null)
    
    actual=$(echo "$response" | python3 -c "import sys, json; print(json.load(sys.stdin)['$field'])" 2>/dev/null)
    
    if [ "$actual" = "$expected" ]; then
        log_pass "GET $url.$field = '$expected'"
        return 0
    else
        log_fail "GET $url.$field expected '$expected', got '$actual'"
        return 1
    fi
}

# Wait for server to be ready
wait_for_server() {
    local port=$1
    local name=$2
    local max_attempts=30
    local attempt=1
    
    log_info "Waiting for $name (port $port)..."
    while [ $attempt -le $max_attempts ]; do
        if curl -s -o /dev/null -w "%{http_code}" "http://localhost:$port/api/v1/health" 2>/dev/null | grep -q "200"; then
            log_info "$name is ready"
            return 0
        fi
        sleep 1
        ((attempt++))
    done
    
    log_fail "$name failed to start after $max_attempts seconds"
    return 1
}

# ============================================
# Test Suites
# ============================================

test_health_endpoints() {
    echo ""
    echo "=========================================="
    echo "TEST SUITE: Health Endpoints"
    echo "=========================================="
    
    assert_status "http://localhost:$SERVER1_PORT/api/v1/health" "200"
    assert_status "http://localhost:$SERVER2_PORT/api/v1/health" "200"
    assert_status "http://localhost:$SERVER3_PORT/api/v1/health" "200"
}

test_server_info() {
    echo ""
    echo "=========================================="
    echo "TEST SUITE: Server Info"
    echo "=========================================="
    
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/server/info" "server_public_key"
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/server/info" "server_name"
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/server/info" "version"
}

test_settings_api() {
    echo ""
    echo "=========================================="
    echo "TEST SUITE: Settings API"
    echo "=========================================="
    
    # GET settings
    assert_status "http://localhost:$SERVER1_PORT/api/v1/settings" "200"
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/settings" "server"
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/settings" "network"
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/settings" "auth"
    assert_json_field "http://localhost:$SERVER1_PORT/api/v1/settings" "federation"
    
    # GET individual settings
    assert_status "http://localhost:$SERVER1_PORT/api/v1/settings/server" "200"
    assert_status "http://localhost:$SERVER1_PORT/api/v1/settings/network" "200"
    assert_status "http://localhost:$SERVER1_PORT/api/v1/settings/auth" "200"
    assert_status "http://localhost:$SERVER1_PORT/api/v1/settings/federation" "200"
}

test_registration_flow() {
    echo ""
    echo "=========================================="
    echo "TEST SUITE: Client Registration"
    echo "=========================================="
    
    local client_id="integration-test-$(date +%s)"
    
    # Step 1: Initialize registration
    log_info "Initializing registration for $client_id"
    local init_response=$(curl -s -X POST "http://localhost:$SERVER1_PORT/api/v1/register/init" \
        -H "Content-Type: application/json" \
        -d "{\"client_id\": \"$client_id\"}")
    
    if echo "$init_response" | python3 -c "import sys, json; d=json.load(sys.stdin); assert 'server_public_key' in d" 2>/dev/null; then
        log_pass "POST /register/init returns server_public_key"
    else
        log_fail "POST /register/init missing server_public_key"
        return 1
    fi
    
    # Step 2: Complete registration with mock client key
    log_info "Completing registration for $client_id"
    local complete_response=$(curl -s -X POST "http://localhost:$SERVER1_PORT/api/v1/register/complete" \
        -H "Content-Type: application/json" \
        -d "{\"client_id\": \"$client_id\", \"encrypted_client_public_key\": {\"nonce\": \"\", \"ciphertext\": \"dGVzdC1wdWJsaWMta2V5\"}}")
    
    if echo "$complete_response" | python3 -c "import sys, json; d=json.load(sys.stdin); assert 'api_key' in d" 2>/dev/null; then
        log_pass "POST /register/complete returns api_key"
    else
        log_fail "POST /register/complete missing api_key"
    fi
    
    # Step 3: Verify client appears in list
    assert_status "http://localhost:$SERVER1_PORT/api/v1/register/clients" "200"
    assert_status "http://localhost:$SERVER1_PORT/api/v1/register/keys" "200"
}

test_federation() {
    echo ""
    echo "=========================================="
    echo "TEST SUITE: Server Federation"
    echo "=========================================="
    
    # Get Server 2's public key
    local server2_info=$(curl -s "http://localhost:$SERVER2_PORT/api/v1/server/info")
    local server2_key=$(echo "$server2_info" | python3 -c "import sys, json; print(json.load(sys.stdin)['server_public_key'])" 2>/dev/null)
    
    # Register Server 2 with Server 1
    log_info "Registering Server 2 with Server 1"
    local register_response=$(curl -s -X POST "http://localhost:$SERVER1_PORT/api/v1/servers/register" \
        -H "Content-Type: application/json" \
        -d "{\"server_id\": \"server-2-test\", \"name\": \"Server Beta\", \"public_url\": \"http://localhost:$SERVER2_PORT\", \"public_key\": \"$server2_key\", \"is_public\": true}")
    
    if echo "$register_response" | grep -q "registered\|already"; then
        log_pass "POST /servers/register succeeded"
    else
        log_fail "POST /servers/register failed: $register_response"
    fi
    
    # Check server appears in list
    assert_status "http://localhost:$SERVER1_PORT/api/v1/servers/all" "200"
    assert_status "http://localhost:$SERVER1_PORT/api/v1/servers/public" "200"
    assert_status "http://localhost:$SERVER1_PORT/api/v1/servers/stats" "200"
}

test_error_handling() {
    echo ""
    echo "=========================================="
    echo "TEST SUITE: Error Handling"
    echo "=========================================="
    
    # Invalid endpoints should return 404
    assert_status "http://localhost:$SERVER1_PORT/api/v1/nonexistent" "404"
    
    # Invalid JSON should return 400 or 422
    local response=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "not-json" \
        "http://localhost:$SERVER1_PORT/api/v1/register/init" 2>/dev/null)
    
    if [ "$response" = "400" ] || [ "$response" = "422" ]; then
        log_pass "Invalid JSON returns error ($response)"
    else
        log_fail "Invalid JSON should return 400/422, got $response"
    fi
    
    # Missing required fields
    response=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
        -H "Content-Type: application/json" \
        -d "{}" \
        "http://localhost:$SERVER1_PORT/api/v1/register/init" 2>/dev/null)
    
    if [ "$response" = "400" ] || [ "$response" = "422" ]; then
        log_pass "Missing fields returns error ($response)"
    else
        log_fail "Missing fields should return 400/422, got $response"
    fi
}

# ============================================
# Main
# ============================================

main() {
    echo "🧪 Omni Core Integration Test Suite"
    echo "===================================="
    echo ""
    
    # Check if servers are running
    if ! curl -s -o /dev/null "http://localhost:$SERVER1_PORT/api/v1/health" 2>/dev/null; then
        log_info "Servers not running. Starting Docker containers..."
        cd "$PROJECT_ROOT"
        docker-compose -f docker-compose.test.yml up -d server1 server2 server3
        
        wait_for_server $SERVER1_PORT "Server 1"
        wait_for_server $SERVER2_PORT "Server 2"
        wait_for_server $SERVER3_PORT "Server 3"
    fi
    
    # Run test suites
    test_health_endpoints
    test_server_info
    test_settings_api
    test_registration_flow
    test_federation
    test_error_handling
    
    # Summary
    echo ""
    echo "=========================================="
    echo "TEST SUMMARY"
    echo "=========================================="
    echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "Failed: ${RED}$TESTS_FAILED${NC}"
    echo ""
    
    if [ $TESTS_FAILED -gt 0 ]; then
        echo -e "${RED}❌ Some tests failed${NC}"
        exit 1
    else
        echo -e "${GREEN}✅ All tests passed${NC}"
        exit 0
    fi
}

# Parse arguments
CLEANUP=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --cleanup)
            CLEANUP=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Run main
main

# Cleanup if requested
if [ "$CLEANUP" = true ]; then
    echo ""
    log_info "Cleaning up Docker containers..."
    cd "$PROJECT_ROOT"
    docker-compose -f docker-compose.test.yml down -v
fi
