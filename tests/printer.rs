#[test]
#[cfg(feature = "std")]
fn test_print_sixel() {
    let img = include_bytes!("assets/snake.six");
    let sixel = std::str::from_utf8(img.as_ref()).unwrap_or("");
    println!("{sixel}");
}
