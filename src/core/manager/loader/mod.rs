use std::process::Command;

pub enum RCAction {
    STOP,
    START,
    PAUSE,
    RESUME,
    DISCARD,
    PRUNE_ALL
}

impl RCAction {
    pub fn as_str(&self) -> &str {
        match self {
            RCAction::STOP => "stop",
            RCAction::START => "start",
            RCAction::PAUSE => "pause",
            RCAction::RESUME => "resume",
            RCAction::DISCARD => "prune",
            RCAction::PRUNE_ALL => "prune"
        }
    }
}

pub fn call_cmd(container_id_or_name: &str, action: RCAction) {
    let output = Command::new("cmd")
        .arg("/C")
        .arg("../scripts/container.cmd")
        .arg(action.as_str())
        .arg(container_id_or_name)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Command : docker {} {}", action.as_str(), container_id_or_name);
        //println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Command failed to execute : docker {} {}", action.as_str(), container_id_or_name);
    }
}
