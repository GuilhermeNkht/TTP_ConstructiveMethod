use chrono::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logger(log_file: &str) {
    // Abre o arquivo de log em modo append
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file)
        .unwrap();

    Builder::new()
        .format(move |_, record| {
            let timestamp = Local::now().format("%H:%M:%S");
            let line = format!("[{}][{}] {}\n", timestamp, record.level(), record.args());

            print!("{}", line);
            std::io::stdout().flush().unwrap();

            let mut file = &file;
            file.write_all(line.as_bytes()).unwrap();

            Ok(())
        })
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();
}
