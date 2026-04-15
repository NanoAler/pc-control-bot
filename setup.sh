#!/bin/bash
set -e

echo "========================================"
echo "  PC Control Bot - Setup"
echo "========================================"
echo ""

# Detect distribution
if command -v pacman &> /dev/null; then
    echo "Detected: Arch Linux"
    echo "Installing dependencies..."
    sudo pacman -S --needed rustup pulseaudio ffmpeg libqt5-core libqt5-gui spectacle rfkill --noconfirm
    rustup default stable
elif command -v apt &> /dev/null; then
    echo "Detected: Ubuntu/Debian"
    echo "Installing dependencies..."
    sudo apt update -qq
    sudo apt install -y rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
else
    echo "Error: Unsupported distribution"
    exit 1
fi

echo ""
echo "Building bot..."
cargo build --release

echo ""
echo "Building screenshot tool..."
cd src/utils
mkdir -p build && cd build
cmake .. > /dev/null 2>&1
make > /dev/null 2>&1
sudo cp screenshot_tool /usr/local/bin/
cd ../../..

echo ""
echo "========================================"
read -p "Configure bot now? [Y/n] " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    if [ -f .env ]; then
        echo ".env already exists, skipping configuration"
    else
        echo ""
        echo "Enter your Telegram bot token from @BotFather:"
        read -p "> " TOKEN

        echo ""
        echo "Enter your user ID from @userinfobot:"
        read -p "> " USER_ID

        cat > .env << EOF
TELEOXIDE_TOKEN=$TOKEN
ALLOWED_USER_IDS=$USER_ID
EOF
        echo ""
        echo "Configuration saved to .env"
    fi
fi

echo ""
echo "========================================"
echo "  Setup complete!"
echo "========================================"
echo ""
echo "Run: cargo run --release"
