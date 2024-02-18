use alloc::ffi::CString;
use alloc::string::String;
use core::ffi::{c_char, c_int};

pub enum DbgLevel {
    Dead,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
    KDebug,
}

pub fn dbglog(level: DbgLevel, string: String) {
    let c_str = CString::new(string).unwrap();
    unsafe {
        extern "C" {
            fn dbglog(level: c_int, string: *const c_char);
        }

        dbglog(level as c_int, c_str.as_ptr() as *const i8);
    }
}

#[macro_export]
macro_rules! dbglog {
    ($level:expr, $($arg:expr),+) => {
        {
            $crate::dbglog::dbglog($level, alloc::format!($($arg),+));
        }
    };
}

pub fn dbglog_set_level(level: DbgLevel) {
    unsafe {
        extern "C" {
            fn dbglog_set_level(level: c_int);
        }

        dbglog_set_level(level as c_int);
    }
}
