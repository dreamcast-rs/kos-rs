// Rust for KallistiOS/Dreamcast
// Copyright (C) 2024 Eric Fradella
// https://dreamcast.rs/

use kos_sys::prelude::*;
use std::{
	ffi::CString,
	string::String,
};

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
        kos_sys::os::dbglog::dbglog(level as c_int, c_str.as_ptr() as *const i8);
    }
}

#[macro_export]
macro_rules! dbglog {
    ($level:expr, $($arg:expr),+) => {
        {
            $crate::dbglog::dbglog($level, format!($($arg),+));
        }
    };
}

pub fn dbglog_set_level(level: DbgLevel) {
    unsafe {
        kos_sys::os::dbglog::dbglog_set_level(level as c_int);
    }
}
