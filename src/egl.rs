#![cfg(target_os = "android")]

use std::ffi::CStr;
use std::ffi::CString;
use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};

pub fn get_proc_address(proc_name: &str) -> *const c_void {
    unsafe {
        let string = CString::new(proc_name).unwrap();

        ffi::eglGetProcAddress(string.as_ptr())
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------
mod ffi {
    use libc::{c_char, c_int, c_short, c_uchar, c_uint, c_ushort, c_void};
    extern {
        pub fn eglGetProcAddress(proc_name: *const c_char) -> *const c_void;
    }
}

struct HH {
    ActiveTePtr: *const c_void,
}

fn missin() -> ! {
    panic!("")
}

impl HH {
    pub fn new() -> HH {
        HH {
            ActiveTePtr: 0 as *const c_void,
        }
    }
}



fn he() {
    let xx = HH{ActiveTePtr: get_proc_address("glActiveTexture") };
}