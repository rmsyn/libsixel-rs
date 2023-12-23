use image::io::Reader as ImageReader;
use libsixel_rs::{quant::MethodForDiffuse, DeviceControlString, Result};

fn main() -> Result<()> {
    let img = ImageReader::open("tests/assets/snek.webp")
        .unwrap()
        .decode()
        .unwrap();

    let (width, height) = (img.width() as usize, img.height() as usize);

    let mut pixels = img.into_bytes();
    let sixel =
        DeviceControlString::from_rgb(pixels.as_mut(), width, height, MethodForDiffuse::Atkinson)?;

    println!("{sixel}");

    Ok(())
}
