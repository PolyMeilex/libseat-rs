use libseat_sys as sys;

use std::ptr::NonNull;

use sys::{
    libseat_log_level_LIBSEAT_LOG_LEVEL_DEBUG, libseat_log_level_LIBSEAT_LOG_LEVEL_ERROR,
    libseat_log_level_LIBSEAT_LOG_LEVEL_INFO, libseat_log_level_LIBSEAT_LOG_LEVEL_LAST,
    libseat_log_level_LIBSEAT_LOG_LEVEL_SILENT,
};

#[repr(u32)]
pub enum LogLevel {
    Silent = libseat_log_level_LIBSEAT_LOG_LEVEL_SILENT,
    Error = libseat_log_level_LIBSEAT_LOG_LEVEL_ERROR,
    Info = libseat_log_level_LIBSEAT_LOG_LEVEL_INFO,
    Debug = libseat_log_level_LIBSEAT_LOG_LEVEL_DEBUG,
    Last = libseat_log_level_LIBSEAT_LOG_LEVEL_LAST,
}

pub fn set_log_level(level: LogLevel) {
    unsafe {
        sys::libseat_set_log_level(level as u32);
    }
}

// TODO: This guard is stupid, callback should live as long as seat is alive
pub struct TemporaryFreeGuard(*mut SeatListenerUserData, *mut sys::libseat_seat_listener);

impl Drop for TemporaryFreeGuard {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.0);
            Box::from_raw(self.1);
        }
    }
}

type SeatClosure = dyn FnMut(&mut Seat);

struct SeatListenerUserData {
    enable_seat: Box<SeatClosure>,
    disable_seat: Box<SeatClosure>,
}

extern "C" fn enable_seat(seat: *mut sys::libseat, data: *mut std::os::raw::c_void) {
    let data = data as *mut SeatListenerUserData;
    let data = unsafe { &mut *data };

    let mut seat = unsafe { Seat(NonNull::new_unchecked(seat)) };
    (data.enable_seat)(&mut seat);
}

extern "C" fn disable_seat(seat: *mut sys::libseat, data: *mut std::os::raw::c_void) {
    let data = data as *mut SeatListenerUserData;
    let data = unsafe { &mut *data };

    let mut seat = unsafe { Seat(NonNull::new_unchecked(seat)) };
    (data.disable_seat)(&mut seat);
}

pub struct Seat(NonNull<sys::libseat>);

impl Seat {
    pub fn open<E, D>(enable: E, disable: D) -> (Option<Self>, TemporaryFreeGuard)
    where
        E: FnMut(&mut Self) + 'static,
        D: FnMut(&mut Self) + 'static,
    {
        let listener = sys::libseat_seat_listener {
            enable_seat: Some(enable_seat),
            disable_seat: Some(disable_seat),
        };
        let listener = Box::into_raw(Box::new(listener));

        let user_data = SeatListenerUserData {
            enable_seat: Box::new(enable),
            disable_seat: Box::new(disable),
        };

        let user_data = Box::into_raw(Box::new(user_data));

        let seat = unsafe { sys::libseat_open_seat(listener, user_data as *mut _) };

        let s = NonNull::new(seat).map(Self);

        (s, TemporaryFreeGuard(user_data, listener))
    }
}

impl Seat {
    pub unsafe fn name(&mut self) -> &str {
        let cstr = sys::libseat_seat_name(self.0.as_mut());
        let cstr = std::ffi::CStr::from_ptr(cstr as *const _);
        cstr.to_str().unwrap()
    }

    pub unsafe fn dispatch(&mut self, timeout: i32) {
        sys::libseat_dispatch(self.0.as_mut(), timeout);
    }

    pub unsafe fn disable(&mut self) {
        sys::libseat_disable_seat(self.0.as_mut());
    }

    pub unsafe fn close(mut self) {
        sys::libseat_close_seat(self.0.as_mut());
    }

    pub unsafe fn close_device(&mut self, device_id: i32) {
        sys::libseat_close_device(self.0.as_mut(), device_id);
    }
}
