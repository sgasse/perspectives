use std::sync::OnceLock;

use ab_glyph::{FontRef, PxScale};
use image::{
    imageops::{crop, overlay, rotate90},
    Rgba, RgbaImage,
};
use imageproc::{definitions::Image, drawing::draw_text};
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub fn calc_perspective_image(text: &str, canvas_size: f32) -> ImageData {
    let img = get_scaled_cropped_text(text, canvas_size / 26.0, canvas_size, canvas_size as u32);
    let img = overlay_with_rotated(img, canvas_size as u32);

    let (width, _height) = img.dimensions();
    ImageData::new_with_u8_clamped_array(Clamped(&img.into_raw()), width).unwrap()
}

fn get_scaled_cropped_text(
    text: &str,
    x_scale: f32,
    y_scale: f32,
    background_size: u32,
) -> Image<Rgba<u8>> {
    let img = RgbaImage::from_pixel(background_size, background_size, Rgba([0, 0, 0, 0]));
    let mut img = draw_text(
        &img,
        Rgba([0, 0, 0, 255]),
        0,
        0,
        PxScale {
            x: x_scale,
            y: y_scale,
        },
        &load_font(),
        text,
    );
    let bbox = find_bbox(&img);
    let width = bbox.1 - bbox.0;
    let height = bbox.3 - bbox.2;
    crop(&mut img, bbox.0, bbox.2, width, height).to_image()
}

fn overlay_with_rotated(img: Image<Rgba<u8>>, length: u32) -> Image<Rgba<u8>> {
    let (width, height) = img.dimensions();

    let rotated = rotate90(&img);

    let mut bottom_layer = RgbaImage::from_pixel(length, length, Rgba([255, 255, 255, 255]));

    // Calculate corner point for image to overlay on the bottom layer.
    let x = ((length - width) / 2) as i64;
    let y = ((length - height) / 2) as i64;

    overlay(&mut bottom_layer, &img, x, y);
    // Use inverted corner point (y, x) for rotated image.
    overlay(&mut bottom_layer, &rotated, y, x);

    bottom_layer
}

fn find_bbox(img: &Image<Rgba<u8>>) -> (u32, u32, u32, u32) {
    let (width, height) = img.dimensions();

    // Initialize bounding box limits.
    let mut xmin = width;
    let mut xmax = 0;
    let mut ymin = height;
    let mut ymax = 0;

    // Loop over all pixels.
    for (x, y, pixel) in img.enumerate_pixels() {
        // Find pixels which are non-empty.
        if *pixel != Rgba([0, 0, 0, 0]) {
            // Update bounding box limits.
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

fn load_font() -> &'static FontRef<'static> {
    static FONT: OnceLock<FontRef> = OnceLock::new();
    FONT.get_or_init(|| {
        let font_data: &[u8] = include_bytes!("../resources/DejaVuSansMono.ttf");
        FontRef::try_from_slice(font_data).unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::get_scaled_cropped_text;

    #[test]
    fn check_letters_in_bound() {
        let mut max_height = 0;

        for byte_num in 33..=126 {
            let byte_arr = [byte_num];
            let test_letter = std::str::from_utf8(&byte_arr).unwrap();
            let img = get_scaled_cropped_text(&format!("{}", test_letter), 15.0, 400.0, 400);
            let (_width, height) = img.dimensions();

            if height > max_height {
                max_height = height;
            }
        }

        println!("Maximum height: {}px", max_height);
        assert!(max_height < 400);
    }
}
