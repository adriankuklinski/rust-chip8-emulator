pub mod chip8;
mod cpu;
mod ram;
mod utils;

use console_error_panic_hook;
use crate::chip8::Chip8;
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn initialize_chip8() -> Chip8 {
    console_error_panic_hook::set_once();
    Chip8::new()
}

#[wasm_bindgen]
pub fn run_cycle(chip8: &mut Chip8) {
    chip8.run_cycle();
}

