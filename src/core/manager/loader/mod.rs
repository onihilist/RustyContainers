use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use crate::colorama::Colored;
use crate::{rc_error, rc_info, rc_success, log_message};

pub enum RCAction {
    STOP,
    BUILD,
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
            RCAction::BUILD => "build",
            RCAction::START => "start",
            RCAction::PAUSE => "pause",
            RCAction::RESUME => "resume",
            RCAction::DISCARD => "rm",
            RCAction::PRUNE_ALL => "system prune -a"
        }
    }
}

pub fn call_cmd(container_id_or_name: &str, action: RCAction) -> io::Result<()> {
    let cmd_bytes: &[u8] = include_bytes!("../scripts/container.cmd");

    let temp_dir = env::temp_dir();
    let temp_file_path = temp_dir.join("temp_script.cmd");

    {
        let mut temp_file = File::create(&temp_file_path)?;
        temp_file.write_all(cmd_bytes)?;
    }

    rc_info!(format!("Executing command: docker {} {}", action.as_str(), container_id_or_name));

    let output = Command::new("cmd")
        .arg("/C")
        .arg(&temp_file_path)
        .arg(action.as_str())
        .arg(container_id_or_name)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                rc_success!("Command executed successfully");
                rc_info!(format!("{}", String::from_utf8_lossy(&output.stdout)));
            } else {
                rc_error!(format!("Command failed with status : {}", output.status));
                rc_error!(format!("{}", String::from_utf8_lossy(&output.stderr)));
            }
        }
        Err(e) => {
            rc_error!(format!("Failed to execute command : {}", e));
        }
    }
    Ok(())
}
