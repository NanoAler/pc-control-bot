use std::process::Command;

pub async fn launch(app: &str) {
    let _ = Command::new("gtk-launch")
        .arg(app)
        .output()
        .or_else(|_| {
            Command::new("nohup")
                .arg(app)
                .arg("&")
                .output()
        });
}
