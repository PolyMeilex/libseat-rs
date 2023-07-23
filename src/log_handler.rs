use log::{debug, error, info, trace};
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
pub struct LogHandler;

impl LogHandler {
    pub fn new() -> Self {
        crate::set_log_level(LogLevel::Debug);

        unsafe { init_preformated_log_handler(ffi_handler, std::ptr::null()) };

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

impl Drop for LogHandler {
    fn drop(&mut self) {
        unsafe { drop_preformated_log_handler() }
    }
}

extern "C" fn ffi_handler(level: LogLevel, msg: *const c_char, _data: *const c_void) {
    let cstr = unsafe { CStr::from_ptr(msg) };
    match cstr.to_str() {
        Ok(msg) => LogHandler::log(level, msg),
        Err(err) => error!("{:?}", err),
    }
}
