#!/bin/bash

set -e

echo "Building sinfo..."
cargo build --release

echo "Installing sinfo to /usr/local/bin (requires sudo)..."
sudo cp target/release/sinfo /usr/local/bin/sinfo

echo "sinfo installed! You can now run 'sinfo' from anywhere."