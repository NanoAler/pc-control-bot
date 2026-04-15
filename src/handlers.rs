use teloxide::{prelude::*, types::{ParseMode, InputFile}, utils::command::BotCommands};
use crate::commands::Command;
use crate::auth::Auth;
use crate::ui;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn handle(bot: Bot, msg: Message, cmd: Command, auth: Auth) -> HandlerResult {
    let user_id = msg.from.as_ref().map(|u| u.id.0 as i64).unwrap_or(0);

    if !auth.is_allowed(user_id).await {
        bot.send_message(msg.chat.id, "Access denied").await?;
        return Ok(());
    }

    match cmd {
        Command::Help => {
            let text = Command::descriptions().to_string();
            bot.send_message(msg.chat.id, text).parse_mode(ParseMode::Html).await?;
            show_menu(&bot, &msg).await?;
        }
        Command::Menu => {
            show_menu(&bot, &msg).await?;
        }
        Command::Volume(level) => { 
            crate::services::audio::set_volume(level).await; 
            bot.send_message(msg.chat.id, format!("Volume: {}%", level))
                .reply_markup(ui::volume_control())
                .await?; 
        }
        Command::GetVolume => {
            match crate::services::audio::get_volume().await {
                Ok(vol) => {
                    bot.send_message(msg.chat.id, format!("Volume: {}%", vol))
                        .reply_markup(ui::volume_control())
                        .await?;
                }
                Err(e) => { bot.send_message(msg.chat.id, format!("Error: {}", e)).await?; }
            }
        }
        Command::Brightness(level) => { 
            crate::services::display::set_brightness(level).await; 
            bot.send_message(msg.chat.id, format!("Brightness: {}%", level))
                .reply_markup(ui::brightness_control())
                .await?; 
        }
        Command::Lock => { 
            crate::services::system::lock_screen().await; 
            bot.send_message(msg.chat.id, "Screen locked")
                .reply_markup(ui::back_to_menu())
                .await?; 
        }
        Command::Microphone => { 
            crate::services::audio::toggle_microphone().await; 
            bot.send_message(msg.chat.id, "Microphone toggled")
                .reply_markup(ui::back_to_menu())
                .await?; 
        }
        Command::Launch(app) => { 
            crate::services::apps::launch(&app).await; 
            bot.send_message(msg.chat.id, format!("Launched: {}", app))
                .reply_markup(ui::back_to_menu())
                .await?; 
        }
        Command::Exec(text) => {
            let result = crate::services::system::exec_command(&text).await;
            let output = if result.len() > 4000 {
                format!("{}...\n\n(truncated)", &result[..4000])
            } else {
                result
            };
            bot.send_message(msg.chat.id, format!("```\n{}```", escape_for_code(&output)))
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(ui::back_to_menu())
                .await?;
        }
        Command::Camera => {
            if let Some(path) = crate::services::camera::capture().await {
                bot.send_photo(msg.chat.id, InputFile::file(&path))
                    .reply_markup(ui::back_to_menu())
                    .await?;
                let _ = std::fs::remove_file(&path);
            } else {
                bot.send_message(msg.chat.id, "Camera not found")
                    .reply_markup(ui::back_to_menu())
                    .await?;
            }
        }
        Command::Screenshot => {
            crate::services::display::screenshot().await;
            if let Some(path) = find_screenshot() {
                bot.send_photo(msg.chat.id, InputFile::file(&path))
                    .reply_markup(ui::back_to_menu())
                    .await?;
                let _ = std::fs::remove_file(&path);
            } else {
                bot.send_message(msg.chat.id, "Screenshot failed")
                    .reply_markup(ui::back_to_menu())
                    .await?;
            }
        }
        Command::Devices => {
            match crate::services::devices::list().await {
                Ok(devices) => {
                    bot.send_message(msg.chat.id, format!("Devices:\n```\n{}```", escape_for_code(&devices)))
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(ui::back_to_menu())
                        .await?;
                }
                Err(e) => { bot.send_message(msg.chat.id, format!("Error: {}", e)).await?; }
            }
        }
        Command::Bluetooth => { 
            crate::services::bluetooth::toggle().await; 
            bot.send_message(msg.chat.id, "Bluetooth toggled")
                .reply_markup(ui::back_to_menu())
                .await?; 
        }
        Command::Shutdown => {
            bot.send_message(msg.chat.id, "Are you sure you want to shutdown?")
                .reply_markup(ui::confirm_action("shutdown"))
                .await?;
        }
        Command::Reboot => {
            bot.send_message(msg.chat.id, "Are you sure you want to reboot?")
                .reply_markup(ui::confirm_action("reboot"))
                .await?;
        }
        Command::Sleep => {
            bot.send_message(msg.chat.id, "Are you sure you want to sleep?")
                .reply_markup(ui::confirm_action("sleep"))
                .await?;
        }
        Command::Processes => {
            match crate::services::system::list_processes().await {
                Ok(procs) => {
                    bot.send_message(msg.chat.id, format!("Processes:\n```\n{}```", escape_for_code(&procs)))
                        .parse_mode(ParseMode::MarkdownV2)
                        .reply_markup(ui::back_to_menu())
                        .await?;
                }
                Err(e) => { bot.send_message(msg.chat.id, format!("Error: {}", e)).await?; }
            }
        }
        Command::Cmd(text) => {
            let result = crate::services::system::exec_command(&text).await;
            let output = if result.len() > 4000 {
                format!("{}...\n\n(truncated)", &result[..4000])
            } else {
                result
            };
            bot.send_message(msg.chat.id, format!("```\n{}```", escape_for_code(&output)))
                .parse_mode(ParseMode::MarkdownV2)
                .reply_markup(ui::back_to_menu())
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

async fn show_menu(bot: &Bot, msg: &Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "PC Control Panel")
        .reply_markup(ui::main_menu())
        .await?;
    Ok(())
}

fn escape_for_code(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace('*', "\\*")
        .replace('_', "\\_")
}

pub async fn handle_callback(bot: Bot, query: teloxide::types::CallbackQuery, auth: Auth) -> HandlerResult {
    let user_id = query.from.id.0 as i64;

    if !auth.is_allowed(user_id).await {
        bot.answer_callback_query(&query.id).text("Access denied").await?;
        return Ok(());
    }

    let data = query.data.as_deref().unwrap_or("");

    match data {
        "menu:back" | "menu:main" => {
            bot.answer_callback_query(&query.id).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), "PC Control Panel")
                    .reply_markup(ui::main_menu())
                    .await?;
            }
        }
        "menu:volume" => {
            bot.answer_callback_query(&query.id).await?;
            let vol = crate::services::audio::get_volume().await.unwrap_or(50);
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), &format!("Volume: {}%", vol))
                    .reply_markup(ui::volume_control())
                    .await?;
            }
        }
        "menu:brightness" => {
            bot.answer_callback_query(&query.id).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), "Brightness Control")
                    .reply_markup(ui::brightness_control())
                    .await?;
            }
        }
        "menu:lock" => {
            crate::services::system::lock_screen().await;
            bot.answer_callback_query(&query.id).text("Screen locked").await?;
        }
        "menu:microphone" => {
            crate::services::audio::toggle_microphone().await;
            bot.answer_callback_query(&query.id).text("Microphone toggled").await?;
        }
        "menu:screenshot" => {
            bot.answer_callback_query(&query.id).text("📸 Screenshot taken").await?;
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
            bot.answer_callback_query(&query.id).text("📷 Photo taken").await?;
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
            bot.answer_callback_query(&query.id).text("Bluetooth toggled").await?;
        }
        "menu:devices" => {
            bot.answer_callback_query(&query.id).await?;
            if let Some(msg) = &query.message {
                match crate::services::devices::list().await {
                    Ok(devices) => {
                        bot.send_message(msg.chat().id, format!("Devices:\n```\n{}```", escape_for_code(&devices)))
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_markup(ui::back_to_menu())
                            .await?;
                    }
                    Err(e) => {
                        bot.send_message(msg.chat().id, format!("Error: {}", e))
                            .reply_markup(ui::back_to_menu())
                            .await?;
                    }
                }
            }
        }
        "menu:shutdown" => {
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), "Are you sure you want to shutdown?")
                    .reply_markup(ui::confirm_action("shutdown"))
                    .await?;
            }
        }
        "menu:reboot" => {
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), "Are you sure you want to reboot?")
                    .reply_markup(ui::confirm_action("reboot"))
                    .await?;
            }
        }
        "menu:sleep" => {
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), "Are you sure you want to sleep?")
                    .reply_markup(ui::confirm_action("sleep"))
                    .await?;
            }
        }
        "confirm:shutdown" => {
            bot.answer_callback_query(&query.id).text("Shutting down...").await?;
            crate::services::system::shutdown().await;
        }
        "confirm:reboot" => {
            bot.answer_callback_query(&query.id).text("Rebooting...").await?;
            crate::services::system::reboot().await;
        }
        "confirm:sleep" => {
            bot.answer_callback_query(&query.id).text("Going to sleep...").await?;
            crate::services::system::sleep().await;
        }
        "confirm:cancel" => {
            bot.answer_callback_query(&query.id).text("Cancelled").await?;
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
            bot.answer_callback_query(&query.id).text(&format!("Volume: {}%", new_vol)).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), &format!("Volume: {}%", new_vol))
                    .reply_markup(ui::volume_control())
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
            bot.answer_callback_query(&query.id).text(&format!("Brightness: {}%", new_bright)).await?;
            if let Some(msg) = &query.message {
                bot.edit_message_text(msg.chat().id, msg.id(), &format!("Brightness: {}%", new_bright))
                    .reply_markup(ui::brightness_control())
                    .await?;
            }
        }
        _ => {
            bot.answer_callback_query(&query.id).text("Unknown command").await?;
        }
    }
    Ok(())
}
