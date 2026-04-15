use std::process::Command;
use crate::i18n::{Lang, T};

pub async fn lock_screen() {
    let _ = Command::new("loginctl")
        .args(["lock-session"])
        .output();
    let _ = Command::new("xdg-screensaver")
        .arg("lock")
        .output();
}

pub async fn exec_command(cmd: &str, lang: Lang) -> String {
    match Command::new("sh")
        .args(["-c", cmd])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stdout.is_empty() && stderr.is_empty() {
                T::get("done_no_output", lang).to_string()
            } else if stdout.is_empty() {
                stderr.to_string()
            } else {
                stdout.to_string()
            }
        }
        Err(e) => format!("Error: {}", e),
    }
}

pub async fn shutdown() {
    let _ = Command::new("systemctl")
        .args(["poweroff"])
        .output();
}

pub async fn reboot() {
    let _ = Command::new("systemctl")
        .args(["reboot"])
        .output();
}

pub async fn sleep() {
    let _ = Command::new("systemctl")
        .args(["suspend"])
        .output();
}

pub async fn list_processes() -> Result<String, String> {
    let output = Command::new("ps")
        .args(["aux", "--sort=-rss"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().take(25).collect();

    Ok(lines.join("\n"))
}
