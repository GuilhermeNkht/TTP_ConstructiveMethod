use chrono::Local;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::fs::OpenOptions;
use std::io::Write;

#[cfg(debug_assertions)]
pub const LOGS_ENABLED: bool = true;

#[cfg(not(debug_assertions))]
pub const LOGS_ENABLED: bool = false;

/// Initializes the logger to write messages to console and the file.
///
/// The logger prints messages with a timestamp and log level (info!).
/// Logging can be globally enabled or disabled using the `LOGS_ENABLED` flag
/// in this class.
///
/// # Arguments
/// * `log_file` - A string representing the path of the file where logs will be saved.
///
/// # Panics
/// This function will panic if the log file cannot be created or written.
///
/// # Example
/// ```
/// // Initialize logger before generating solutions
/// init_logger("experiment.log");
/// info!("Logger initialized!");
/// ```
pub fn init_logger(log_file: &str) {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .unwrap();

    Builder::new()
        .format(move |buf, record| {
            let timestamp = Local::now().format("%H:%M:%S");
            let line = format!("[{}][{}] {}\n", timestamp, record.level(), record.args());

            if LOGS_ENABLED {
                print!("{}", line);
            }

            let mut f = &file;
            f.write_all(line.as_bytes()).unwrap();
            Ok(())
        })
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();
}