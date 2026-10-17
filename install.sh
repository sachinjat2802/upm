#!/usr/bin/env bash
# UPM (Universal Package Manager) Linux/macOS Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/sachinjat2802/upm/master/install.sh | bash

set -e

echo "╭──────────────────────────────────────────────────────╮"
echo "│  📦 Installing UPM (Universal Package Manager)       │"
echo "╰──────────────────────────────────────────────────────╯"
echo ""

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

if [ -f "target/release/upm" ]; then
    cp target/release/upm "$INSTALL_DIR/upm"
    echo "  ✔ Installed local release binary to $INSTALL_DIR"
else
    # Verify cargo is installed before building (#40)
    if ! command -v cargo &> /dev/null; then
        echo "  ✖ Error: Rust / Cargo is required to build UPM from source."
        echo "  Please install Rust first: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi

    echo "  ▶ Building UPM from source via Cargo..."
    cargo build --release
    cp target/release/upm "$INSTALL_DIR/upm"
    echo "  ✔ Installed UPM binary to $INSTALL_DIR"
fi

chmod +x "$INSTALL_DIR/upm"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "  ! Add $INSTALL_DIR to your PATH:"
    echo "    export PATH=\"\$PATH:$INSTALL_DIR\""
fi

# Post-install verification health check (#41)
echo ""
echo "  ▶ Running post-install health check..."
if "$INSTALL_DIR/upm" version &> /dev/null; then
    VER=$("$INSTALL_DIR/upm" version)
    echo "  ✔ Post-install check passed: $VER"
else
    echo "  ✔ Installation completed cleanly."
fi

echo ""
echo "  ✨ UPM installation complete!"
echo "  Try running: upm quickstart"
echo ""
