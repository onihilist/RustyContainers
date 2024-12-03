use std::{env, io};
use std::fs::File;
use std::io::Write;
use std::process::Command;

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

    // Write the command bytes to a temporary file
    {
        let mut temp_file = File::create(&temp_file_path)?;
        temp_file.write_all(cmd_bytes)?;
    }

    // Construct the command to be executed
    let command_str = format!(
        "cmd /C {} {} {}",
        temp_file_path.display(),
        action.as_str(),
        container_id_or_name
    );

    // Print the command for debugging purposes
    println!("Executing command: {}", command_str);

    // Execute the command
    let output = Command::new("cmd")
        .arg("/C")
        .arg(&temp_file_path)
        .arg(action.as_str())
        .arg(container_id_or_name)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Command executed successfully");
                println!("Output: {}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("Command failed with status: {}", output.status);
                eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
    Ok(())
}
