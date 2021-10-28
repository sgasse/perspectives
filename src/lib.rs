mod utils;

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use image::{
    imageops::{crop, overlay, rotate90},
    open as open_image, Rgba, RgbaImage,
};
use imageproc::{definitions::Image, drawing::draw_text};
use rusttype::{Font, Scale};
use wasm_bindgen::Clamped;
use web_sys::window;
use web_sys::ImageData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, perspectives!");
}

#[allow(dead_code)]
fn read_and_decode(img_file: &str) -> Result<RgbaImage, String> {
    match open_image(img_file) {
        Ok(img) => Ok(img.into_rgba8()),
        Err(_) => Err(format!("Could not read {}", img_file)),
    }
}

pub fn write_image(img: Image<Rgba<u8>>, filename: &str) -> Result<(), String> {
    match img.save(filename) {
        Ok(_) => {
            println!("Wrote file {}", filename);
            return Ok(());
        }
        Err(_) => {
            return Err(format!("Could not save to {}", filename));
        }
    }
}

fn load_font() -> Font<'static> {
    let font_data: &[u8] = include_bytes!("../resources/DejaVuSans.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    return font;
}

fn find_bbox(img: &Image<Rgba<u8>>) -> (u32, u32, u32, u32) {
    let (width, height) = img.dimensions();
    let mut xmin = width;
    let mut xmax = 0;
    let mut ymin = height;
    let mut ymax = 0;

    for (x, y, pixel) in img.enumerate_pixels() {
        if *pixel != Rgba([0, 0, 0, 0]) {
            if x < xmin {
                xmin = x;
            }
            if y < ymin {
                ymin = y
            }
            if x > xmax {
                xmax = x;
            }
            if y > ymax {
                ymax = y;
            }
        }
    }

    (xmin, xmax, ymin, ymax)
}

pub fn get_scaled_cropped_text(text: &str) -> Image<Rgba<u8>> {
    let mut img = RgbaImage::from_pixel(400, 400, Rgba([0, 0, 0, 0]));
    img = draw_text(
        &mut img,
        Rgba([0, 0, 0, 255]),
        20,
        0,
        Scale { x: 20.0, y: 400.0 },
        &load_font(),
        text,
    );
    let bbox = find_bbox(&img);
    let width = bbox.1 - bbox.0;
    let height = bbox.3 - bbox.2;
    crop(&mut img, bbox.0, bbox.2, width, height).to_image()
}

pub fn overlay_with_rotated(img: Image<Rgba<u8>>) -> Image<Rgba<u8>> {
    let (width, height) = img.dimensions();
    let length = u32::max(width, height) + 10;

    let rotated = rotate90(&img.clone());

    let mut bottom = RgbaImage::from_pixel(length, length, Rgba([255, 255, 255, 255]));

    overlay(
        &mut bottom,
        &img,
        (length - width) / 2,
        (length - height) / 2,
    );

    overlay(
        &mut bottom,
        &rotated,
        // inverted for rotated image
        (length - height) / 2,
        (length - width) / 2,
    );
    bottom
}

#[wasm_bindgen]
pub fn create_image() {
    let window = window().unwrap();
    let img = get_scaled_cropped_text("WASM for fun");
    let canvas = get_canvas("textImage").unwrap();

    let (sw, sh) = img.dimensions();

    let img_arr: &[u8] = &img.into_raw();

    let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(img_arr), sw, sh).unwrap();

    let ctx = get_2d_context(&canvas).unwrap();
    ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
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