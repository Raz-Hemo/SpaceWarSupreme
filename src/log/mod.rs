// This module implements a circular log. After X lines are written, 
// new lines overwrite the oldest ones.
// Usage is "use crate::log" to include and "logger().info("asd") " to write lines
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

pub struct CircularLogIter<'a> {
    _logger: &'a CircularLog,
    iteration_count: usize,
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

impl<'a> IntoIterator for &'a CircularLog {
    type Item = &'a CircularLogLine;
    type IntoIter = CircularLogIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CircularLogIter {
            _logger: self, 
            iteration_count: 0
        }
    }
}

impl<'a> Iterator for CircularLogIter<'a> {
    type Item = &'a CircularLogLine;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration_count == self._logger.lines.len() {
            None
        }
        else {
            self.iteration_count += 1;
            Some(&self._logger.lines[
                (self.iteration_count + self._logger.current_pos - 1) % self._logger.lines.len()
                ])
        }
    }
}

lazy_static! {
    pub static ref LOGGER: Mutex<CircularLog> = {
        let result = Mutex::from(CircularLog::new());

        std::panic::set_hook(Box::new(|panic_info| {
            let (filename, line) =
            panic_info.location().map(|loc| (loc.file(), loc.line()))
                .unwrap_or(("<unknown>", 0));

            let cause = panic_info.payload().downcast_ref::<String>().map(String::deref);
            let cause = cause.unwrap_or_else(||
                panic_info.payload().downcast_ref::<&str>().map(|s| *s)
                    .unwrap_or("<cause unknown>")
            );

            // Attempt to acquire logger
            let locked_logger = LOGGER.lock();
            if locked_logger.is_err() {
                return;
            }
            let mut locked_logger = locked_logger.unwrap();

            // Write panic
            locked_logger.error(&format!("A panic occurred at {}:{}: {}", filename, line, cause)[..]);

            // Open a crash report file
            use std::io::{Write};
            let f = std::fs::File::create(format!("./crash_reports/{}", 
                                                  Local::now().format("%Y-%m-%d %H-%M-%S.txt")
            ));
            if f.is_err() {
                return;
            }
            let mut f = std::io::BufWriter::new(f.unwrap());

            // Write the logs in order. TODO should implement iteration on circular logger
            // directly because this is bugged rn
            for logline in locked_logger.into_iter() {
                if writeln!(f, "{}", &logline.line).is_err() {
                    continue;
                }
            }
            
            // Best effort flush
            if f.flush().is_err() {
                return;
            }
        }));

        result
    };
}

pub fn logger() -> std::sync::MutexGuard<'static, CircularLog> {
    LOGGER.lock().expect("Logger object is poisoned")
}
