use std::process::Command;

pub fn open(editor_cmd: &str, path: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", editor_cmd, path))
        .status()
        .expect("failed to open file");
}
