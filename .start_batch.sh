#!/bin/bash
# Check if running as root - if so, exit with error
if [ "$EUID" -eq 0 ]; then
    echo "Error: Don't run this script directly with sudo"
    echo "Usage: ./.start.sh"
    echo "The script will handle sudo internally"
    exit 1
fi

echo "Building vpn-batch as user..."
RUST_LOG=debug RUST_BACKTRACE=full cargo build -p vpn-batch
if [ $? -ne 0 ]; then
    echo "Build failed, exiting..."
    exit 1
fi

echo "Loading environment variables from .env..."
# Source .env file to load variables into current shell
if [ -f .env ]; then
    set -a  # automatically export all variables
    source .env
    set +a  # disable automatic export
    echo "Loaded environment variables from .env"
else
    echo "Warning: .env file not found"
fi

echo "Running vpn-batch with sudo..."
# Pass all necessary environment variables explicitly
sudo DATABASE_URL="$DATABASE_URL" \
     WG_CONF_DIR="$WG_CONF_DIR" \
     AWS_QUEUE_URL="$AWS_QUEUE_URL" \
     RUST_LOG=debug \
     RUST_BACKTRACE=full \
     PATH="$PATH" \
     HOME="$HOME" \
     ./target/debug/vpn-batch
