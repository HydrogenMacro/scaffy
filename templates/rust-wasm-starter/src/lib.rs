pub mod ffi;

use log::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();

    info!("Hello from Rust!");

    ffi::example();
}
