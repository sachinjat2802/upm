#!/usr/bin/env bash
# UPM Automated 1-Command Repository Migration Script (Linux/macOS)
# Usage: curl -fsSL https://raw.githubusercontent.com/sachinjat2802/upm/master/scripts/migrate.sh | bash

set -e

echo "╭──────────────────────────────────────────────────────╮"
echo "│  ⚡ UPM Automated 1-Command Repository Migration     │"
echo "╰──────────────────────────────────────────────────────╯"
echo ""

if ! command -v upm &> /dev/null; then
    echo "  ▶ Installing UPM runtime..."
    curl -fsSL https://raw.githubusercontent.com/sachinjat2802/upm/master/install.sh | bash
fi

upm migrate .
