mod utils;
mod convert;
mod read_mappings;
mod data;
mod functions;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(input: &str) {
    let foo = &data::HASH_MAP;
    let source = foo.get("devanagari").unwrap();
    let destination = foo.get("slp1").unwrap();
    alert(&format!("{}", convert::convert_indic_to_roman(&input.to_string(), source, destination)));
}
