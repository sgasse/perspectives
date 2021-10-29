use wasm_bindgen::Clamped;

use image::{
    imageops::{crop, overlay, rotate90},
    Rgba, RgbaImage,
};
use imageproc::{definitions::Image, drawing::draw_text};
use rusttype::{Font, Scale};
use web_sys::ImageData;

pub fn calc_perspective_image(text: &str) -> ImageData {
    let img = get_scaled_cropped_text(text);
    let img = overlay_with_rotated(img);

    let (sw, sh) = img.dimensions();

    let img_arr: &[u8] = &img.into_raw();

    ImageData::new_with_u8_clamped_array_and_sh(Clamped(img_arr), sw, sh).unwrap()
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

fn load_font() -> Font<'static> {
    let font_data: &[u8] = include_bytes!("../resources/DejaVuSans.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).unwrap();
    return font;
}
