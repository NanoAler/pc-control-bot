# PC Control Bot v0.1.0

Telegram bot for remote PC control on Linux (KDE Wayland).

## Features

- **Media Controls**: Volume (0-100), Brightness (0-100), Microphone toggle
- **Screen**: Lock screen, Screenshot capture, Camera photo
- **System**: Shutdown, Reboot, Sleep mode
- **Device Management**: USB/input device list, Bluetooth toggle
- **Process Monitoring**: View running processes
- **Remote Access**: Launch applications, Execute shell commands
- **Interactive UI**: Inline keyboard menu with quick actions

## Quick Setup

1. Download `pc_control_bot` binary
2. Create `.env` file with your bot token and user ID
3. Run `./pc_control_bot`

## Usage

```
/menu     - Interactive control panel
/volume 50   - Set volume to 50%
/brightness 75 - Set brightness to 75%
/screenshot  - Capture screen
/camera      - Take photo from webcam
/shutdown    - Power off PC (with confirmation)
/cmd ls -la  - Execute shell command
```

## Requirements

- Linux with KDE Plasma
- pulseaudio, ffmpeg, spectacle, rfkill

## Download

See releases for pre-built binaries.
