use std::process::Command;

pub async fn list() -> Result<String, String> {
    let mut result = String::from("=== USB Devices ===\n");

    if let Ok(output) = Command::new("lsusb").output() {
        result.push_str(&String::from_utf8_lossy(&output.stdout));
    }

    result.push_str("\n=== Input Devices ===\n");
    if let Ok(output) = Command::new("xinput").args(["list"]).output() {
        result.push_str(&String::from_utf8_lossy(&output.stdout));
    }

    Ok(result)
}
