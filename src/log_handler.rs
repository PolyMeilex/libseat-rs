use slog::{debug, error, info, trace};
use std::{
    ffi::CStr,
    os::raw::{c_char, c_void},
};

use crate::LogLevel;

pub type LogHandlerFn =
    unsafe extern "C" fn(level: LogLevel, msg: *const c_char, data: *const c_void);

extern "C" {
    pub fn init_preformated_log_handler(handler: LogHandlerFn, data: *const c_void);
    pub fn drop_preformated_log_handler();
}

/// Custom LibSeat log handler
#[derive(Debug)]
pub struct LogHandler(Box<slog::Logger>);

impl LogHandler {
    pub fn new(logger: slog::Logger) -> Self {
        crate::set_log_level(LogLevel::Debug);
        // From now one slog is responsible for filtering log levels

        let logger = Box::new(logger);

        unsafe {
            init_preformated_log_handler(ffi_handler, logger.as_ref() as *const _ as *const _)
        };
        Self(logger)
    }

    fn log(logger: &slog::Logger, level: LogLevel, msg: &str) {
        match LogLevel::from(level) {
            LogLevel::Silent => trace!(logger, "{}", msg),
            LogLevel::Error => error!(logger, "{}", msg),
            LogLevel::Info => info!(logger, "{}", msg),
            LogLevel::Debug | LogLevel::Last => debug!(logger, "{}", msg),
        }
    }
}

impl Drop for LogHandler {
    fn drop(&mut self) {
        unsafe { drop_preformated_log_handler() }
    }
}

extern "C" fn ffi_handler(level: LogLevel, msg: *const c_char, data: *const c_void) {
    let logger: &slog::Logger = unsafe { &*(data as *const _) };

    let cstr = unsafe { CStr::from_ptr(msg) };
    match cstr.to_str() {
        Ok(msg) => LogHandler::log(&logger, level, msg),
        Err(err) => error!(logger, "{:?}", err),
    }
}
