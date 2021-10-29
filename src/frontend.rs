use super::draw::calc_perspective_image;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlInputElement;
use web_sys::ImageData;

pub fn setup_input_onchange_callback() {
    let document = web_sys::window().unwrap().document().unwrap();

    let callback = Closure::wrap(Box::new(move || {
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document
            .get_element_by_id("inputText")
            .expect("#inputText should exist");
        let input_field = input_field
            .dyn_ref::<HtmlInputElement>()
            .expect("#inputText should be a HtmlInputElement");

        let text = input_field.value();
        if text != "" && text != " " {
            let img_data = calc_perspective_image(&*text);
            set_img_data(img_data);
        } else {
            clear_canvas();
        }
    }) as Box<dyn FnMut()>);

    // Attach the closure as `onchange` callback to the input field.
    document
        .get_element_by_id("inputText")
        .expect("#inputText should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputText should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory.
    callback.forget();
}

pub fn set_canvas_size(width: u32, height: u32) {
    let canvas = get_canvas("textImage").unwrap();

    canvas.set_width(width);
    canvas.set_height(height);
}

fn set_img_data(img_data: ImageData) {
    let canvas = get_canvas("textImage").unwrap();
    let ctx = get_2d_context(&canvas).unwrap();
    ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
}

fn clear_canvas() {
    let canvas = get_canvas("textImage").unwrap();
    let ctx = get_2d_context(&canvas).unwrap();
    ctx.clear_rect(0.0, 0.0, 400.0, 400.0);
}

fn get_canvas(canvas_name: &str) -> Result<web_sys::HtmlCanvasElement, &'static str> {
    let document = match web_sys::window() {
        Some(document) => match document.document() {
            Some(document) => document,
            None => return Err("Could not get document"),
        },
        None => return Err("Could not get document"),
    };

    let canvas = match document.get_element_by_id(canvas_name) {
        Some(canvas) => canvas,
        None => return Err("Could not get canvas"),
    };

    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| "Could not cast canvas")
}

fn get_2d_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<web_sys::CanvasRenderingContext2d, &'static str> {
    match canvas.get_context("2d") {
        Ok(ctx) => match ctx {
            Some(ctx) => {
                return ctx
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .map_err(|_| "Could not cast context")
            }
            None => return Err("Could not get context"),
        },
        Err(_) => return Err("Could not get context"),
    }
}
