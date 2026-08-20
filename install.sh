#!/usr/bin/env bash
# CPM (Cross-language Package Manager) Linux/macOS Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/sachinjat2802/cpm/master/install.sh | bash

set -e

echo "╭──────────────────────────────────────────────────────╮"
echo "│  📦 Installing CPM (Cross-language Package Manager)  │"
echo "╰──────────────────────────────────────────────────────╯"
echo ""

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

if [ -f "target/release/cpm" ]; then
    cp target/release/cpm "$INSTALL_DIR/cpm"
    cp target/release/upm "$INSTALL_DIR/upm"
    echo "  ✔ Installed local release binaries to $INSTALL_DIR"
else
    echo "  ▶ Building CPM from source via Cargo..."
    cargo build --release
    cp target/release/cpm "$INSTALL_DIR/cpm"
    cp target/release/upm "$INSTALL_DIR/upm"
    echo "  ✔ Installed CPM binaries to $INSTALL_DIR"
fi

chmod +x "$INSTALL_DIR/cpm" "$INSTALL_DIR/upm"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "  ! Add $INSTALL_DIR to your PATH:"
    echo "    export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "  ✨ CPM installation complete!"
echo "  Try running: cpm --version"
echo ""
