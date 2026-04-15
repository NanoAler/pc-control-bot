#!/bin/bash
set -e

echo "PC Control Bot - Quick Install"

# Detect distribution
if command -v pacman &> /dev/null; then
    echo "Detected: Arch Linux"
    PKG_MANAGER="pacman"
    echo "Installing dependencies..."
    sudo pacman -S --needed rustup pulseaudio ffmpeg qt5-base spectacle rfkill
    rustup default stable
elif command -v apt &> /dev/null; then
    echo "Detected: Ubuntu/Debian"
    PKG_MANAGER="apt"
    echo "Installing dependencies..."
    sudo apt update
    sudo apt install -y rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
else
    echo "Unsupported distribution. Please install dependencies manually."
    exit 1
fi

# Build
echo "Building..."
cargo build --release

# Install screenshot tool
echo "Installing screenshot tool..."
cd src/utils
mkdir -p build && cd build
cmake .. && make
sudo cp screenshot_tool /usr/local/bin/
cd ../../..

# Create .env if not exists
if [ ! -f .env ]; then
    cp .env.example .env
    echo "Created .env file. Please edit it with your Telegram bot token and user ID."
else
    echo ".env already exists"
fi

echo ""
echo "Done! Edit .env and run: cargo run --release"
