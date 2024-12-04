use std::io::Write;

use file_rotate::{compression::Compression, suffix::{AppendTimestamp, FileLimit}, ContentLimit, FileRotate};
use colored::Colorize;

use crate::config;

pub struct AppLogger {
    _rotate: FileRotate<AppendTimestamp>,
}

impl AppLogger {
    pub fn new(log_cfg: config::config::ConfigLoggerStruct) -> Self {
        let file_name_daily_timestamp = chrono::Local::now().format(&log_cfg.format).to_string();

        let log = FileRotate::new(
            format!("{}/{}.log", log_cfg.path, file_name_daily_timestamp),
            AppendTimestamp::default(FileLimit::MaxFiles(10)),
            ContentLimit::Bytes(1024 * 1024 * 10),
            Compression::OnRotate(5),
            None,
        );

        Self { _rotate: log }
    }

    pub fn info(&mut self, msg: String) -> &Self {
        let text = format!("{}", self.generate_text("INFO".to_string(), msg));
        
        writeln!(self._rotate, "{}", text).unwrap();
        println!("{}", text);

        drop(text);

        self
    }

    pub fn error(&mut self, msg: String) -> &Self {
        let text = format!("{}", self.generate_text("ERROR".to_string(), msg));
        
        writeln!(self._rotate, "{}", text).unwrap();
        eprintln!("{}", text);

        drop(text);

        self
    }

    pub fn warn(&mut self, msg: String) -> &Self {
        let text = format!("{}", self.generate_text("WARN".to_string(), msg));
        
        writeln!(self._rotate, "{}", text).unwrap();
        println!("{}", text);

        drop(text);

        self
    }

    fn generate_text(&self, level: String, msg: String) -> String {
        let now = chrono::Local::now();
        let now_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let level_str = match level.as_str() {
            "INFO" => level.green(),
            "ERROR" => level.red(),
            "WARN" => level.yellow(),
            _ => level.normal(),
        };

        format!("[{}] [{}] {}", now_str, level_str, msg)
    }
}