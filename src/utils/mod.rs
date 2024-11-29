
use colorama::Colored;
use std::env;

macro_rules! rc_debug_mode {
    ($status:expr) => {
        if $status == true && cfg!(debug_assertions) {
            env::set_var("RC_DEBUG_MODE", "true", true);
            match env::var("RC_DEBUG_MODE") {
                Ok(val) => println!(format!("[", "DEBUG".to_string().color("magenta"), "] - Debug mode is activated!"));,
                Err(e) => println!("Couldn't read RC_DEBUG_MODE: {}", e),
            }
        }
    };
}

macro_rules! rc_debug {
    ($msg:expr) => {
        match env::var("RC_DEBUG_MODE") {
            Ok(val) => {
                if val == "true" {
                    println!(format!("[", "DEBUG".to_string().color("magenta"), "] - ", $msg));
                } else {
                    rc_error!("You should set RC_DEBUG_MODE environment variable on \"true\" to use debug messages !");
                }
            },
            Err(e) => rc_warning!("Couldn't read RC_DEBUG_MODE"),
        }
    };
}

macro_rules! rc_info {
    ($msg:expr) => {
        println!(format!("[", "INFO".to_string().color("cyan"), "] - ", $msg));
    };
}

macro_rules! rc_error {
    ($msg:expr) => {
        println!(format!("[", "ERROR".to_string().color("red"), "] - ", $msg));
    };
}

macro_rules! rc_fatal {
    ($msg:expr) => {
        println!(format!("[", "FATAL".to_string().color("bright red").style("bold"), "] - ", $msg));
    };
}

macro_rules! rc_warning {
    ($msg:expr) => {
        println!(format!("[", "WARNING".to_string().color("yellow"), "] - ", $msg));
    };
}

