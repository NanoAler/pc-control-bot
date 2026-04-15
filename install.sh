#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "=== PC Control Bot Installer ==="
echo ""

# Detect distribution
if command -v pacman &> /dev/null; then
    echo "[*] Detected: Arch Linux"
    PKG_MANAGER="pacman"
elif command -v apt &> /dev/null; then
    echo "[*] Detected: Ubuntu/Debian"
    PKG_MANAGER="apt"
else
    echo "[!] Unsupported distribution"
    exit 1
fi

# Install dependencies
echo ""
echo "[*] Installing dependencies..."
if [ "$PKG_MANAGER" = "pacman" ]; then
    sudo pacman -S --needed rustup pulseaudio ffmpeg qt5-base spectacle rfkill 2>/dev/null
    rustup default stable 2>/dev/null || true
elif [ "$PKG_MANAGER" = "apt" ]; then
    sudo apt update -qq
    sudo apt install -y rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill 2>/dev/null
fi

# Build screenshot tool
echo ""
echo "[*] Building screenshot tool..."
cd src/utils
mkdir -p build && cd build
cmake .. > /dev/null 2>&1 && make > /dev/null 2>&1
sudo cp screenshot_tool /usr/local/bin/
cd ../../..

# Build bot
echo "[*] Building bot..."
cargo build --release 2>/dev/null

echo ""
read -p "Configure bot now? [Y/n]: " CONFIG_NOW
CONFIG_NOW=${CONFIG_NOW:-Y}

if [[ "$CONFIG_NOW" =~ ^[Yy]$ ]] || [[ -z "$CONFIG_NOW" ]]; then
    echo ""
    echo "[*] Bot configuration"
    echo ""
    read -p "Enter Telegram Bot Token (from @BotFather): " BOT_TOKEN
    read -p "Enter Your User ID (from @userinfobot): " USER_ID

    cat > .env << EOF
TELEOXIDE_TOKEN=$BOT_TOKEN
ALLOWED_USER_IDS=$USER_ID
EOF
    echo ""
    echo "[+] Configuration saved to .env"
fi

echo ""
echo "[+] Installation complete!"
echo ""
echo "Run: cargo run --release"
