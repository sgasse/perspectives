use image::{
    imageops::{overlay, rotate90},
    open as open_image, Rgba, RgbaImage,
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
    // let img = read_and_decode("test_in.png")?;
    let img = RgbaImage::from_pixel(400, 400, Rgba([0, 0, 0, 0]));
    let img = transform(img)?;
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
    let mut xmin = 400;
    let mut xmax = 0;
    let mut ymin = 400;
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

fn transform(mut img: Image<Rgba<u8>>) -> Result<Image<Rgba<u8>>, String> {
    img = draw_text(
        &mut img,
        Rgba([1, 0, 0, 255]),
        20,
        0,
        Scale { x: 20.0, y: 400.0 },
        &load_font(),
        "Hummus aus Moosach",
    );
    let bbox = find_bbox(&img);
    let img_rotated = rotate90(&img);

    overlay(&mut img, &img_rotated, 0, 0);
    return Ok(img);
}
