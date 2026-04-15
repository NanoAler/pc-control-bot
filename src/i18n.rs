use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    En,
    Ru,
}

impl Lang {
    pub fn as_str(&self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Ru => "ru",
        }
    }
}

#[derive(Clone)]
pub struct LangStorage {
    users: Arc<RwLock<HashMap<i64, Lang>>>,
}

impl LangStorage {
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, user_id: i64, lang: Lang) {
        let mut users = self.users.write().await;
        users.insert(user_id, lang);
    }

    pub async fn get(&self, user_id: i64) -> Lang {
        let users = self.users.read().await;
        users.get(&user_id).copied().unwrap_or(Lang::En)
    }
}

impl Default for LangStorage {
    fn default() -> Self {
        Self::new()
    }
}

pub struct T;

impl T {
    pub fn get(key: &str, lang: Lang) -> String {
        match lang {
            Lang::Ru => RU.get(key).copied().unwrap_or(key).to_string(),
            Lang::En => EN.get(key).copied().unwrap_or(key).to_string(),
        }
    }
}

static RU: once_cell::sync::Lazy<HashMap<&'static str, &'static str>> = 
    once_cell::sync::Lazy::new(|| {
        HashMap::from([
            ("welcome", "🎛️ Панель управления ПК"),
            ("access_denied", "⛔ Доступ запрещён"),
            ("volume", "Громкость"),
            ("volume_set", "🔊 Громкость: {level}%"),
            ("brightness", "Яркость"),
            ("brightness_set", "☀️ Яркость: {level}%"),
            ("screen_locked", "🔒 Экран заблокирован"),
            ("microphone_toggled", "🎤 Микрофон переключён"),
            ("launched", "Запущено: {app}"),
            ("camera_photo_taken", "📷 Фото сделано"),
            ("camera_not_found", "❌ Камера не найдена"),
            ("screenshot_taken", "📸 Скриншот сделан"),
            ("screenshot_failed", "❌ Не удалось сделать скриншот"),
            ("devices", "📋 Устройства"),
            ("bluetooth_toggled", "🔵 Bluetooth переключён"),
            ("shutdown_confirm", "⚠️ Вы уверены, что хотите выключить ПК?"),
            ("reboot_confirm", "⚠️ Вы уверены, что хотите перезагрузить ПК?"),
            ("sleep_confirm", "⚠️ Вы уверены, что хотите перевести ПК в спящий режим?"),
            ("shutting_down", "🛑 Выключаю..."),
            ("rebooting", "🔄 Перезагружаю..."),
            ("going_to_sleep", "💤 Засыпаю..."),
            ("cancelled", "✅ Отменено"),
            ("select_language", "Выберите язык:"),
            ("russian", "🇷🇺 Русский"),
            ("english", "🇬🇧 English"),
            ("error", "Ошибка"),
            ("done_no_output", "Команда выполнена (нет вывода)"),
            ("unknown_command", "Неизвестная команда"),
            ("back_to_menu", "🔙 Главное меню"),
            ("vol_0", "🔇"),
            ("vol_25", "🔈"),
            ("vol_50", "🔉"),
            ("vol_75", "🔊"),
            ("vol_minus", "◀️ -10"),
            ("vol_plus", "▶️ +10"),
            ("bright_0", "🌑"),
            ("bright_25", "🌙"),
            ("bright_50", "☀️"),
            ("bright_75", "🔆"),
            ("bright_minus", "◀️ -10"),
            ("bright_plus", "▶️ +10"),
        ])
    });

static EN: once_cell::sync::Lazy<HashMap<&'static str, &'static str>> = 
    once_cell::sync::Lazy::new(|| {
        HashMap::from([
            ("welcome", "🎛️ PC Control Panel"),
            ("access_denied", "⛔ Access denied"),
            ("volume", "Volume"),
            ("volume_set", "🔊 Volume: {level}%"),
            ("brightness", "Brightness"),
            ("brightness_set", "☀️ Brightness: {level}%"),
            ("screen_locked", "🔒 Screen locked"),
            ("microphone_toggled", "🎤 Microphone toggled"),
            ("launched", "Launched: {app}"),
            ("camera_photo_taken", "📷 Photo taken"),
            ("camera_not_found", "❌ Camera not found"),
            ("screenshot_taken", "📸 Screenshot taken"),
            ("screenshot_failed", "❌ Screenshot failed"),
            ("devices", "📋 Devices"),
            ("bluetooth_toggled", "🔵 Bluetooth toggled"),
            ("shutdown_confirm", "⚠️ Are you sure you want to shutdown?"),
            ("reboot_confirm", "⚠️ Are you sure you want to reboot?"),
            ("sleep_confirm", "⚠️ Are you sure you want to sleep?"),
            ("shutting_down", "🛑 Shutting down..."),
            ("rebooting", "🔄 Rebooting..."),
            ("going_to_sleep", "💤 Going to sleep..."),
            ("cancelled", "✅ Cancelled"),
            ("select_language", "Select language:"),
            ("russian", "🇷🇺 Русский"),
            ("english", "🇬🇧 English"),
            ("error", "Error"),
            ("done_no_output", "Done (no output)"),
            ("unknown_command", "Unknown command"),
            ("back_to_menu", "🔙 Main menu"),
            ("vol_0", "🔇"),
            ("vol_25", "🔈"),
            ("vol_50", "🔉"),
            ("vol_75", "🔊"),
            ("vol_minus", "◀️ -10"),
            ("vol_plus", "▶️ +10"),
            ("bright_0", "🌑"),
            ("bright_25", "🌙"),
            ("bright_50", "☀️"),
            ("bright_75", "🔆"),
            ("bright_minus", "◀️ -10"),
            ("bright_plus", "▶️ +10"),
        ])
    });
