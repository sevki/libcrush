#!/bin/bash
set -e

echo "Building mycrush for WASM..."
cd "$(dirname "$0")"

# Build for web (ES modules)
echo "Building for web (ES modules)..."
wasm-pack build --target web --out-dir pkg

# Build for Node.js
echo "Building for Node.js..."
wasm-pack build --target nodejs --out-dir pkg-node

# Build for bundlers (webpack, rollup, etc.)
echo "Building for bundlers..."
wasm-pack build --target bundler --out-dir pkg-bundler

echo "✨ All builds complete!"
echo ""
echo "Build outputs:"
echo "  - pkg/          - For web (ES modules)"
echo "  - pkg-node/     - For Node.js"
echo "  - pkg-bundler/  - For bundlers (webpack, rollup, etc.)"
echo ""
echo "To publish to npm, run:"
echo "  cd pkg && npm publish"
