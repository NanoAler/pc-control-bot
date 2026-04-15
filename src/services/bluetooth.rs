use std::process::Command;

pub async fn toggle() {
    let status = get_bluetooth_status();
    let _ = Command::new("rfkill")
        .args(["block", "bluetooth"])
        .output();

    if !status {
        let _ = Command::new("rfkill")
            .args(["unblock", "bluetooth"])
            .output();
    }
}

fn get_bluetooth_status() -> bool {
    if let Ok(output) = Command::new("rfkill")
        .args(["list", "bluetooth"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.contains("Soft blocked: no");
    }
    false
}
