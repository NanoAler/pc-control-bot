use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "show help")]
    Help,
    #[command(description = "show menu with buttons")]
    Menu,
    #[command(description = "set volume (0-100)")]
    Volume(u8),
    #[command(description = "get current volume")]
    GetVolume,
    #[command(description = "set brightness (0-100)")]
    Brightness(u8),
    #[command(description = "lock screen")]
    Lock,
    #[command(description = "toggle microphone")]
    Microphone,
    #[command(description = "launch application")]
    Launch(String),
    #[command(description = "execute command")]
    Exec(String),
    #[command(description = "take photo from camera")]
    Camera,
    #[command(description = "take screenshot")]
    Screenshot,
    #[command(description = "list devices")]
    Devices,
    #[command(description = "toggle bluetooth")]
    Bluetooth,
    #[command(description = "shutdown PC")]
    Shutdown,
    #[command(description = "reboot PC")]
    Reboot,
    #[command(description = "sleep mode")]
    Sleep,
    #[command(description = "list processes")]
    Processes,
    #[command(description = "remote command")]
    Cmd(String),
}
