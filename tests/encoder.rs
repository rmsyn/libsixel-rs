#[test]
#[cfg(feature = "e2e-tests")]
fn test_encode_image() -> libsixel_rs::Result<()> {
    use image::io::Reader as ImageReader;
    use libsixel_rs::{config, dimension, dither, output, quant};

    let img = ImageReader::open("tests/assets/snake.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let mut output = output::Output::new()
        .with_penetrate_multiplexer(false)
        .with_8bit_control(false)
        .with_palette_type(config::PaletteType::Rgb);
    let (width, height) = (img.width() as usize, img.height() as usize);

    let mut dither = dither::SixelDither::get(config::BuiltinPalette::Xterm256)?;

    dither.quality_mode = quant::QualityMode::HighColor;

    println!("Image width:{width}, height:{height}");

    let sd = dimension::SpaceDimension {
        width,
        height,
        depth: 3,
        x: 0,
        y: 0,
    };

    let mut pixels = img.into_bytes();

    output.encode(pixels.as_mut(), &mut dither, sd)
}
