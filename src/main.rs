extern crate perspectives;

use perspectives::draw::{get_scaled_cropped_text, overlay_with_rotated};
use perspectives::fs_io::write_image;

fn main() {
    match process_main() {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    }
}

fn process_main() -> Result<(), String> {
    let img = get_scaled_cropped_text("Rust for fun", 15.0, 400.0);
    let img = overlay_with_rotated(img);
    write_image(img, "test_out.png")?;
    Ok(())
}
