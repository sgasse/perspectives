use crate::CANVAS_NAME;
use crate::INPUT_FIELD_NAME;

use super::draw::calc_perspective_image;

use log::debug;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlInputElement;
use web_sys::ImageData;

/// Bind closure to react to changing input to the onChange event
///
/// We store a handle to the inner update callback closure so it does not get
/// dropped and we also do not have to leak the memory, hence the unused
/// variable/assignment.
#[allow(unused_variables, unused_assignments)]
pub(crate) fn setup_input_onchange_callback(canvas_size: f64, timeout_millis: i32) {
    let document = web_sys::window().unwrap().document().unwrap();

    // If the user types a whole word or sentence, we do not want to update the
    // whole text for every individual letter entered. Therefore, we do not
    // trigger the recalculation of the image directly. Instead, we set a
    // timeout for the update closure and replace it with a newer one if the
    // text field was updated while we were waiting.
    // To be able to clear the right callback and not have it destroyed and leaking memory,
    // we store both the ID and the handle.
    let mut last_inner_callback_id: Option<i32> = None;
    let mut update_callback_handle: Option<Closure<dyn FnMut()>> = None;

    let callback = Closure::wrap(Box::new(move || {
        let window = web_sys::window().unwrap();
        // Clear last scheduled update callback if it exists.
        if let Some(id) = last_inner_callback_id {
            window.clear_timeout_with_handle(id);
        }

        let update_callback = Closure::wrap(get_image_update_closure(canvas_size));

        // Schedule a new update callback.
        last_inner_callback_id = Some(
            window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    update_callback.as_ref().unchecked_ref(),
                    timeout_millis,
                )
                .expect("failed to set timeout with callback"),
        );

        // Assign handle to variable.
        // This way, we do not destroy the closure upon exiting the callback
        // and we also do not leak memory on every onChange event.
        update_callback_handle = Some(update_callback);
    }) as Box<dyn FnMut()>);

    // Attach the closure as `onchange` callback to the input field.
    document
        .get_element_by_id(INPUT_FIELD_NAME)
        .expect("input field should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("input field should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

pub(crate) fn get_min_window_dim() -> f64 {
    let window = web_sys::window().unwrap();

    let inner_width = window.inner_width().unwrap().as_f64().unwrap();
    let inner_height = window.inner_height().unwrap().as_f64().unwrap();

    let mut min_res = inner_width;
    if inner_height < min_res {
        min_res = inner_height;
    }

    min_res
}

pub(crate) fn set_canvas_size(canvas_name: &str, width: u32, height: u32) {
    let canvas = get_canvas(canvas_name).expect("failed to get canvas");

    canvas.set_width(width);
    canvas.set_height(height);
}

fn get_image_update_closure(canvas_size: f64) -> Box<dyn FnMut()> {
    Box::new(move || {
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document
            .get_element_by_id(INPUT_FIELD_NAME)
            .expect("input field should exist");
        let input_field = input_field
            .dyn_ref::<HtmlInputElement>()
            .expect("input field should be a HtmlInputElement");

        let text = input_field.value();
        if !text.trim().is_empty() {
            let img_data = calc_perspective_image(&text, get_min_window_dim());
            set_img_data(CANVAS_NAME, img_data);
            debug!("Updated rendered text to \"{text}\"");
        } else {
            clear_canvas(CANVAS_NAME, canvas_size);
            debug!("Cleared canvas");
        }
    })
}

fn set_img_data(canvas_name: &str, img_data: ImageData) {
    let canvas = get_canvas(canvas_name).unwrap();
    let ctx = get_2d_context(&canvas).unwrap();
    ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
}

fn clear_canvas(canvas_name: &str, size: f64) {
    let canvas = get_canvas(canvas_name).unwrap();
    let ctx = get_2d_context(&canvas).unwrap();
    ctx.clear_rect(0.0, 0.0, size, size);
}

fn get_canvas(canvas_name: &str) -> Option<web_sys::HtmlCanvasElement> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(canvas_name))
        .and_then(|c| c.dyn_into::<web_sys::HtmlCanvasElement>().ok())
}

fn get_2d_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> Option<web_sys::CanvasRenderingContext2d> {
    canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|x| x.dyn_into::<web_sys::CanvasRenderingContext2d>().ok())
}
