// This module implements a circular log. After X lines are written, 
// new lines overwrite the oldest ones.
// Usage is "mod log" to include and " logger!().info("asd") " to write lines
extern crate chrono;
use chrono::Local;
use std::sync::Mutex;
use lazy_static::lazy_static;

const MAX_LOG_LINES: usize = 1000;

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug)]
pub struct CircularLogLine
{
    line: String,
    level: LogLevel,
}

#[derive(Debug)]
pub struct CircularLog
{
    lines: Vec<CircularLogLine>,
    current_pos: usize,
}

impl CircularLog {
    pub fn new () -> CircularLog {
        if MAX_LOG_LINES == 0 {
            panic!("Can't have MAX_LOG_LINES be 0");
        }

        CircularLog {
            lines: vec![],
            current_pos: 0,
        }
    }

    pub fn write_log(&mut self, level: LogLevel, line: &str) {
        let log_level_str: &str = match level {
            LogLevel::Error => "ERROR",
            LogLevel::Warning => "WARNING",
            LogLevel::Info => "INFO",
        };

        let time_str: String =  Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();

        // Construct a line that looks like "[2020-01-18 16:16:32] ERROR - something happened"
        let log_line = CircularLogLine {
            line: format!("{} {} - {}", 
                          time_str, log_level_str, line),
            level: level,
        };

        if self.lines.len() < MAX_LOG_LINES {
            self.lines.push(log_line);
            self.current_pos = (self.current_pos + 1) % MAX_LOG_LINES;
        }
        else {
            self.lines[self.current_pos] = log_line;
            self.current_pos = (self.current_pos + 1) % MAX_LOG_LINES;
        }
    }
    pub fn error(&mut self, line: &str) {
        self.write_log(LogLevel::Error, line);
    }
    pub fn warning(&mut self, line: &str) {
        self.write_log(LogLevel::Warning, line);
    }
    pub fn info(&mut self, line: &str) {
        self.write_log(LogLevel::Info, line);
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<CircularLog> = Mutex::from(CircularLog::new());
}

#[macro_export]
macro_rules! logger {
    () => {
        log::LOGGER.lock().unwrap()
    };
}