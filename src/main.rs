use image::{open as open_image, Rgb, RgbImage};
use imageproc::{
    definitions::Image,
    drawing::{draw_cross, draw_text},
    geometric_transformations::{warp, Interpolation, Projection},
};
use rusttype::{Font, Scale};

fn main() {
    match process_main() {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    }
}

fn process_main() -> Result<(), String> {
    let img = read_and_decode("test_in.png")?;
    let img = transform(img)?;
    write_image(img, "test_out.png")?;
    Ok(())
}

fn read_and_decode(img_file: &str) -> Result<RgbImage, String> {
    match open_image(img_file) {
        Ok(img) => Ok(img.into_rgb8()),
        Err(_) => Err(format!("Could not read {}", img_file)),
    }
}

fn write_image(img: Image<Rgb<u8>>, filename: &str) -> Result<(), String> {
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

fn transform(mut img: Image<Rgb<u8>>) -> Result<Image<Rgb<u8>>, String> {
    img = draw_text(
        &mut img,
        Rgb([50, 255, 50]),
        20,
        20,
        Scale { x: 20.0, y: 20.0 },
        &load_font(),
        "This is a very long text let's write it",
    );
    let trans_fn = Projection::scale(1.0, 1.0);
    let img = warp(&img, &trans_fn, Interpolation::Bilinear, Rgb([0, 0, 0]));
    return Ok(img);
}
