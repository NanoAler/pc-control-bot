# PC Control Bot

Telegram бот для удалённого управления ПК на Linux (KDE Wayland).

## Возможности

- Управление громкостью (pulseaudio)
- Управление яркостью (KDE qdbus)
- Блокировка экрана
- Переключение микрофона
- Снимок экрана
- Фото с камеры
- Список устройств
- Переключение Bluetooth
- Список процессов
- Запуск приложений
- Выполнение команд
- Системные команды (выключение/перезагрузка/сон)

## Требования

- Linux с KDE Plasma
- Rust (stable)
- Токен Telegram бота

## Быстрая установка

1. Создайте бота через [@BotFather](https://t.me/BotFather)
2. Получите свой ID через [@userinfobot](https://t.me/userinfobot)
3. Запустите скрипт установки:
```bash
chmod +x install.sh && ./install.sh
```

## Ручная установка

### 1. Установка зависимостей

**Arch Linux:**
```bash
sudo pacman -S rustup pulseaudio ffmpeg qt5-base spectacle rfkill
rustup default stable
```

**Ubuntu/Debian:**
```bash
sudo apt install rustc cargo pulseaudio ffmpeg qtbase5-dev spectacle rfkill
```

### 2. Сборка инструмента для скриншотов

```bash
cd src/utils
mkdir -p build && cd build
cmake ..
make
sudo cp screenshot_tool /usr/local/bin/
cd ../../..
```

### 3. Сборка

```bash
cargo build --release
```

### 4. Настройка

```bash
cp .env.example .env
```

Отредактируйте `.env`:
```
TELEOXIDE_TOKEN=ваш_токен_бота
ALLOWED_USER_IDS=ваш_user_id
```

### 5. Запуск

```bash
./target/release/pc_control_bot
```

### 6. Автозапуск (опционально)

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

## Команды

| Команда | Описание |
|---------|----------|
| `/help` | Показать помощь |
| `/menu` | Показать меню с кнопками |
| `/volume <0-100>` | Установить громкость |
| `/getvolume` | Получить текущую громкость |
| `/brightness <0-100>` | Установить яркость |
| `/lock` | Заблокировать экран |
| `/mic` | Переключить микрофон |
| `/screenshot` | Сделать скриншот |
| `/camera` | Сделать фото с камеры |
| `/devices` | Список устройств |
| `/bluetooth` | Переключить Bluetooth |
| `/shutdown` | Выключить ПК |
| `/reboot` | Перезагрузить ПК |
| `/sleep` | Спящий режим |
| `/processes` | Список процессов |
| `/launch <приложение>` | Запустить приложение |
| `/cmd <команда>` | Выполнить команду |

## Лицензия

MIT
