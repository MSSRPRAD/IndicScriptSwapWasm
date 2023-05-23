mod utils;
pub mod convert;
pub mod data;
pub mod functions;
pub mod read_mappings;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 0 -> i2i
// 1 -> i2r
// 2 -> r2r
// 3 -> r2i

#[wasm_bindgen]
pub fn script_swap(input: &str, src: &str, dest: &str, conv: usize) -> JsValue {
    let foo = &data::HASH_MAP;
    let source = foo.get(src).unwrap();
    let destination = foo.get(dest).unwrap();
    let output: String;
    match conv {
        0 => {
            output = convert::convert_indic_to_indic(&input.to_string(), source, destination);
        }, 
        1 => {
            output = convert::convert_indic_to_roman(&input.to_string(), source, destination);
        },
        2 => {
            output = convert::convert_roman_to_roman(&input.to_string(), source, destination);
        }, 
        3 => {
            output = convert::convert_roman_to_indic(&input.to_string(), source, destination);
        },
        _ => {
            output = String::from("Could Not Convert. Wrong conversion selected!");
        }
    }
    JsValue::from(output)
}
