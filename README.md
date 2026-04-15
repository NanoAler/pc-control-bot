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

- Rust (stable)
- Telegram Bot Token
- Linux with KDE Plasma
- Dependencies: `pactl`, `ffmpeg`, `qdbus`, `loginctl`, `spectacle`, `rfkill`

## Setup

1. Create a bot via [@BotFather](https://t.me/BotFather)

2. Configure environment:
```bash
cp .env.example .env
```

Edit `.env`:
```
TELEOXIDE_TOKEN=your_bot_token
ALLOWED_USER_IDS=your_telegram_user_id
```

3. Build and run:
```bash
cargo build --release
cargo run --release
```

## Build Screenshot Tool

The screenshot tool requires Qt6:
```bash
cd src/utils
mkdir -p build && cd build
cmake ..
make
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
