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

// Here we use custom wasm type
// Other option is to include #[wasm_bindgen] next to #[repr(C)] in the upstream lib
// Let's try it out

#[wasm_bindgen]
#[derive(Copy,Clone)]
pub struct Foo {
    pub count: i32,
}

impl Foo {
    fn from_rust(foo: rust_binding::Foo) -> Self {
        Foo{count: foo.count}
    }

    fn to_rust(&self) -> rust_binding::Foo {
        rust_binding::Foo{count: self.count}
    }
}

#[wasm_bindgen]
pub fn foo_new(count: i32) -> Foo {
    Foo::from_rust(rust_binding::new_foo(count))
}

#[wasm_bindgen]
// note that we need to pass in &Foo as derive(Copy) doesn't help
// not setting it causes foo to be freed
pub fn foo_multiply(foo: &Foo, val: i32) -> i32 {
    foo.to_rust().multiply(val)
}

// also note that we cannot accept &mut args, thus update is useless (uncomment and see compiler error)
//#[wasm_bindgen]
//pub fn foo_update(ptr: Option<&mut Foo>, val: i32) {
//    match ptr {
//        Some(foo) => foo.to_rust().update(val),
//        None => {},
//    };
//}

// work-around consuming one object and returning a new one...
// here we don't need that, but assume there is another mutable operation
#[wasm_bindgen]
pub fn foo_update(old: Foo, val: i32) -> Foo {
    let mut out = old.to_rust();
    out.update(val);
    Foo::from_rust(out)
}
