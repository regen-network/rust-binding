mod logic;

use ::libc::c_int;

#[no_mangle]
pub extern "C" fn sum(a: c_int, b: c_int) -> c_int {
    logic::sum(a as u32, b as u32) as i32
}