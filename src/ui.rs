use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn main_menu() -> InlineKeyboardMarkup {
    let buttons = vec![
        vec![
            InlineKeyboardButton::callback("🔊 Громкость", "menu:volume"),
            InlineKeyboardButton::callback("☀️ Яркость", "menu:brightness"),
        ],
        vec![
            InlineKeyboardButton::callback("🔒 Экран", "menu:lock"),
            InlineKeyboardButton::callback("🎤 Микрофон", "menu:microphone"),
        ],
        vec![
            InlineKeyboardButton::callback("📸 Скриншот", "menu:screenshot"),
            InlineKeyboardButton::callback("📷 Камера", "menu:camera"),
        ],
        vec![
            InlineKeyboardButton::callback("📋 Устройства", "menu:devices"),
            InlineKeyboardButton::callback("📊 Процессы", "menu:processes"),
        ],
        vec![InlineKeyboardButton::callback(
            "🔵 Bluetooth",
            "menu:bluetooth",
        )],
        vec![
            InlineKeyboardButton::callback("🛑 Выключение", "menu:shutdown"),
            InlineKeyboardButton::callback("🔄 Перезагрузка", "menu:reboot"),
        ],
        vec![InlineKeyboardButton::callback("💤 Сон", "menu:sleep")],
    ];

    InlineKeyboardMarkup::new(buttons)
}

pub fn confirm_action(action: &str) -> InlineKeyboardMarkup {
    let action_emoji = match action {
        "shutdown" => "🛑",
        "reboot" => "🔄",
        "sleep" => "💤",
        _ => "⚠️",
    };

    let buttons = vec![vec![
        InlineKeyboardButton::callback(
            format!("✅ Да, {}", action_emoji),
            format!("confirm:{}", action),
        ),
        InlineKeyboardButton::callback("❌ Отмена", "confirm:cancel"),
    ]];

    InlineKeyboardMarkup::new(buttons)
}

pub fn volume_control() -> InlineKeyboardMarkup {
    let buttons = vec![
        vec![
            InlineKeyboardButton::callback("🔇", "vol:0"),
            InlineKeyboardButton::callback("🔈", "vol:25"),
            InlineKeyboardButton::callback("🔉", "vol:50"),
            InlineKeyboardButton::callback("🔊", "vol:75"),
            InlineKeyboardButton::callback("MAX", "vol:100"),
        ],
        vec![
            InlineKeyboardButton::callback("◀️ -10", "vol:-10"),
            InlineKeyboardButton::callback("▶️ +10", "vol:+10"),
        ],
        vec![InlineKeyboardButton::callback("🔙 Назад", "menu:back")],
    ];

    InlineKeyboardMarkup::new(buttons)
}

pub fn brightness_control() -> InlineKeyboardMarkup {
    let buttons = vec![
        vec![
            InlineKeyboardButton::callback("🌑", "bright:0"),
            InlineKeyboardButton::callback("🌙", "bright:25"),
            InlineKeyboardButton::callback("☀️", "bright:50"),
            InlineKeyboardButton::callback("🔆", "bright:75"),
            InlineKeyboardButton::callback("MAX", "bright:100"),
        ],
        vec![
            InlineKeyboardButton::callback("◀️ -10", "bright:-10"),
            InlineKeyboardButton::callback("▶️ +10", "bright:+10"),
        ],
        vec![InlineKeyboardButton::callback("🔙 Назад", "menu:back")],
    ];

    InlineKeyboardMarkup::new(buttons)
}

pub fn back_to_menu() -> InlineKeyboardMarkup {
    let buttons = vec![vec![InlineKeyboardButton::callback(
        "🔙 Главное меню",
        "menu:back",
    )]];

    InlineKeyboardMarkup::new(buttons)
}
