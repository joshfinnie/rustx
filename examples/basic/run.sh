#!/bin/bash

# RustX WASM Example Runner
# This script builds the WASM module and starts the example server

set -e

echo "╔════════════════════════════════════════════╗"
echo "║     RustX WASM Example Setup & Runner     ║"
echo "╚════════════════════════════════════════════╝"
echo ""

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed"
    echo "📦 Installing wasm-pack..."
    cargo install wasm-pack
fi

# Navigate to project root (two levels up from this script)
cd "$(dirname "$0")/../.."
PROJECT_ROOT=$(pwd)

echo "📍 Project root: $PROJECT_ROOT"
echo ""

# Build the WASM module for CDN
echo "🔨 Building WASM module..."
wasm-pack build --target no-modules --out-dir dist --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✅ WASM module built successfully"
echo ""

# Build CDN distribution
echo "📦 Building CDN distribution..."
./build-cdn.sh

echo "✅ CDN distribution ready"
echo ""

# Start the server
echo "🚀 Starting example server..."
echo ""
cd examples/basic
python3 server.py
