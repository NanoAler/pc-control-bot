#!/bin/bash
set -e

echo "=== PC Control Bot Setup ==="
echo ""

# Detect distribution
if command -v pacman &> /dev/null; then
    echo "Detected: Arch Linux"
    echo "Installing dependencies..."
    sudo pacman -S --needed rustup pulseaudio ffmpeg qt5-base spectacle rfkill || true
    rustup default stable
elif command -v apt &> /dev/null; then
    echo "Detected: Ubuntu/Debian"
    echo "Installing dependencies..."
    sudo apt update
    sudo apt install -y rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
else
    echo "Unsupported distribution. Install dependencies manually."
fi

# Build screenshot tool
echo ""
echo "Building screenshot tool..."
cd src/utils
mkdir -p build && cd build
cmake .. > /dev/null 2>&1
make > /dev/null 2>&1
sudo cp screenshot_tool /usr/local/bin/
cd ../../..

# Build bot
echo "Building bot..."
cargo build --release

# Configure
echo ""
read -p "Configure bot now? [Y/n]: " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    if [ -f .env ]; then
        read -p ".env exists. Overwrite? [y/N]: " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Skipping configuration."
        else
            echo ""
            echo "Enter your Telegram bot token (from @BotFather):"
            read -p "Token: " token

            echo ""
            echo "Enter your user ID (from @userinfobot):"
            read -p "User ID: " user_id

            cat > .env << EOF
TELEOXIDE_TOKEN=$token
ALLOWED_USER_IDS=$user_id
EOF
            echo ".env created!"
        fi
    else
        echo ""
        echo "Enter your Telegram bot token (from @BotFather):"
        read -p "Token: " token

        echo ""
        echo "Enter your user ID (from @userinfobot):"
        read -p "User ID: " user_id

        cat > .env << EOF
TELEOXIDE_TOKEN=$token
ALLOWED_USER_IDS=$user_id
EOF
        echo ".env created!"
    fi
fi

# Auto-start
echo ""
read -p "Enable auto-start with systemd? [y/N]: " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    WORKDIR=$(pwd)
    cat > /tmp/pc-control-bot.service << EOF
[Unit]
Description=PC Control Telegram Bot
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$WORKDIR
ExecStart=$WORKDIR/target/release/pc_control_bot
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF
    sudo cp /tmp/pc-control-bot.service /etc/systemd/system/
    sudo systemctl daemon-reload
    sudo systemctl enable pc-control-bot
    echo "Auto-start enabled!"
fi

echo ""
echo "Done! Run: ./target/release/pc_control_bot"
