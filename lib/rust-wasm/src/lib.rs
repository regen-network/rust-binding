mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sum(a: u32, b: u32) -> u32{
    rust_binding::sum(a, b)
}

#[wasm_bindgen]
pub fn count(word: &str) -> usize{
    rust_binding::count(word)
}

#[wasm_bindgen]
pub fn concat(a: &str, b: &str) -> String {
    rust_binding::concat(a, b)
}
