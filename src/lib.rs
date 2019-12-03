mod logic;

use std::ffi::CStr;
use libc::{c_char, c_int};

#[no_mangle]
pub extern "C" fn sum(a: c_int, b: c_int) -> c_int {
    logic::sum(a as u32, b as u32) as i32
}

#[no_mangle]
pub extern "C" fn count(word: *const c_char) -> c_int {
    let cstr = unsafe { CStr::from_ptr(word) };
    // TODO: never panic in ffi calls
    let words = cstr.to_str().unwrap();
    logic::count(words) as c_int
}
