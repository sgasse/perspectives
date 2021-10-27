use image::{
    imageops::{crop, overlay, rotate90},
    open as open_image, GenericImage, GenericImageView, Rgba, RgbaImage,
};
use imageproc::{definitions::Image, drawing::draw_text};
use rusttype::{Font, Scale};

fn main() {
    match process_main() {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    }
}

fn process_main() -> Result<(), String> {
    let img = get_scaled_cropped_text("Hummus aus Moosach");
    let img = overlay_with_rotated(img);
    write_image(img, "test_out.png")?;
    Ok(())
}

#[allow(dead_code)]
fn read_and_decode(img_file: &str) -> Result<RgbaImage, String> {
    match open_image(img_file) {
        Ok(img) => Ok(img.into_rgba8()),
        Err(_) => Err(format!("Could not read {}", img_file)),
    }
}

fn write_image(img: Image<Rgba<u8>>, filename: &str) -> Result<(), String> {
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

fn get_scaled_cropped_text(text: &str) -> Image<Rgba<u8>> {
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

fn overlay_with_rotated(img: Image<Rgba<u8>>) -> Image<Rgba<u8>> {
    let (width, height) = img.dimensions();
    let length = u32::max(width, height) + 10;

    let rotated = rotate90(&img.clone());

    let mut bottom = RgbaImage::from_pixel(length, length, Rgba([0, 0, 0, 0]));

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
