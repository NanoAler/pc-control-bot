use std::process::Command;

pub async fn capture() -> Option<String> {
    let path = format!("/tmp/camera_{}.jpg", chrono::Local::now().format("%Y%m%d_%H%M%S"));

    let result = Command::new("ffmpeg")
        .args([
            "-y",
            "-f", "v4l2",
            "-i", "/dev/video0",
            "-vframes", "1",
            "-update", "1",
            &path,
        ])
        .output();

    if result.is_ok() && std::path::Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}
