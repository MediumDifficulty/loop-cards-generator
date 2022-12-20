mod utils;
mod generator;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate(config: JsValue, seed: u64) -> JsValue {
    serde_wasm_bindgen::to_value(
        &generator::generate_set(&serde_wasm_bindgen::from_value(config).unwrap(), seed)
    ).unwrap()
}
