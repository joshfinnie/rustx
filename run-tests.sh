#!/bin/bash

# RustX Test Runner
# Runs all tests for the RustX project

set -e

echo "╔════════════════════════════════════════════╗"
echo "║         RustX Test Suite Runner           ║"
echo "╚════════════════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${RED}❌ wasm-pack is not installed${NC}"
    echo "📦 Installing wasm-pack..."
    cargo install wasm-pack
fi

echo "🧪 Running RustX Test Suite..."
echo ""

# Run unit tests in headless browser (Chrome)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Running Unit Tests (Headless Chrome)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if wasm-pack test --headless --chrome; then
    echo -e "${GREEN}✅ Unit tests passed!${NC}"
else
    echo -e "${RED}❌ Unit tests failed${NC}"
    exit 1
fi

echo ""

# Run integration tests
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Running Integration Tests"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if wasm-pack test --headless --chrome --test integration; then
    echo -e "${GREEN}✅ Integration tests passed!${NC}"
else
    echo -e "${RED}❌ Integration tests failed${NC}"
    exit 1
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}🎉 All tests passed!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Optional: Run in Firefox too
if [ "$1" == "--firefox" ]; then
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Running Tests in Firefox"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    if wasm-pack test --headless --firefox; then
        echo -e "${GREEN}✅ Firefox tests passed!${NC}"
    else
        echo -e "${YELLOW}⚠️  Firefox tests failed (optional)${NC}"
    fi
fi

# Show coverage info
echo ""
echo "📊 Test Coverage:"
echo "  - Unit tests: 23 tests"
echo "  - Integration tests: 13 tests"
echo "  - Total: 36 tests"
echo ""
echo "💡 Tip: Run with --firefox to test in Firefox too:"
echo "  ./run-tests.sh --firefox"
echo ""
