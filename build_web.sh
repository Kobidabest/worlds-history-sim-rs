#!/bin/bash
# Build the evolution simulator WASM module and output to web/pkg/
set -e

echo "Building WASM module..."
cd simulation
wasm-pack build --target web --out-dir ../web/pkg
echo ""
echo "Build complete! To run the web app:"
echo "  cd web && python3 -m http.server 8080"
echo "  Then open http://localhost:8080"
