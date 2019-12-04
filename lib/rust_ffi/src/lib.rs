use rust_binding as src;

use std::ffi::{CStr, CString};
use libc::{c_char};

// Note that for python we need to use c_int rather than i32/i64
#[no_mangle]
pub extern "C" fn foo_new(count: i32) -> src::Foo {
    src::new_foo(count)
}

#[no_mangle]
pub extern "C" fn foo_multiply(foo: src::Foo, val: i32) -> i32 {
    foo.multiply(val)
}

#[no_mangle]
pub extern "C" fn foo_update(ptr: Option<&mut src::Foo>, val: i32) {
    match ptr {
        Some(foo) => foo.update(val),
        None => {},
    };
}

#[no_mangle]
pub extern "C" fn sum(a: u32, b: u32) -> u32 {
    src::sum(a, b)
}

#[no_mangle]
pub extern "C" fn count(word: *const c_char) -> usize {
    let cstr = unsafe { CStr::from_ptr(word) };
    // TODO: never panic in ffi calls
    let words = cstr.to_str().unwrap();
    src::count(words)
}

// TODO: note this creates a memory leak, we need a cleanup function
// how to do this without putting the burden on the other side (ideally auto-freed via swagger)?
#[no_mangle]
pub extern "C" fn concat(a: *const c_char, b: *const c_char) -> *mut c_char {
    // TODO: never panic in ffi calls
    let a = unsafe { CStr::from_ptr(a) }.to_str().unwrap();
    let b = unsafe { CStr::from_ptr(b) }.to_str().unwrap();
    let res = src::concat(a, b);
    CString::new(res).unwrap().into_raw()
}
