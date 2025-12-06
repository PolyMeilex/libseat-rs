use log::{debug, error, info, trace};
use std::{ffi::CStr, os::raw::c_char, sync::LazyLock};

use crate::LogLevel;

type LogHandlerFn = unsafe extern "C" fn(level: LogLevel, msg: *const c_char);

extern "C" {
    fn init_preformatted_log_handler(handler: LogHandlerFn);
}

/// Custom LibSeat log handler
#[derive(Debug)]
struct LogHandler;

impl LogHandler {
    fn new() -> Self {
        crate::set_log_level(LogLevel::Debug);

        unsafe { init_preformatted_log_handler(ffi_handler) };

        Self
    }

    fn log(level: LogLevel, msg: &str) {
        match level {
            LogLevel::Silent => trace!("{}", msg),
            LogLevel::Error => error!("{}", msg),
            LogLevel::Info => info!("{}", msg),
            LogLevel::Debug | LogLevel::Last => debug!("{}", msg),
        }
    }
}

extern "C" fn ffi_handler(level: LogLevel, msg: *const c_char) {
    let cstr = unsafe { CStr::from_ptr(msg) };
    match cstr.to_str() {
        Ok(msg) => LogHandler::log(level, msg),
        Err(err) => error!("{:?}", err),
    }
}

static LOG_HANDLER: LazyLock<LogHandler> = LazyLock::new(LogHandler::new);

/// Initialise libseat log handler
pub fn init() {
    let _ = &*LOG_HANDLER;
}
