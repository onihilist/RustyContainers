use std::process::Command;

pub enum RCAction {
    STOP,
    START,
    PAUSE,
    RESUME,
    DISCARD
}

impl RCAction {
    pub fn as_str(&self) -> &str {
        match self {
            RCAction::STOP => "stop",
            RCAction::START => "start",
            RCAction::PAUSE => "pause",
            RCAction::RESUME => "resume",
            RCAction::DISCARD => "prune",
        }
    }
}

pub fn call_cmd(action: RCAction) {

    let output = Command::new("cmd")
        .arg("/C")
        .arg("../scripts/container.cmd")
        .arg(action.as_str())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Command executed successfully.");
        println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed to execute.");
        eprintln!("Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
