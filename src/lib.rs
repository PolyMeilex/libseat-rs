use libseat_sys as sys;

use std::{
    ffi::CString,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    os::unix::io::RawFd,
    path::Path,
    ptr::NonNull,
};

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

type SeatClosure = dyn FnMut(&mut SeatRef);

struct SeatListenerUserData {
    enable_seat: Box<SeatClosure>,
    disable_seat: Box<SeatClosure>,
}

extern "C" fn enable_seat(seat: *mut sys::libseat, data: *mut std::os::raw::c_void) {
    let data = data as *mut SeatListenerUserData;
    let data = unsafe { &mut *data };

    let mut seat = unsafe { SeatRef(NonNull::new_unchecked(seat)) };
    (data.enable_seat)(&mut seat);
}

extern "C" fn disable_seat(seat: *mut sys::libseat, data: *mut std::os::raw::c_void) {
    let data = data as *mut SeatListenerUserData;
    let data = unsafe { &mut *data };

    let mut seat = unsafe { SeatRef(NonNull::new_unchecked(seat)) };
    (data.disable_seat)(&mut seat);
}

pub struct Seat {
    inner: SeatRef,
    _ffi_listener: Box<sys::libseat_seat_listener>,
    _user_listener: Box<SeatListenerUserData>,
}

impl Seat {
    pub fn open<E, D>(enable: E, disable: D) -> Option<Self>
    where
        E: FnMut(&mut SeatRef) + 'static,
        D: FnMut(&mut SeatRef) + 'static,
    {
        let listener = sys::libseat_seat_listener {
            enable_seat: Some(enable_seat),
            disable_seat: Some(disable_seat),
        };
        let mut listener = Box::new(listener);

        let user_data = SeatListenerUserData {
            enable_seat: Box::new(enable),
            disable_seat: Box::new(disable),
        };
        let mut user_data = Box::new(user_data);

        let seat =
            unsafe { sys::libseat_open_seat(&mut *listener, &mut *user_data as *mut _ as *mut _) };

        NonNull::new(seat).map(|nn| Self {
            inner: SeatRef(nn),
            _ffi_listener: listener,
            _user_listener: user_data,
        })
    }
}

impl Drop for Seat {
    fn drop(&mut self) {
        unsafe { sys::libseat_close_seat(self.0.as_mut()) };
    }
}

impl Deref for Seat {
    type Target = SeatRef;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Seat {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct SeatRef(NonNull<sys::libseat>);

impl SeatRef {
    pub fn name(&mut self) -> &str {
        unsafe {
            let cstr = sys::libseat_seat_name(self.0.as_mut());
            let cstr = std::ffi::CStr::from_ptr(cstr as *const _);
            cstr.to_str().unwrap()
        }
    }

    pub fn dispatch(&mut self, timeout: i32) {
        unsafe { sys::libseat_dispatch(self.0.as_mut(), timeout) };
    }

    pub fn disable(&mut self) {
        unsafe { sys::libseat_disable_seat(self.0.as_mut()) };
    }

    pub fn open_device<P: AsRef<Path>>(&mut self, path: &P) -> Option<(i32, RawFd)> {
        let path = path.as_ref();
        let string = path.as_os_str().to_str().unwrap();
        let cstring = CString::new(string).unwrap();

        let mut fd = MaybeUninit::uninit();
        let dev_id =
            unsafe { sys::libseat_open_device(self.0.as_mut(), cstring.as_ptr(), fd.as_mut_ptr()) };

        if dev_id != -1 {
            let fd = unsafe { fd.assume_init() };
            Some((dev_id, fd))
        } else {
            None
        }
    }

    pub fn close_device(&mut self, device_id: i32) {
        unsafe { sys::libseat_close_device(self.0.as_mut(), device_id) };
    }
}
