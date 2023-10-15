pub mod draw;
pub mod frontend;
pub mod fs_io;
mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use frontend::get_min_window_dim;
use frontend::set_canvas_size;
use frontend::setup_input_onchange_callback;

#[wasm_bindgen]
pub fn wasm_main() {
    set_panic_hook();

    let canvas_size = get_min_window_dim() as u32;
    set_canvas_size(canvas_size, canvas_size);

    setup_input_onchange_callback(500);
}
