use alloc::ffi::CString;
use alloc::string::String;
use core::ffi::c_char;

pub fn printf(string: String) {
    let c_str = CString::new(string).unwrap();

    unsafe {
        extern "C" {
            fn printf(string: *const c_char);
        }

        printf(c_str.as_ptr() as *const i8);
    }
}

/// For no_std projects, this macro is provided to mimic the print macro
/// provided by the Rust standard library.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            $crate::print::printf(alloc::format!($($arg)*));
        }
    };
}

/// For no_std projects, this macro is provided to mimic the println macro
/// provided by the Rust standard library.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            $crate::print::printf(alloc::format!($($arg)*) + "\n");
        }
    };
}
