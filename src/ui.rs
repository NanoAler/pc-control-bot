use crate::i18n::{Lang, T};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn language_select() -> InlineKeyboardMarkup {
    let buttons = vec![vec![
        InlineKeyboardButton::callback(T::get("russian", Lang::Ru), "lang:ru"),
        InlineKeyboardButton::callback(T::get("english", Lang::En), "lang:en"),
    ]];
    InlineKeyboardMarkup::new(buttons)
}

pub fn main_menu(lang: Lang) -> InlineKeyboardMarkup {
    let (
        volume,
        brightness,
        lock,
        mic,
        screenshot,
        camera,
        devices,
        bluetooth,
        shutdown,
        reboot,
        sleep,
        lang_text,
    ) = if lang == Lang::Ru {
        (
            "Громкость",
            "Яркость",
            "Экран",
            "Микрофон",
            "Скриншот",
            "Камера",
            "Устройства",
            "Bluetooth",
            "Выключение",
            "Перезагрузка",
            "Сон",
            "Язык",
        )
    } else {
        (
            "Volume",
            "Brightness",
            "Screen",
            "Microphone",
            "Screenshot",
            "Camera",
            "Devices",
            "Bluetooth",
            "Shutdown",
            "Reboot",
            "Sleep",
            "Language",
        )
    };

    let buttons = vec![
        vec![
            InlineKeyboardButton::callback(&format!("🔊 {}", volume), "menu:volume"),
            InlineKeyboardButton::callback(&format!("☀️ {}", brightness), "menu:brightness"),
        ],
        vec![
            InlineKeyboardButton::callback(&format!("🔒 {}", lock), "menu:lock"),
            InlineKeyboardButton::callback(&format!("🎤 {}", mic), "menu:microphone"),
        ],
        vec![
            InlineKeyboardButton::callback(&format!("📸 {}", screenshot), "menu:screenshot"),
            InlineKeyboardButton::callback(&format!("📷 {}", camera), "menu:camera"),
        ],
        vec![
            InlineKeyboardButton::callback(&format!("📋 {}", devices), "menu:devices"),
            InlineKeyboardButton::callback(&format!("🔵 {}", bluetooth), "menu:bluetooth"),
        ],
        vec![
            InlineKeyboardButton::callback(&format!("🛑 {}", shutdown), "menu:shutdown"),
            InlineKeyboardButton::callback(&format!("🔄 {}", reboot), "menu:reboot"),
        ],
        vec![InlineKeyboardButton::callback(
            &format!("💤 {}", sleep),
            "menu:sleep",
        )],
        vec![InlineKeyboardButton::callback(
            &format!("🌐 {}", lang_text),
            "menu:lang",
        )],
    ];
    InlineKeyboardMarkup::new(buttons)
}

pub fn confirm_action(action: &str, lang: Lang) -> InlineKeyboardMarkup {
    let confirm_text = match (action, lang) {
        ("shutdown", Lang::Ru) => "🛑 Да, выключить",
        ("shutdown", Lang::En) => "🛑 Yes, shutdown",
        ("reboot", Lang::Ru) => "🔄 Да, перезагрузить",
        ("reboot", Lang::En) => "🔄 Yes, reboot",
        ("sleep", Lang::Ru) => "💤 Да, уснуть",
        ("sleep", Lang::En) => "💤 Yes, sleep",
        _ => "✅ Yes",
    };

    let cancel_text = if lang == Lang::Ru {
        "❌ Отмена"
    } else {
        "❌ Cancel"
    };

    let buttons = vec![vec![
        InlineKeyboardButton::callback(confirm_text, &format!("confirm:{}", action)),
        InlineKeyboardButton::callback(cancel_text, "confirm:cancel"),
    ]];
    InlineKeyboardMarkup::new(buttons)
}

pub fn volume_control(lang: Lang) -> InlineKeyboardMarkup {
    let buttons = vec![
        vec![
            InlineKeyboardButton::callback(T::get("vol_0", lang), "vol:0"),
            InlineKeyboardButton::callback(T::get("vol_25", lang), "vol:25"),
            InlineKeyboardButton::callback(T::get("vol_50", lang), "vol:50"),
            InlineKeyboardButton::callback(T::get("vol_75", lang), "vol:75"),
            InlineKeyboardButton::callback("MAX", "vol:100"),
        ],
        vec![
            InlineKeyboardButton::callback(T::get("vol_minus", lang), "vol:-10"),
            InlineKeyboardButton::callback(T::get("vol_plus", lang), "vol:+10"),
        ],
        vec![InlineKeyboardButton::callback(
            T::get("back_to_menu", lang),
            "menu:back",
        )],
    ];
    InlineKeyboardMarkup::new(buttons)
}

pub fn brightness_control(lang: Lang) -> InlineKeyboardMarkup {
    let buttons = vec![
        vec![
            InlineKeyboardButton::callback(T::get("bright_0", lang), "bright:0"),
            InlineKeyboardButton::callback(T::get("bright_25", lang), "bright:25"),
            InlineKeyboardButton::callback(T::get("bright_50", lang), "bright:50"),
            InlineKeyboardButton::callback(T::get("bright_75", lang), "bright:75"),
            InlineKeyboardButton::callback("MAX", "bright:100"),
        ],
        vec![
            InlineKeyboardButton::callback(T::get("bright_minus", lang), "bright:-10"),
            InlineKeyboardButton::callback(T::get("bright_plus", lang), "bright:+10"),
        ],
        vec![InlineKeyboardButton::callback(
            T::get("back_to_menu", lang),
            "menu:back",
        )],
    ];
    InlineKeyboardMarkup::new(buttons)
}

pub fn back_to_menu(lang: Lang) -> InlineKeyboardMarkup {
    let buttons = vec![vec![InlineKeyboardButton::callback(
        T::get("back_to_menu", lang),
        "menu:back",
    )]];
    InlineKeyboardMarkup::new(buttons)
}
