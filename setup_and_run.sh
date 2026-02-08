#!/bin/bash
set -e

# Build the project
cargo build --release

# Start/Restart the application
pm2 startOrReload ecosystem.config.js
pm2 save
