use image::io::Reader as ImageReader;
use libsixel_rs::{config, dimension, dither, output, Result};

fn main() -> Result<()> {
    let img = ImageReader::open("tests/assets/snek.webp")
        .unwrap()
        .decode()
        .unwrap();

    let mut output = output::Output::new().with_palette_type(config::PaletteType::Rgb);
    let (width, height) = (img.width() as usize, img.height() as usize);

    let mut dither = dither::SixelDither::get(config::BuiltinPalette::Xterm256)?;

    println!("Image width:{width}, height:{height}");

    let sd = dimension::SpaceDimension {
        width,
        height,
        depth: 3,
        x: 0,
        y: 0,
    };

    let mut pixels = img.into_bytes();

    output.encode(pixels.as_mut(), &mut dither, sd)?;

    Ok(())
}
