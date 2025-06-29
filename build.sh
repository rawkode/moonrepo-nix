#!/usr/bin/env bash

set -e

echo "Building moon Nix toolchain plugin..."

# Add wasm32-wasi target if not already installed
rustup target add wasm32-wasip1

# Build the plugin
cargo build --release --target wasm32-wasip1

# Copy to root for easy access
cp target/wasm32-wasip1/release/moon_toolchain_nix.wasm .

echo "✅ Build complete! Plugin available at: moon_toolchain_nix.wasm"
echo ""
echo "To use this plugin, add to your .moon/toolchain.yml:"
echo ""
echo "nix:"
echo "  plugin: \"file://$(pwd)/moon_toolchain_nix.wasm\""
