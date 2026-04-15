use std::process::Command;

pub async fn set_volume(level: u8) {
    let level = level.min(100);
    let _ = Command::new("pactl")
        .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", level)])
        .output();
}

pub async fn get_volume() -> Result<u8, String> {
    let output = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for part in stdout.split_whitespace() {
        if part.ends_with('%') {
            if let Ok(v) = part.trim_end_matches('%').parse::<u8>() {
                return Ok(v.min(100));
            }
        }
        if let Ok(v) = part.parse::<u32>() {
            return Ok(((v as f32 / 65536.0) * 100.0) as u8);
        }
    }
    Ok(0)
}

pub async fn toggle_microphone() {
    let _ = Command::new("pactl")
        .args(["set-source-mute", "@DEFAULT_SOURCE@", "toggle"])
        .output();
}
