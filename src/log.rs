use libseat_sys::{libseat_log_level, libseat_set_log_level};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Silent,
    Error,
    Info,
    Debug,
    Last,
}

impl From<LogLevel> for libseat_log_level {
    fn from(v: LogLevel) -> libseat_log_level {
        use libseat_log_level::*;
        match v {
            LogLevel::Silent => LIBSEAT_LOG_LEVEL_SILENT,
            LogLevel::Error => LIBSEAT_LOG_LEVEL_ERROR,
            LogLevel::Info => LIBSEAT_LOG_LEVEL_INFO,
            LogLevel::Debug => LIBSEAT_LOG_LEVEL_DEBUG,
            LogLevel::Last => LIBSEAT_LOG_LEVEL_LAST,
        }
    }
}

pub fn set_log_level(level: LogLevel) {
    unsafe { libseat_set_log_level(level.into()) }
}
