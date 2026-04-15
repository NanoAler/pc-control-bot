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

1. Create a bot via [@BotFather](https://t.me/BotFather)
2. Get your user ID from [@userinfobot](https://t.me/userinfobot)
3. Run the setup script:
```bash
chmod +x install.sh && ./install.sh
```

## Manual Installation

### 1. Install Dependencies

**Arch Linux:**
```bash
sudo pacman -S rustup pulseaudio ffmpeg qt5-base spectacle rfkill
rustup default stable
```

**Ubuntu/Debian:**
```bash
sudo apt install rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
```

### 2. Build Screenshot Tool

```bash
cd src/utils
mkdir -p build && cd build
cmake ..
make
sudo cp screenshot_tool /usr/local/bin/
cd ../../..
```

### 3. Build

```bash
cargo build --release
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

### 5. Run

```bash
./target/release/pc_control_bot
```

### 6. Auto-start (Optional)

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
