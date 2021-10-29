pub mod draw;
pub mod frontend;
pub mod fs_io;
mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use frontend::set_canvas_size;
use frontend::setup_input_onchange_callback;

#[wasm_bindgen]
pub fn wasm_main() {
    set_panic_hook();
    set_canvas_size(400, 400);
    setup_input_onchange_callback();
}
