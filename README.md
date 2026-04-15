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

## Quick Setup

Run the automated setup script:

```bash
chmod +x setup.sh
./setup.sh
```

The script will:
1. Install all dependencies
2. Build the bot and screenshot tool
3. Ask if you want to configure the bot now

## Manual Installation

### 1. Get Telegram Token

1. Open [@BotFather](https://t.me/BotFather) → create bot → copy token
2. Open [@userinfobot](https://t.me/userinfobot) → copy your user ID

### 2. Install Dependencies

**Arch Linux:**
```bash
sudo pacman -S rustup pulseaudio ffmpeg libqt5-core libqt5-gui spectacle rfkill
rustup default stable
```

**Ubuntu/Debian:**
```bash
sudo apt install rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
```

### 3. Build

```bash
cargo build --release
```

### 4. Build Screenshot Tool

```bash
cd src/utils
mkdir -p build && cd build
cmake ..
make
sudo cp screenshot_tool /usr/local/bin/
cd ../..
```

### 5. Configure

```bash
cp .env.example .env
nano .env
```

Add your token and user ID:
```
TELEOXIDE_TOKEN=your_bot_token_here
ALLOWED_USER_IDS=your_user_id_here
```

### 6. Run

```bash
cargo run --release
```

## Auto-start with systemd (Optional)

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
