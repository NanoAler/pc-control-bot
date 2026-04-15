use teloxide::{prelude::*, types::{ParseMode, InputFile}, utils::command::BotCommands};
use crate::commands::Command;
use crate::auth::Auth;
use crate::ui;
use crate::i18n::{Lang, LangStorage, T};

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn handle(bot: Bot, msg: Message, cmd: Command, auth: Auth, lang_storage: LangStorage) -> HandlerResult {
    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    if !auth.is_allowed(user_id).await {
        bot.send_message(msg.chat.id, T::get("access_denied", Lang::En)).await?;
        return Ok(());
    }

    let lang = lang_storage.get(user_id).await;

    match cmd {
        Command::Help => {
            let text = Command::descriptions().to_string();
            bot.send_message(msg.chat.id, text).parse_mode(ParseMode::Html).await?;
            show_menu(&bot, &msg, lang).await?;
        }
        Command::Menu => {
            show_menu(&bot, &msg, lang).await?;
        }
        Command::Volume(level) => { 
            crate::services::audio::set_volume(level).await; 
            let text = T::get("volume_set", lang).replace("{level}", &level.to_string());
            bot.send_message(msg.chat.id, text)
                .reply_markup(ui::volume_control(lang))
                .await?; 
        }
        Command::GetVolume => {
            match crate::services::audio::get_volume().await {
                Ok(vol) => {
                    let text = T::get("volume_set", lang).replace("{level}", &vol.to_string());
                    bot.send_message(msg.chat.id, text)
                        .reply_markup(ui::volume_control(lang))
                        .await?;
                }
                Err(e) => { 
                    let text = format!("{}: {}", T::get("error", lang), e);
                    bot.send_message(msg.chat.id, text).await?; 
                }
            }
        }
        Command::Brightness(level) => { 
            crate::services::display::set_brightness(level).await; 
            let text = T::get("brightness_set", lang).replace("{level}", &level.to_string());
            bot.send_message(msg.chat.id, text)
                .reply_markup(ui::brightness_control(lang))
                .await?; 
        }
        Command::Lock => { 
            crate::services::system::lock_screen().await; 
            bot.send_message(msg.chat.id, T::get("screen_locked", lang))
                .reply_markup(ui::back_to_menu(lang))
                .await?; 
        }
        Command::Microphone => { 
            crate::services::audio::toggle_microphone().await; 
            bot.send_message(msg.chat.id, T::get("microphone_toggled", lang))
                .reply_markup(ui::back_to_menu(lang))
                .await?; 
        }
        Command::Launch(app) => { 
            crate::services::apps::launch(&app).await; 
            let text = T::get("launched", lang).replace("{app}", &app);
            bot.send_message(msg.chat.id, text)
                .reply_markup(ui::back_to_menu(lang))
                .await?; 
        }
        Command::Exec(text) => {
            let result = crate::services::system::exec_command(&text, lang).await;
            let output = if result.len() > 4000 {
                format!("{}...\n\n(truncated)", &result[..4000])
            } else {
                result
            };
            bot.send_message(msg.chat.id, format!("```\n{}```", escape_for_code(&output)))
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(ui::back_to_menu(lang))
                .await?;
        }
        Command::Camera => {
            if let Some(path) = crate::services::camera::capture().await {
                bot.send_photo(msg.chat.id, InputFile::file(&path))
                    .reply_markup(ui::back_to_menu(lang))
                    .await?;
                let _ = std::fs::remove_file(&path);
            } else {
                bot.send_message(msg.chat.id, T::get("camera_not_found", lang))
                    .reply_markup(ui::back_to_menu(lang))
                    .await?;
            }
        }
        Command::Screenshot => {
            crate::services::display::screenshot().await;
            if let Some(path) = find_screenshot() {
                bot.send_photo(msg.chat.id, InputFile::file(&path))
                    .reply_markup(ui::back_to_menu(lang))
                    .await?;
                let _ = std::fs::remove_file(&path);
            } else {
                bot.send_message(msg.chat.id, T::get("screenshot_failed", lang))
                    .reply_markup(ui::back_to_menu(lang))
                    .await?;
            }
        }
        Command::Devices => {
            match crate::services::devices::list().await {
                Ok(devices) => {
                    let header = T::get("devices", lang);
                    bot.send_message(msg.chat.id, format!("{}:\n```\n{}```", header, escape_for_code(&devices)))
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(ui::back_to_menu(lang))
                        .await?;
                }
                Err(e) => { 
                    let text = format!("{}: {}", T::get("error", lang), e);
                    bot.send_message(msg.chat.id, text).await?; 
                }
            }
        }
        Command::Bluetooth => { 
            crate::services::bluetooth::toggle().await; 
            bot.send_message(msg.chat.id, T::get("bluetooth_toggled", lang))
                .reply_markup(ui::back_to_menu(lang))
                .await?; 
        }
        Command::Shutdown => {
            bot.send_message(msg.chat.id, T::get("shutdown_confirm", lang))
                .reply_markup(ui::confirm_action("shutdown", lang))
                .await?;
        }
        Command::Reboot => {
            bot.send_message(msg.chat.id, T::get("reboot_confirm", lang))
                .reply_markup(ui::confirm_action("reboot", lang))
                .await?;
        }
        Command::Sleep => {
            bot.send_message(msg.chat.id, T::get("sleep_confirm", lang))
                .reply_markup(ui::confirm_action("sleep", lang))
                .await?;
        }
        Command::Processes => {
            match crate::services::system::list_processes().await {
                Ok(procs) => {
                    bot.send_message(msg.chat.id, format!("Processes:\n```\n{}```", escape_for_code(&procs)))
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(ui::back_to_menu(lang))
                        .await?;
                }
                Err(e) => { 
                    let text = format!("{}: {}", T::get("error", lang), e);
                    bot.send_message(msg.chat.id, text).await?; 
                }
            }
        }
        Command::Cmd(text) => {
            let result = crate::services::system::exec_command(&text, lang).await;
            let output = if result.len() > 4000 {
                format!("{}...\n\n(truncated)", &result[..4000])
            } else {
                result
            };
            bot.send_message(msg.chat.id, format!("```\n{}```", escape_for_code(&output)))
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(ui::back_to_menu(lang))
                .await?;
        }
    }
    Ok(())
}

fn find_screenshot() -> Option<String> {
    if let Ok(entries) = std::fs::read_dir("/tmp") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("screenshot_") && name.ends_with(".png") {
                return Some(format!("/tmp/{}", name));
            }
        }
    }
    None
}

async fn show_menu(bot: &Bot, msg: &Message, lang: Lang) -> HandlerResult {
    bot.send_message(msg.chat.id, T::get("welcome", lang))
        .reply_markup(ui::main_menu(lang))
        .await?;
    Ok(())
}

fn escape_for_code(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('*', "\\*")
        .replace('_', "\\_")
}

pub async fn handle_callback(bot: Bot, query: teloxide::types::CallbackQuery, auth: Auth, lang_storage: LangStorage) -> HandlerResult {
    let user_id = query.from.id.0 as i64;

    if !auth.is_allowed(user_id).await {
        bot.answer_callback_query(&query.id).text(T::get("access_denied", Lang::En)).await?;
        return Ok(());
    }

    let data = query.data.as_deref().unwrap_or("");
    let lang = lang_storage.get(user_id).await;

    match data {
        "menu:lang" => {
            bot.answer_callback_query(&query.id).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("select_language", lang))
                    .reply_markup(ui::language_select())
                    .await?;
            }
        }
        "lang:ru" => {
            lang_storage.set(user_id, Lang::Ru).await;
            bot.answer_callback_query(&query.id).text("Язык изменён на Русский").await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("welcome", Lang::Ru))
                    .reply_markup(ui::main_menu(Lang::Ru))
                    .await?;
            }
        }
        "lang:en" => {
            lang_storage.set(user_id, Lang::En).await;
            bot.answer_callback_query(&query.id).text("Language changed to English").await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("welcome", Lang::En))
                    .reply_markup(ui::main_menu(Lang::En))
                    .await?;
            }
        }
        "menu:back" | "menu:main" => {
            bot.answer_callback_query(&query.id).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("welcome", lang))
                    .reply_markup(ui::main_menu(lang))
                    .await?;
            }
        }
        "menu:volume" => {
            bot.answer_callback_query(&query.id).await?;
            let vol = crate::services::audio::get_volume().await.unwrap_or(50);
            let text = T::get("volume_set", lang).replace("{level}", &vol.to_string());
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), text)
                    .reply_markup(ui::volume_control(lang))
                    .await?;
            }
        }
        "menu:brightness" => {
            bot.answer_callback_query(&query.id).await?;
            let text = T::get("brightness", lang).to_string();
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), text)
                    .reply_markup(ui::brightness_control(lang))
                    .await?;
            }
        }
        "menu:lock" => {
            crate::services::system::lock_screen().await;
            bot.answer_callback_query(&query.id).text(T::get("screen_locked", lang)).await?;
        }
        "menu:microphone" => {
            crate::services::audio::toggle_microphone().await;
            bot.answer_callback_query(&query.id).text(T::get("microphone_toggled", lang)).await?;
        }
        "menu:screenshot" => {
            bot.answer_callback_query(&query.id).text(T::get("screenshot_taken", lang)).await?;
            crate::services::display::screenshot().await;
            if let Some(msg) = &query.message {
                let chat_id = msg.chat().id;
                if let Some(path) = find_screenshot() {
                    bot.send_photo(chat_id, InputFile::file(&path)).await?;
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
        "menu:camera" => {
            bot.answer_callback_query(&query.id).text(T::get("camera_photo_taken", lang)).await?;
            if let Some(msg) = &query.message {
                let chat_id = msg.chat().id;
                if let Some(path) = crate::services::camera::capture().await {
                    bot.send_photo(chat_id, InputFile::file(&path)).await?;
                    let _ = std::fs::remove_file(&path);
                }
            }
        }
        "menu:bluetooth" => {
            crate::services::bluetooth::toggle().await;
            bot.answer_callback_query(&query.id).text(T::get("bluetooth_toggled", lang)).await?;
        }
        "menu:devices" => {
            bot.answer_callback_query(&query.id).await?;
            if let Some(msg) = &query.message {
                match crate::services::devices::list().await {
                    Ok(devices) => {
                        let header = T::get("devices", lang);
                        bot.send_message(msg.chat().id, format!("{}:\n```\n{}```", header, escape_for_code(&devices)))
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_markup(ui::back_to_menu(lang))
                            .await?;
                    }
                    Err(e) => {
                        let text = format!("{}: {}", T::get("error", lang), e);
                        bot.send_message(msg.chat().id, text).await?;
                    }
                }
            }
        }
        "menu:shutdown" => {
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("shutdown_confirm", lang))
                    .reply_markup(ui::confirm_action("shutdown", lang))
                    .await?;
            }
        }
        "menu:reboot" => {
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("reboot_confirm", lang))
                    .reply_markup(ui::confirm_action("reboot", lang))
                    .await?;
            }
        }
        "menu:sleep" => {
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("sleep_confirm", lang))
                    .reply_markup(ui::confirm_action("sleep", lang))
                    .await?;
            }
        }
        "confirm:shutdown" => {
            bot.answer_callback_query(&query.id).text(T::get("shutting_down", lang)).await?;
            crate::services::system::shutdown().await;
        }
        "confirm:reboot" => {
            bot.answer_callback_query(&query.id).text(T::get("rebooting", lang)).await?;
            crate::services::system::reboot().await;
        }
        "confirm:sleep" => {
            bot.answer_callback_query(&query.id).text(T::get("going_to_sleep", lang)).await?;
            crate::services::system::sleep().await;
        }
        "confirm:cancel" => {
            bot.answer_callback_query(&query.id).text(T::get("cancelled", lang)).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), T::get("welcome", lang))
                    .reply_markup(ui::main_menu(lang))
                    .await?;
            }
        }
        s if s.starts_with("vol:") => {
            let delta = &s[4..];
            let current = crate::services::audio::get_volume().await.unwrap_or(50);
            let new_vol = if delta.starts_with('+') {
                (current.saturating_add(delta[1..].parse::<u8>().unwrap_or(0))).min(100)
            } else if delta.starts_with('-') {
                current.saturating_sub(delta[1..].parse::<u8>().unwrap_or(0))
            } else {
                delta.parse::<u8>().unwrap_or(current)
            };
            crate::services::audio::set_volume(new_vol).await;
            let text = T::get("volume_set", lang).replace("{level}", &new_vol.to_string());
            bot.answer_callback_query(&query.id).text(&text).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), text)
                    .reply_markup(ui::volume_control(lang))
                    .await?;
            }
        }
        s if s.starts_with("bright:") => {
            let delta = &s[7..];
            let new_bright = if delta.starts_with('+') {
                (50u8.saturating_add(delta[1..].parse::<u8>().unwrap_or(0))).min(100)
            } else if delta.starts_with('-') {
                50u8.saturating_sub(delta[1..].parse::<u8>().unwrap_or(0))
            } else {
                delta.parse::<u8>().unwrap_or(50)
            };
            crate::services::display::set_brightness(new_bright).await;
            let text = T::get("brightness_set", lang).replace("{level}", &new_bright.to_string());
            bot.answer_callback_query(&query.id).text(&text).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), text)
                    .reply_markup(ui::brightness_control(lang))
                    .await?;
            }
        }
        _ => {
            bot.answer_callback_query(&query.id).text(T::get("unknown_command", lang)).await?;
        }
    }
    Ok(())
}
