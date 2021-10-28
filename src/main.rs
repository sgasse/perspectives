extern crate perspectives;

use perspectives::get_scaled_cropped_text;
use perspectives::overlay_with_rotated;
use perspectives::write_image;

fn main() {
    match process_main() {
        Ok(_) => (),
        Err(msg) => println!("{}", msg),
    }
}

fn process_main() -> Result<(), String> {
    let img = get_scaled_cropped_text("Rust for fun");
    let img = overlay_with_rotated(img);
    write_image(img, "test_out.png")?;
    Ok(())
}
