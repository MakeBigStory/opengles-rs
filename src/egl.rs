#![cfg(target_os = "android")]

pub fn get_proc_address(proc_name: &str) -> extern "C" fn() {
    unsafe {
        let string = CString::new(proc_name).unwrap();

        ffi::eglGetProcAddress(string.as_ptr())
    }
}

// -------------------------------------------------------------------------------------------------
// FFI
// -------------------------------------------------------------------------------------------------
mod ffi {
    extern {
        pub fn eglGetProcAddress(proc_name: *const c_char) -> extern "C" fn();
    }
}