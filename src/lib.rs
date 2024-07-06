mod draw;
mod frontend;
mod utils;

use log::debug;
use log::info;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use frontend::get_min_window_dim;
use frontend::set_canvas_size;
use frontend::setup_input_onchange_callback;

const CANVAS_NAME: &str = "warpedText";
const INPUT_FIELD_NAME: &str = "userInput";

#[wasm_bindgen]
pub fn wasm_main() {
    set_panic_hook();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    info!("Logging initialized");

    let canvas_size = get_min_window_dim();
    let canvas_size_u = canvas_size as u32;
    set_canvas_size(CANVAS_NAME, canvas_size_u, canvas_size_u);
    debug!("Set canvas size to {canvas_size_u}x{canvas_size_u}",);

    setup_input_onchange_callback(canvas_size, 500);
}
