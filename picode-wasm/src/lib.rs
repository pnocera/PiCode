//! PiCode WASM - WebAssembly bindings

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PiCodeWasm;

#[wasm_bindgen]
impl PiCodeWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PiCodeWasm {
        PiCodeWasm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _wasm = PiCodeWasm::new();
    }
}