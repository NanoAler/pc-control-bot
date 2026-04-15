# PC Control Bot

Telegram bot for remote PC control on Linux (KDE Wayland).

## Features

- Volume control (pulseaudio)
- Brightness control (KDE qdbus)
- Screen lock
- Microphone toggle
- Screenshot capture
- Camera capture
- Device listing
- Bluetooth toggle
- Process list
- App launch
- Remote command execution
- System control (shutdown/reboot/sleep)

## Requirements

- Linux with KDE Plasma
- Rust (stable)
- Telegram Bot Token

## Quick Install

```bash
chmod +x install.sh
./install.sh
```

The script will:
1. Install all dependencies (Arch/Ubuntu)
2. Build the screenshot tool
3. Build the bot
4. Ask if you want to configure the bot now

## Manual Setup

### 1. Create Telegram Bot

1. Open [@BotFather](https://t.me/BotFather) and create a new bot
2. Copy the bot token

### 2. Get Your User ID

1. Open [@userinfobot](https://t.me/userinfobot)
2. Copy your user ID (number)

### 3. Install Dependencies

**Arch Linux:**
```bash
sudo pacman -S rustup pulseaudio ffmpeg qt5-base spectacle rfkill
rustup default stable
```

**Ubuntu/Debian:**
```bash
sudo apt install rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
```

### 4. Configure

```bash
cp .env.example .env
```

Edit `.env`:
```
TELEOXIDE_TOKEN=your_bot_token_here
ALLOWED_USER_IDS=your_user_id_here
```

### 5. Build Screenshot Tool

```bash
cd src/utils
mkdir -p build && cd build
cmake ..
make
sudo cp screenshot_tool /usr/local/bin/
```

### 6. Build and Run

```bash
cargo build --release
cargo run --release
```

### 7. Run on Startup (Optional)

```bash
sudo tee /etc/systemd/system/pc-control-bot.service > /dev/null <<EOF
[Unit]
Description=PC Control Telegram Bot
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/target/release/pc_control_bot
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable pc-control-bot
sudo systemctl start pc-control-bot
```

## Commands

| Command | Description |
|---------|-------------|
| `/help` | Show help |
| `/menu` | Show interactive menu |
| `/volume <0-100>` | Set volume |
| `/getvolume` | Get current volume |
| `/brightness <0-100>` | Set brightness |
| `/lock` | Lock screen |
| `/mic` | Toggle microphone |
| `/screenshot` | Take screenshot |
| `/camera` | Take photo |
| `/devices` | List devices |
| `/bluetooth` | Toggle bluetooth |
| `/shutdown` | Shutdown PC |
| `/reboot` | Reboot PC |
| `/sleep` | Sleep mode |
| `/processes` | List processes |
| `/launch <app>` | Launch application |
| `/cmd <command>` | Execute shell command |

## License

MIT
