mod logic;

use std::ffi::{CStr, CString};
use libc::{c_char, c_int};

// Note that for python we need to use c_int rather than i32/i64
#[no_mangle]
pub extern "C" fn foo_new(count: c_int) -> logic::Foo {
    logic::new_foo(count as i32)
}

#[no_mangle]
pub extern "C" fn foo_multiply(foo: logic::Foo, val: c_int) -> c_int {
    foo.multiply(val as i32) as c_int
}

#[no_mangle]
pub extern "C" fn foo_update(ptr: Option<&mut logic::Foo>, val: c_int) {
    match ptr {
        Some(foo) => foo.update(val as i32),
        None => {},
    };
}

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

// TODO: note this creates a memory leak, we need a cleanup function
// how to do this without putting the burden on the other side (ideally auto-freed via swagger)?
#[no_mangle]
pub extern "C" fn concat(a: *const c_char, b: *const c_char) -> *mut c_char {
    // TODO: never panic in ffi calls
    let a = unsafe { CStr::from_ptr(a) }.to_str().unwrap();
    let b = unsafe { CStr::from_ptr(b) }.to_str().unwrap();
    let res = logic::concat(a, b);
    CString::new(res).unwrap().into_raw()
}
