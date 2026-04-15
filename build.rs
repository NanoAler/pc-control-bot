use std::process::Command;

fn main() {
    let src = "src/utils/screenshot.cpp";
    let out = "src/utils/screenshot_tool";

    println!("cargo:rerun-if-changed={}", src);

    let pkg_config = Command::new("pkg-config")
        .args(["--cflags", "--libs", "Qt6Core", "Qt6Gui", "Qt6Widgets"])
        .output();

    let mut cmd = Command::new("g++");
    cmd.args(["-fPIC", src, "-o", out]);

    if let Ok(pkg) = pkg_config {
        let args: Vec<String> = String::from_utf8_lossy(&pkg.stdout)
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        cmd.args(&args);
    }

    let result = cmd.output();

    match result {
        Ok(o) if o.status.success() => {
            println!("Built screenshot_tool");
        }
        Ok(o) => {
            eprintln!("screenshot.cpp failed:");
            eprintln!("{}", String::from_utf8_lossy(&o.stderr));
        }
        Err(e) => {
            eprintln!("g++ failed: {}", e);
        }
    }
}
