mod commands;
mod handlers;
mod services;
mod auth;
mod ui;
mod i18n;

use log::info;
use teloxide::prelude::*;
use dptree::entry;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    info!("Starting PC Control Bot...");

    let bot = Bot::from_env();

    let auth = auth::Auth::new();
    let lang_storage = i18n::LangStorage::new();

    if let Ok(allowed_ids) = std::env::var("ALLOWED_USER_IDS") {
        let ids: Vec<i64> = allowed_ids
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        auth.add_users(&ids).await;
        info!("Added {} allowed users", ids.len());
    }

    let auth_clone = auth.clone();
    let lang_clone = lang_storage.clone();

    let handler = entry()
        .branch(
            Update::filter_message()
                .filter_command::<commands::Command>()
                .endpoint(move |bot, msg, cmd| {
                    handlers::handle(bot, msg, cmd, auth_clone.clone(), lang_clone.clone())
                }),
        )
        .branch(
            Update::filter_callback_query()
                .endpoint(move |bot, q: teloxide::types::CallbackQuery| {
                    handlers::handle_callback(bot, q, auth.clone(), lang_storage.clone())
                }),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
