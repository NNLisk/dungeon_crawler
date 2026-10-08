use std::{fmt::Display, time::SystemTime};

// this makes logging a bit easier, separating log messages
// and you can limit which you see when running

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Fatal,
}

#[derive(Clone, Debug)]
pub struct Logger {
    min_level: LogLevel,
    tag: String,
} 

impl Logger {
    pub fn new(min_level: LogLevel, tag: &str) -> Self {
        let t = String::from(tag);
        Logger { min_level, tag: t }
    }

    pub fn info<T: Display>(&self, msg: T) {
        self.log(LogLevel::Info, "\x1b[32m", msg);
    }

    pub fn debug<T: Display>(&self, msg: T) {
        self.log(LogLevel::Debug, "\x1b[32m", msg);

    }

    pub fn fatal<T: Display>(&self, msg: T) {
        self.log(LogLevel::Fatal, "\x1b[32m", msg);
        std::process::exit(0)
    }

    pub fn log<T: Display>(&self, level: LogLevel, color_code: &str, msg: T) {

        if level < self.min_level {
            return;
        }

        // let now = SystemTime::now()
        //     .duration_since(SystemTime::UNIX_EPOCH)
        //     .unwrap_or_default()
        //     .as_secs();

        let line = format!(
            "[{}{}\x1b[0m] {}\n",
            color_code, self.tag, msg
        );

        print!("{}", line);
    }
}