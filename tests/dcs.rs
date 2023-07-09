#[test]
#[cfg(feature = "e2e-tests")]
fn test_encode_dcs() -> libsixel_rs::Result<()> {
    use image::io::Reader as ImageReader;
    use libsixel_rs::device_control_string::DeviceControlString;
    use std::io::Write;

    let img = ImageReader::open("tests/assets/snake.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let (width, height) = (img.width() as usize, img.height() as usize);

    println!("Image width:{width}, height:{height}");

    let mut pixels = img.into_bytes();
    let sixel = DeviceControlString::from_rgb(pixels.as_mut(), width as usize, height as usize)?;

    let sixel_str = format!("{sixel}");
    println!("{sixel_str}");

    let mut output = std::fs::File::create("tests/assets/snake-self.six")?;
    output.write_all(sixel_str.as_bytes())?;

    Ok(())
}
