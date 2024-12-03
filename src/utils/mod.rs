
#[macro_export]
macro_rules! log_message {
    ($fmt:expr $(, $arg:expr)*) => {{
        use std::fs::{self, OpenOptions};
        use std::path::Path;
        use std::io::Write;

        let now = chrono::Local::now();
        let datetime = now.format("[%Y-%m-%d %H:%M:%S]").to_string();

        let log_dir = Path::new("logs");
        if !log_dir.exists() {
            fs::create_dir(log_dir).expect("Failed to create logs directory");
        }

        let filepath = log_dir.join("logfile.log");

        let message = format!($fmt $(, $arg)*);

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filepath)
            .expect("Failed to open log file");

        writeln!(file, "{}{}", datetime, message).expect("Failed to write to log file");
    }};
}

#[macro_export]
macro_rules! rc_debug_mode {
    ($status:expr) => {
        use std::env;
        if $status == true && cfg!(debug_assertions) {
            env::set_var("RC_DEBUG_MODE", "true");
            match env::var("RC_DEBUG_MODE") {
                Ok(val) => {
                    println!(
                        "[{}] - Debug mode is activated !",
                        "DEBUG".to_string().color("magenta")
                    );
                    log_message!("[DEBUG] - Debug mode is activated !");
                }
                Err(e) => {
                    println!("Couldn't read RC_DEBUG_MODE: {}", e);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! rc_debug {
    ($msg:expr) => {
        use std::env;
        match env::var("RC_DEBUG_MODE") {
            Ok(val) => {
                if val == "true" {
                    println!(
                        "[{}] - {}",
                        "DEBUG".to_string().color("magenta"),
                        $msg
                    );
                    log_message!(format!("[DEBUG] - {}", $msg));
                } else {
                    rc_error!("You should set RC_DEBUG_MODE environment variable on \"true\" to use debug messages !");
                }
            },
            Err(e) => rc_warning!("Couldn't read RC_DEBUG_MODE"),
        }
    };
}

#[macro_export]
macro_rules! rc_success {
    ($msg:expr) => {
        println!(
            "[{}] - {}",
            "SUCCESS".to_string().color("green"),
            $msg
        );
        log_message!("{}", format!("[SUCCESS] - {}", $msg));
    };
}


#[macro_export]
macro_rules! rc_info {
    ($msg:expr) => {
        println!(
            "[{}] - {}",
            "INFO".to_string().color("cyan"),
            $msg
        );
        log_message!("{}", format!("[INFO] - {}", $msg));
    };
}

#[macro_export]
macro_rules! rc_error {
    ($msg:expr) => {
        println!(
            "[{}] - {}",
            "ERROR".to_string().color("red"),
            $msg
        );
        log_message!("{}", format!("[ERROR] - {}", $msg));
    };
}

#[macro_export]
macro_rules! rc_fatal {
    ($msg:expr) => {
        println!(
            "[{}] - {}",
            "FATAL".to_string().color("bright red"),
            $msg
        );
        log_message!("{}", format!("[FATAL] - {}", $msg));
    };
}

#[macro_export]
macro_rules! rc_warning {
    ($msg:expr) => {
        println!(
            "[{}] - {}",
            "WARNING".to_string().color("yellow"),
            $msg
        );
        log_message!("{}", format!("[WARNING] - {}", $msg));
    };
}

