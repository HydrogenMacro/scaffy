use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../demo/example.js")]
extern "C" {
    pub fn example();
}