use crate::core::settings::app_settings;
use colored::*; // Add `colored = "2"` in Cargo.toml
use std::fmt::Display;

// Simple enum for clarity
#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug = 10,
    Info = 20,
    Warn = 30,
    Error = 40,
    Critical = 50,
}

fn should_log(level: LogLevel) -> bool {
    let dev = app_settings().development_mode;
    match level {
        LogLevel::Debug => dev, // Only if dev mode
        _ => true,              // Always for others
    }
}

pub fn log_internal<L: Display>(level: LogLevel, label: &str, color: Color, message: L) {
    if should_log(level) {
        let label_colored = label.color(color).bold();
        println!("[{}] {}", label_colored, message);
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::core::settings::app_settings().development_mode {
            let location = format!("[{}:{}]", file!(), line!());
            $crate::core::logs::log_internal(
                $crate::core::logs::LogLevel::Debug,
                "DEBUG",
                colored::Color::Cyan,
                format!("{} {}", location, format_args!($($arg)*))
            );
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        let location = format!("[{}:{}]", file!(), line!());
        $crate::core::logs::log_internal(
            $crate::core::logs::LogLevel::Info,
            "INFO",
            colored::Color::Green,
            format!("{} {}", location, format_args!($($arg)*))
        );
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        let location = format!("[{}:{}]", file!(), line!());
        $crate::core::logs::log_internal(
            $crate::core::logs::LogLevel::Warn,
            "WARN",
            colored::Color::Yellow,
            format!("{} {}", location, format_args!($($arg)*))
        );
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        let location = format!("[{}:{}]", file!(), line!());
        $crate::core::logs::log_internal(
            $crate::core::logs::LogLevel::Error,
            "ERROR",
            colored::Color::Red,
            format!("{} {}", location, format_args!($($arg)*))
        );
    };
}

#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        let location = format!("[{}:{}]", file!(), line!());
        $crate::core::logs::log_internal(
            $crate::core::logs::LogLevel::Critical,
            "CRITICAL",
            colored::Color::BrightRed,
            format!("{} {}", location, format_args!($($arg)*))
        );
    };
}
