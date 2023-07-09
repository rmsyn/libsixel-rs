//! Types and functions for handling Sixel frames.

/// Maximum frame width.
pub const MAX_FRAME_WIDTH: usize = 1_000_000;

/// Maximum frame height.
pub const MAX_FRAME_HEIGHT: usize = 1_000_000;

/// Represents a Sixel frame of pixels.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SixelFrame {
    pub pixels: Vec<u8>,
    pub palette: &'static [u8],
    pub width: usize,
    pub height: usize,
    pub ncolors: usize,
    pub pixel_format: i32,
    pub delay: i32,
    pub frame_no: i32,
    pub loop_count: usize,
    pub multiframe: i32,
    pub transparent: i32,
}

impl SixelFrame {
    /// Creates a new [SixelFrame].
    pub const fn new() -> Self {
        Self {
            pixels: Vec::new(),
            palette: &[],
            width: 0,
            height: 0,
            ncolors: -1,
            pixel_format: 0x3,
            delay: 0,
            frame_no: 0,
            loop_count: 0,
            multiframe: 0,
            transparent: -1,
        }
    }

    /// Initializes the [SixelFrame].
    pub fn init(
        &mut self,
        pixels: &[u8],
        width: usize,
        height: usize,
        pixel_format: i32,
        palette: &'static [u8],
        ncolors: usize,
    ) -> Result<()> {
        if width > MAX_FRAME_WIDTH {
            Err(Error::Frame(format!("init: max frame width: {MAX_FRAME_WIDTH}, have: {width}")))
        } else if height > MAX_FRAME_HEIGHT as libc::c_int {
            Err(Error::Frame(format!("init: max frame height: {MAX_FRAME_HEIGHT}, have: {height}")))
        } else {
            self.pixels = pixels;
            self.width = width;
            self.height = height;
            self.pixel_format = pixel_format;
            self.palette = palette;
            self.ncolors = ncolors;

            Ok(())
        }
    }

    /// Gets the pixels for the [SixelFrame].
    pub fn pixels(&self) -> &[u8] {
        self.pixels.as_ref()
    }

    /// Gets the palette for the [SixelFrame].
    pub fn palette(&self) -> &[u8] {
        self.palette
    }

    /// Gets the width for the [SixelFrame].
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height for the [SixelFrame].
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets the ncolors for the [SixelFrame].
    pub fn ncolors(&self) -> usize {
        self.ncolors
    }

    /// Gets the pixel format for the [SixelFrame].
    pub fn pixel_format(&self) -> i32 {
        self.pixel_format
    }

    /// Gets the delay for the [SixelFrame].
    pub fn delay(&self) -> i32 {
        self.delay
    }

    /// Gets the frame number for the [SixelFrame].
    pub fn frame_no(&self) -> i32 {
        self.frame_no
    }

    /// Gets the loop count for the [SixelFrame].
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    /// Gets the frame number for the [SixelFrame].
    pub fn multi_frame(&self) -> i32 {
        self.multi_frame
    }

    /// Gets the transparent for the [SixelFrame].
    pub fn transparent(&self) -> i32 {
        self.transparent
    }
}

/*
#[no_mangle]
pub unsafe extern "C" fn sixel_frame_strip_alpha(
    mut frame: *mut sixel_frame_t,
    mut bgcolor: *mut libc::c_uchar,
) -> libc::c_int {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut alpha: libc::c_uchar = 0;
    sixel_frame_ref(frame);
    dst = (*frame).pixels;
    src = dst;
    if !bgcolor.is_null() {
        match (*frame).pixelformat {
            16 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    alpha = *src.offset(0 as libc::c_int as isize);
                    let fresh0 = src;
                    src = src.offset(1);
                    let fresh1 = dst;
                    dst = dst.offset(1);
                    *fresh1 = (*fresh0 as libc::c_int * alpha as libc::c_int
                        + *bgcolor.offset(0 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh2 = src;
                    src = src.offset(1);
                    let fresh3 = dst;
                    dst = dst.offset(1);
                    *fresh3 = (*fresh2 as libc::c_int * alpha as libc::c_int
                        + *bgcolor.offset(1 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh4 = src;
                    src = src.offset(1);
                    let fresh5 = dst;
                    dst = dst.offset(1);
                    *fresh5 = (*fresh4 as libc::c_int * alpha as libc::c_int
                        + *bgcolor.offset(2 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    src = src.offset(1);
                    src;
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            17 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    alpha = *src.offset(3 as libc::c_int as isize);
                    let fresh6 = src;
                    src = src.offset(1);
                    let fresh7 = dst;
                    dst = dst.offset(1);
                    *fresh7 = (*fresh6 as libc::c_int * alpha as libc::c_int
                        + *bgcolor.offset(0 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh8 = src;
                    src = src.offset(1);
                    let fresh9 = dst;
                    dst = dst.offset(1);
                    *fresh9 = (*fresh8 as libc::c_int * alpha as libc::c_int
                        + *bgcolor.offset(1 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh10 = src;
                    src = src.offset(1);
                    let fresh11 = dst;
                    dst = dst.offset(1);
                    *fresh11 = (*fresh10 as libc::c_int * alpha as libc::c_int
                        + *bgcolor.offset(2 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    src = src.offset(1);
                    src;
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            18 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    alpha = *src.offset(0 as libc::c_int as isize);
                    let fresh12 = dst;
                    dst = dst.offset(1);
                    *fresh12 = (*src.offset(3 as libc::c_int as isize) as libc::c_int
                        * alpha as libc::c_int
                        + *bgcolor.offset(0 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh13 = dst;
                    dst = dst.offset(1);
                    *fresh13 = (*src.offset(2 as libc::c_int as isize) as libc::c_int
                        * alpha as libc::c_int
                        + *bgcolor.offset(1 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh14 = dst;
                    dst = dst.offset(1);
                    *fresh14 = (*src.offset(1 as libc::c_int as isize) as libc::c_int
                        * alpha as libc::c_int
                        + *bgcolor.offset(2 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    src = src.offset(4 as libc::c_int as isize);
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            19 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    alpha = *src.offset(3 as libc::c_int as isize);
                    let fresh15 = dst;
                    dst = dst.offset(1);
                    *fresh15 = (*src.offset(2 as libc::c_int as isize) as libc::c_int
                        * alpha as libc::c_int
                        + *bgcolor.offset(0 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh16 = dst;
                    dst = dst.offset(1);
                    *fresh16 = (*src.offset(1 as libc::c_int as isize) as libc::c_int
                        * alpha as libc::c_int
                        + *bgcolor.offset(1 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    let fresh17 = dst;
                    dst = dst.offset(1);
                    *fresh17 = (*src.offset(0 as libc::c_int as isize) as libc::c_int
                        * alpha as libc::c_int
                        + *bgcolor.offset(2 as libc::c_int as isize) as libc::c_int
                            * (0xff as libc::c_int - alpha as libc::c_int)
                        >> 8 as libc::c_int) as libc::c_uchar;
                    src = src.offset(4 as libc::c_int as isize);
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            _ => {}
        }
    } else {
        match (*frame).pixelformat {
            16 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    src = src.offset(1);
                    src;
                    let fresh18 = src;
                    src = src.offset(1);
                    let fresh19 = dst;
                    dst = dst.offset(1);
                    *fresh19 = *fresh18;
                    let fresh20 = src;
                    src = src.offset(1);
                    let fresh21 = dst;
                    dst = dst.offset(1);
                    *fresh21 = *fresh20;
                    let fresh22 = src;
                    src = src.offset(1);
                    let fresh23 = dst;
                    dst = dst.offset(1);
                    *fresh23 = *fresh22;
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            17 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    let fresh24 = src;
                    src = src.offset(1);
                    let fresh25 = dst;
                    dst = dst.offset(1);
                    *fresh25 = *fresh24;
                    let fresh26 = src;
                    src = src.offset(1);
                    let fresh27 = dst;
                    dst = dst.offset(1);
                    *fresh27 = *fresh26;
                    let fresh28 = src;
                    src = src.offset(1);
                    let fresh29 = dst;
                    dst = dst.offset(1);
                    *fresh29 = *fresh28;
                    src = src.offset(1);
                    src;
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            18 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    let fresh30 = dst;
                    dst = dst.offset(1);
                    *fresh30 = *src.offset(3 as libc::c_int as isize);
                    let fresh31 = dst;
                    dst = dst.offset(1);
                    *fresh31 = *src.offset(2 as libc::c_int as isize);
                    let fresh32 = dst;
                    dst = dst.offset(1);
                    *fresh32 = *src.offset(1 as libc::c_int as isize);
                    src = src.offset(4 as libc::c_int as isize);
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            19 => {
                i = 0 as libc::c_int;
                while i < (*frame).height * (*frame).width {
                    let fresh33 = dst;
                    dst = dst.offset(1);
                    *fresh33 = *src.offset(2 as libc::c_int as isize);
                    let fresh34 = dst;
                    dst = dst.offset(1);
                    *fresh34 = *src.offset(1 as libc::c_int as isize);
                    let fresh35 = dst;
                    dst = dst.offset(1);
                    *fresh35 = *src.offset(0 as libc::c_int as isize);
                    src = src.offset(4 as libc::c_int as isize);
                    i += 1;
                    i;
                }
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            }
            _ => {}
        }
    }
    status = 0 as libc::c_int;
    sixel_frame_unref(frame);
    return status;
}

unsafe extern "C" fn sixel_frame_convert_to_rgb888(
    mut frame: *mut sixel_frame_t,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut normalized_pixels: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: size_t = 0;
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    sixel_frame_ref(frame);
    match (*frame).pixelformat {
        128 | 129 | 130 => {
            size = ((*frame).width * (*frame).height * 4 as libc::c_int) as size_t;
            normalized_pixels = sixel_allocator_malloc((*frame).allocator, size)
                as *mut libc::c_uchar;
            if normalized_pixels.is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_frame_convert_to_rgb888: sixel_allocator_malloc() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 9511792277212468016;
            } else {
                src = normalized_pixels
                    .offset(
                        ((*frame).width * (*frame).height * 3 as libc::c_int) as isize,
                    );
                dst = normalized_pixels;
                status = sixel_helper_normalize_pixelformat(
                    src,
                    &mut (*frame).pixelformat,
                    (*frame).pixels,
                    (*frame).pixelformat,
                    (*frame).width,
                    (*frame).height,
                );
                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                    sixel_allocator_free(
                        (*frame).allocator,
                        normalized_pixels as *mut libc::c_void,
                    );
                    current_block = 9511792277212468016;
                } else {
                    p = src;
                    while dst < src {
                        let fresh36 = dst;
                        dst = dst.offset(1);
                        *fresh36 = *((*frame).palette)
                            .offset((*p as libc::c_int * 3 as libc::c_int) as isize)
                            .offset(0 as libc::c_int as isize);
                        let fresh37 = dst;
                        dst = dst.offset(1);
                        *fresh37 = *((*frame).palette)
                            .offset((*p as libc::c_int * 3 as libc::c_int) as isize)
                            .offset(1 as libc::c_int as isize);
                        let fresh38 = dst;
                        dst = dst.offset(1);
                        *fresh38 = *((*frame).palette)
                            .offset((*p as libc::c_int * 3 as libc::c_int) as isize)
                            .offset(2 as libc::c_int as isize);
                        p = p.offset(1);
                        p;
                    }
                    sixel_allocator_free(
                        (*frame).allocator,
                        (*frame).pixels as *mut libc::c_void,
                    );
                    (*frame).pixels = normalized_pixels;
                    (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                    current_block = 6717214610478484138;
                }
            }
        }
        131 => {
            size = ((*frame).width * (*frame).height * 3 as libc::c_int) as size_t;
            normalized_pixels = sixel_allocator_malloc((*frame).allocator, size)
                as *mut libc::c_uchar;
            if normalized_pixels.is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_frame_convert_to_rgb888: sixel_allocator_malloc() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 9511792277212468016;
            } else {
                src = (*frame).pixels;
                dst = normalized_pixels;
                while dst != normalized_pixels.offset(size as isize) {
                    let fresh39 = dst;
                    dst = dst.offset(1);
                    *fresh39 = *((*frame).palette)
                        .offset(
                            (*src as libc::c_int * 3 as libc::c_int + 0 as libc::c_int)
                                as isize,
                        );
                    let fresh40 = dst;
                    dst = dst.offset(1);
                    *fresh40 = *((*frame).palette)
                        .offset(
                            (*src as libc::c_int * 3 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        );
                    let fresh41 = dst;
                    dst = dst.offset(1);
                    *fresh41 = *((*frame).palette)
                        .offset(
                            (*src as libc::c_int * 3 as libc::c_int + 2 as libc::c_int)
                                as isize,
                        );
                    src = src.offset(1);
                    src;
                }
                sixel_allocator_free(
                    (*frame).allocator,
                    (*frame).pixels as *mut libc::c_void,
                );
                (*frame).pixels = normalized_pixels;
                (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                current_block = 6717214610478484138;
            }
        }
        3 => {
            current_block = 6717214610478484138;
        }
        67 | 99 | 83 | 1 | 2 | 4 | 5 | 17 | 16 => {
            size = ((*frame).width * (*frame).height * 3 as libc::c_int) as size_t;
            normalized_pixels = sixel_allocator_malloc((*frame).allocator, size)
                as *mut libc::c_uchar;
            if normalized_pixels.is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_frame_convert_to_rgb888: sixel_allocator_malloc() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 9511792277212468016;
            } else {
                status = sixel_helper_normalize_pixelformat(
                    normalized_pixels,
                    &mut (*frame).pixelformat,
                    (*frame).pixels,
                    (*frame).pixelformat,
                    (*frame).width,
                    (*frame).height,
                );
                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                    sixel_allocator_free(
                        (*frame).allocator,
                        normalized_pixels as *mut libc::c_void,
                    );
                    current_block = 9511792277212468016;
                } else {
                    sixel_allocator_free(
                        (*frame).allocator,
                        (*frame).pixels as *mut libc::c_void,
                    );
                    (*frame).pixels = normalized_pixels;
                    current_block = 6717214610478484138;
                }
            }
        }
        _ => {
            status = 0x1000 as libc::c_int | 0x200 as libc::c_int;
            sixel_helper_set_additional_message(
                b"do_resize: invalid pixelformat.\0" as *const u8 as *const libc::c_char,
            );
            current_block = 9511792277212468016;
        }
    }
    match current_block {
        6717214610478484138 => {
            status = 0 as libc::c_int;
        }
        _ => {}
    }
    sixel_frame_unref(frame);
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn sixel_frame_resize(
    mut frame: *mut sixel_frame_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut method_for_resampling: libc::c_int,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut size: size_t = 0;
    let mut scaled_frame: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    sixel_frame_ref(frame);
    if width <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_resize: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_resize: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if width > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_resize: given width parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_resize: given height parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else {
        status = sixel_frame_convert_to_rgb888(frame);
        if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
            size = (width as size_t)
                .wrapping_mul(height as size_t)
                .wrapping_mul(3 as libc::c_ulong);
            scaled_frame = sixel_allocator_malloc((*frame).allocator, size)
                as *mut libc::c_uchar;
            if scaled_frame.is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_frame_resize: sixel_allocator_malloc() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
            } else {
                status = sixel_helper_scale_image(
                    scaled_frame,
                    (*frame).pixels,
                    (*frame).width,
                    (*frame).height,
                    3 as libc::c_int,
                    width,
                    height,
                    method_for_resampling,
                    (*frame).allocator,
                );
                if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                    sixel_allocator_free(
                        (*frame).allocator,
                        (*frame).pixels as *mut libc::c_void,
                    );
                    (*frame).pixels = scaled_frame;
                    (*frame).width = width;
                    (*frame).height = height;
                    status = 0 as libc::c_int;
                }
            }
        }
    }
    sixel_frame_unref(frame);
    return status;
}

unsafe extern "C" fn clip(
    mut pixels: *mut libc::c_uchar,
    mut sx: libc::c_int,
    mut sy: libc::c_int,
    mut pixelformat: libc::c_int,
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut cw: libc::c_int,
    mut ch: libc::c_int,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut depth: libc::c_int = 0;
    let mut message: [libc::c_char; 256] = [0; 256];
    let mut nwrite: libc::c_int = 0;
    match pixelformat {
        131 | 67 | 3 => {
            depth = sixel_helper_compute_depth(pixelformat);
            if depth < 0 as libc::c_int {
                status = 0x1000 as libc::c_int | 0x200 as libc::c_int;
                nwrite = sprintf(
                    message.as_mut_ptr(),
                    b"clip: sixel_helper_compute_depth(%08x) failed.\0" as *const u8
                        as *const libc::c_char,
                    pixelformat,
                );
                if nwrite > 0 as libc::c_int {
                    sixel_helper_set_additional_message(message.as_mut_ptr());
                }
            } else {
                dst = pixels;
                src = pixels
                    .offset((cy * sx * depth) as isize)
                    .offset((cx * depth) as isize);
                y = 0 as libc::c_int;
                while y < ch {
                    memmove(
                        dst as *mut libc::c_void,
                        src as *const libc::c_void,
                        (cw * depth) as size_t,
                    );
                    dst = dst.offset((cw * depth) as isize);
                    src = src.offset((sx * depth) as isize);
                    y += 1;
                    y;
                }
                status = 0 as libc::c_int;
            }
        }
        _ => {
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x2 as libc::c_int;
            nwrite = sprintf(
                message.as_mut_ptr(),
                b"clip: invalid pixelformat(%08x) is specified.\0" as *const u8
                    as *const libc::c_char,
                pixelformat,
            );
            if nwrite > 0 as libc::c_int {
                sixel_helper_set_additional_message(message.as_mut_ptr());
            }
        }
    }
    return status;
}

#[no_mangle]
pub unsafe extern "C" fn sixel_frame_clip(
    mut frame: *mut sixel_frame_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut normalized_pixels: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    sixel_frame_ref(frame);
    if width <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_clip: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_clip: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if width > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_clip: given width parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"sixel_frame_clip: given height parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else {
        match (*frame).pixelformat {
            128 | 129 | 130 | 64 | 65 | 66 => {
                normalized_pixels = sixel_allocator_malloc(
                    (*frame).allocator,
                    ((*frame).width * (*frame).height) as size_t,
                ) as *mut libc::c_uchar;
                status = sixel_helper_normalize_pixelformat(
                    normalized_pixels,
                    &mut (*frame).pixelformat,
                    (*frame).pixels,
                    (*frame).pixelformat,
                    (*frame).width,
                    (*frame).height,
                );
                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                    sixel_allocator_free(
                        (*frame).allocator,
                        normalized_pixels as *mut libc::c_void,
                    );
                    current_block = 16857636641493961589;
                } else {
                    sixel_allocator_free(
                        (*frame).allocator,
                        (*frame).pixels as *mut libc::c_void,
                    );
                    (*frame).pixels = normalized_pixels;
                    current_block = 26972500619410423;
                }
            }
            _ => {
                current_block = 26972500619410423;
            }
        }
        match current_block {
            16857636641493961589 => {}
            _ => {
                status = clip(
                    (*frame).pixels,
                    (*frame).width,
                    (*frame).height,
                    (*frame).pixelformat,
                    x,
                    y,
                    width,
                    height,
                );
                if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                    (*frame).width = width;
                    (*frame).height = height;
                    status = 0 as libc::c_int;
                }
            }
        }
    }
    sixel_frame_unref(frame);
    return status;
}
*/
