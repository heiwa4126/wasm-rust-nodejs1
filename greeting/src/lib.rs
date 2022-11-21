extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greeting(name: String) -> String {
    return format!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_test() {
        assert_eq!(greeting(String::from("Alice")), "Hello, Alice!");
    }
}
