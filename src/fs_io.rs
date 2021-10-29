use image::{open as open_image, Rgba, RgbaImage};
use imageproc::definitions::Image;

#[allow(dead_code)]
pub fn read_and_decode(img_file: &str) -> Result<RgbaImage, String> {
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
