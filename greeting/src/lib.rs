extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greeting(name: String) -> String {
    return format!("Hello, {}", name);
}
