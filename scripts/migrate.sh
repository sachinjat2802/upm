#!/usr/bin/env bash
# CPM Automated 1-Command Repository Migration Script (Linux/macOS)
# Usage: curl -fsSL https://raw.githubusercontent.com/sachinjat2802/cpm/master/scripts/migrate.sh | bash

set -e

echo "╭──────────────────────────────────────────────────────╮"
echo "│  ⚡ CPM Automated 1-Command Repository Migration     │"
echo "╰──────────────────────────────────────────────────────╯"
echo ""

if ! command -v cpm &> /dev/null; then
    echo "  ▶ Installing CPM runtime..."
    curl -fsSL https://raw.githubusercontent.com/sachinjat2802/cpm/master/install.sh | bash
fi

cpm migrate .
