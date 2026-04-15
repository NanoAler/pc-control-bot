use std::process::Command;
use std::path::Path;

pub async fn set_brightness(level: u8) {
    let level = level.min(100);

    if let Ok(output) = Command::new("qdbus")
        .args([
            "org.kde.Solid.PowerManagement",
            "/org/kde/Solid/PowerManagement/Actions/BrightnessControl",
            "org.kde.Solid.PowerManagement.Actions.BrightnessControl.brightnessMax"
        ])
        .output()
    {
        let max: i32 = String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .unwrap_or(100);

        let value = (level as i32 * max) / 100;

        let _ = Command::new("qdbus")
            .args([
                "org.kde.Solid.PowerManagement",
                "/org/kde/Solid/PowerManagement/Actions/BrightnessControl",
                "org.kde.Solid.PowerManagement.Actions.BrightnessControl.setBrightness",
                &value.to_string()
            ])
            .output();
    }
}

pub async fn screenshot() {
    let path = format!("/tmp/screenshot_{}.png", chrono::Local::now().format("%Y%m%d_%H%M%S"));

    let tool_paths = [
        "/home/nani/Documents/pc-control/target/release/screenshot_tool",
        "/home/nani/Documents/pc-control/src/utils/screenshot_tool",
    ];

    for tool in tool_paths.iter() {
        if Path::new(tool).exists() {
            let _ = Command::new(tool).output();
            break;
        }
    }

    if let Ok(entries) = std::fs::read_dir("/tmp") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("screenshot_") && name.ends_with(".png") {
                let src = format!("/tmp/{}", name);
                let _ = std::fs::copy(&src, &path);
                let _ = std::fs::remove_file(&src);
                break;
            }
        }
    }

    if !Path::new(&path).exists() {
        let _ = Command::new("spectacle")
            .args(["-o", &path, "-f", "-n", "-b"])
            .output();
    }
}
