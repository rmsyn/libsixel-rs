use super::*;
use crate::{config, output};

#[test]
fn test_encode_header() -> Result<()> {
    let (width, height) = (64, 64);

    let mut output = output::Output::new()
        .with_8bit_control(true)
        .with_palette_type(config::PaletteType::Rgb);

    output.encode_header(width, height)?;

    let buffer = output.buffer.as_ref();

    assert_eq!(&buffer[..DCS_START_8BIT.len()], DCS_START_8BIT.as_ref());

    Ok(())
}
