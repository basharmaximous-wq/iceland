#!/bin/bash
set -e

echo "Installing Rust targets..."
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

echo "Building x86_64 Linux binary..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "Building aarch64 Linux binary..."
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
cargo build --release --target aarch64-unknown-linux-gnu

echo "Done!"
echo "Binaries are located at:"
echo "  - target/x86_64-unknown-linux-gnu/release/iceland"
echo "  - target/aarch64-unknown-linux-gnu/release/iceland"
