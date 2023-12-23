mod common;

#[test]
#[cfg(feature = "e2e-tests")]
fn test_encode_dcs() -> libsixel_rs::Result<()> {
    use image::io::Reader as ImageReader;
    use libsixel_rs::{quant::MethodForDiffuse, DeviceControlString};
    use std::io::Write;

    common::init();

    let img = ImageReader::open("tests/assets/snake.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let (width, height) = (img.width() as usize, img.height() as usize);

    println!("Image width:{width}, height:{height}");

    let mut pixels = img.into_bytes();
    let sixel =
        DeviceControlString::from_rgb(pixels.as_mut(), width, height, MethodForDiffuse::Atkinson)?;

    let sixel_str = format!("{sixel}");
    println!("{sixel_str}");

    println!("Color map entries: {}", sixel.color_map().items().len());

    let mut output = std::fs::File::create("tests/assets/snake-self.six")?;
    output.write_all(sixel_str.as_bytes())?;

    Ok(())
}

#[test]
#[cfg(feature = "e2e-tests")]
fn test_encode_manual_dcs() -> libsixel_rs::Result<()> {
    use libsixel_rs::{quant::MethodForDiffuse, DeviceControlString};
    use std::io::Write;

    common::init();

    let (width, height) = (3usize, 6usize);
    println!("Image width:{width}, height:{height}");

    #[rustfmt::skip]
    let mut pixels = [
        0xf4, 0x00, 0x00, /**/ 0x00, 0xf4, 0x00, /**/ 0x00, 0x00, 0xf4,
        0xf4, 0x00, 0x00, /**/ 0x00, 0xf4, 0x00, /**/ 0x00, 0x00, 0xf4,
        0xf4, 0x00, 0x00, /**/ 0x00, 0xf4, 0x00, /**/ 0x00, 0x00, 0xf4,
        0xf4, 0x00, 0x00, /**/ 0x00, 0xf4, 0x00, /**/ 0x00, 0x00, 0xf4,
        0xf4, 0x00, 0x00, /**/ 0x00, 0xf4, 0x00, /**/ 0x00, 0x00, 0xf4,
        0xf4, 0x00, 0x00, /**/ 0x00, 0xf4, 0x00, /**/ 0x00, 0x00, 0xf4,
    ];
    let sixel =
        DeviceControlString::from_rgb(pixels.as_mut(), width, height, MethodForDiffuse::Atkinson)?;

    let sixel_str = format!("{sixel}");
    println!("{sixel_str}");

    let mut output = std::fs::File::create("tests/assets/manual-self.six")?;
    output.write_all(sixel_str.as_bytes())?;

    Ok(())
}
