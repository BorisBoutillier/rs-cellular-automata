mod utils;

use automata_lib::{Automata1D, Rule1D};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct WasmAutomata1D {
    automata: Automata1D,
    width: u32,
}
#[wasm_bindgen]
impl WasmAutomata1D {
    pub fn new(n_colors: u8, rule_nb: u64, width: u32) -> WasmAutomata1D {
        utils::set_panic_hook();
        let rule = Rule1D::new(n_colors, rule_nb);
        let automata = Automata1D::new(rule, -(width as i32 / 2), width);
        WasmAutomata1D { automata, width }
    }
    pub fn get_max_rule_nb(n_colors: u8) -> u64 {
        Rule1D::get_max_nb(n_colors)
    }
    pub fn step(&mut self, n_steps: u32) -> Vec<u8> {
        self.automata
            .as_rgb_vec(n_steps)
            .iter()
            .flat_map(|&(r, g, b)| vec![r, g, b, 255])
            .collect::<Vec<_>>()
    }
    pub fn width(&self) -> u32 {
        self.width
    }
}
