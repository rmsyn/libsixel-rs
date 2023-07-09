#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type sixel_allocator;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sixel_allocator_new(
        ppallocator: *mut *mut sixel_allocator_t,
        fn_malloc: sixel_malloc_t,
        fn_calloc: sixel_calloc_t,
        fn_realloc: sixel_realloc_t,
        fn_free: sixel_free_t,
    ) -> SIXELSTATUS;
    fn sixel_allocator_ref(allocator: *mut sixel_allocator_t);
    fn sixel_allocator_unref(allocator: *mut sixel_allocator_t);
    fn sixel_allocator_malloc(
        allocator: *mut sixel_allocator_t,
        n: size_t,
    ) -> *mut libc::c_void;
    fn sixel_allocator_free(allocator: *mut sixel_allocator_t, p: *mut libc::c_void);
    fn sixel_helper_set_additional_message(message: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type SIXELSTATUS = libc::c_int;
pub type sixel_malloc_t = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type sixel_calloc_t = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type sixel_realloc_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type sixel_free_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type sixel_allocator_t = sixel_allocator;
pub type sixel_allocator_function = Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
>;
pub type image_buffer_t = image_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_buffer {
    pub data: *mut libc::c_uchar,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub palette: [libc::c_int; 256],
    pub ncolors: libc::c_int,
}
pub type parser_context_t = parser_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_context {
    pub state: parse_state_t,
    pub pos_x: libc::c_int,
    pub pos_y: libc::c_int,
    pub max_x: libc::c_int,
    pub max_y: libc::c_int,
    pub attributed_pan: libc::c_int,
    pub attributed_pad: libc::c_int,
    pub attributed_ph: libc::c_int,
    pub attributed_pv: libc::c_int,
    pub repeat_count: libc::c_int,
    pub color_index: libc::c_int,
    pub bgindex: libc::c_int,
    pub param: libc::c_int,
    pub nparams: libc::c_int,
    pub params: [libc::c_int; 16],
}
pub type parse_state_t = parse_state;
pub type parse_state = libc::c_uint;
pub const PS_DECGCI: parse_state = 6;
pub const PS_DECGRI: parse_state = 5;
pub const PS_DECGRA: parse_state = 4;
pub const PS_DECSIXEL: parse_state = 3;
pub const PS_DCS: parse_state = 2;
pub const PS_ESC: parse_state = 1;
pub const PS_GROUND: parse_state = 0;
static mut sixel_default_color_table: [libc::c_int; 16] = [
    (((0 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((0 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (0 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
            / 100 as libc::c_int,
    (((20 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((20 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((80 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((13 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (13 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((20 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (20 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((80 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((20 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((20 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((80 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (20 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((53 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((53 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (53 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((26 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((26 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (26 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((33 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((33 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (60 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((60 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((26 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (26 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((33 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((60 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (33 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((60 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((33 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (60 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((33 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((60 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (60 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((60 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((60 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (33 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
    (((80 as libc::c_int * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int)
            << 8 as libc::c_uint)
        + (80 as libc::c_int * 255 as libc::c_int
            + 100 as libc::c_int / 2 as libc::c_int) / 100 as libc::c_int,
];
unsafe extern "C" fn hls_to_rgb(
    mut hue: libc::c_int,
    mut lum: libc::c_int,
    mut sat: libc::c_int,
) -> libc::c_int {
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if sat == 0 as libc::c_int {
        b = lum;
        g = b;
        r = g;
    }
    max = lum as libc::c_double
        + sat as libc::c_double
            * (1.0f64
                - (if lum > 50 as libc::c_int {
                    (lum << 2 as libc::c_int) as libc::c_double / 100.0f64 - 1.0f64
                } else {
                    -(2 as libc::c_int as libc::c_double
                        * (lum as libc::c_double / 100.0f64) - 1.0f64)
                })) / 2.0f64;
    min = lum as libc::c_double
        - sat as libc::c_double
            * (1.0f64
                - (if lum > 50 as libc::c_int {
                    (lum << 2 as libc::c_int) as libc::c_double / 100.0f64 - 1.0f64
                } else {
                    -(2 as libc::c_int as libc::c_double
                        * (lum as libc::c_double / 100.0f64) - 1.0f64)
                })) / 2.0f64;
    hue = (hue + 240 as libc::c_int) % 360 as libc::c_int;
    match hue / 60 as libc::c_int {
        0 => {
            r = max as libc::c_int;
            g = (min + (max - min) * (hue as libc::c_double / 60.0f64)) as libc::c_int;
            b = min as libc::c_int;
        }
        1 => {
            r = (min
                + (max - min) * ((120 as libc::c_int - hue) as libc::c_double / 60.0f64))
                as libc::c_int;
            g = max as libc::c_int;
            b = min as libc::c_int;
        }
        2 => {
            r = min as libc::c_int;
            g = max as libc::c_int;
            b = (min
                + (max - min) * ((hue - 120 as libc::c_int) as libc::c_double / 60.0f64))
                as libc::c_int;
        }
        3 => {
            r = min as libc::c_int;
            g = (min
                + (max - min) * ((240 as libc::c_int - hue) as libc::c_double / 60.0f64))
                as libc::c_int;
            b = max as libc::c_int;
        }
        4 => {
            r = (min
                + (max - min) * ((hue - 240 as libc::c_int) as libc::c_double / 60.0f64))
                as libc::c_int;
            g = min as libc::c_int;
            b = max as libc::c_int;
        }
        5 => {
            r = max as libc::c_int;
            g = min as libc::c_int;
            b = (min
                + (max - min) * ((360 as libc::c_int - hue) as libc::c_double / 60.0f64))
                as libc::c_int;
        }
        _ => {
            unreachable!();
        }
    }
    return (((r * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
        / 100 as libc::c_int) << 16 as libc::c_uint)
        + (((g * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
            / 100 as libc::c_int) << 8 as libc::c_uint)
        + (b * 255 as libc::c_int + 100 as libc::c_int / 2 as libc::c_int)
            / 100 as libc::c_int;
}
unsafe extern "C" fn image_buffer_init(
    mut image: *mut image_buffer_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut bgindex: libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut n: libc::c_uint = 0;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut size: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if width <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if width > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: given width parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: given height parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else {
        size = (width as size_t)
            .wrapping_mul(height as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong);
        (*image).width = width;
        (*image).height = height;
        (*image).data = sixel_allocator_malloc(allocator, size) as *mut libc::c_uchar;
        (*image).ncolors = 2 as libc::c_int;
        if ((*image).data).is_null() {
            sixel_helper_set_additional_message(
                b"sixel_deocde_raw: sixel_allocator_malloc() failed.\0" as *const u8
                    as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
        } else {
            memset((*image).data as *mut libc::c_void, bgindex, size);
            n = 0;
            n = 0 as libc::c_int as libc::c_uint;
            while n < 16 as libc::c_int as libc::c_uint {
                (*image).palette[n as usize] = sixel_default_color_table[n as usize];
                n = n.wrapping_add(1);
                n;
            }
            r = 0 as libc::c_int;
            while r < 6 as libc::c_int {
                g = 0 as libc::c_int;
                while g < 6 as libc::c_int {
                    b = 0 as libc::c_int;
                    while b < 6 as libc::c_int {
                        let fresh0 = n;
                        n = n.wrapping_add(1);
                        (*image)
                            .palette[fresh0
                            as usize] = ((r * 42 as libc::c_int) << 16 as libc::c_uint)
                            + ((g * 42 as libc::c_int) << 8 as libc::c_uint)
                            + b * 42 as libc::c_int;
                        b += 1;
                        b;
                    }
                    g += 1;
                    g;
                }
                r += 1;
                r;
            }
            i = 0 as libc::c_int;
            while i < 24 as libc::c_int {
                let fresh1 = n;
                n = n.wrapping_add(1);
                (*image)
                    .palette[fresh1
                    as usize] = ((i * 11 as libc::c_int) << 16 as libc::c_uint)
                    + ((i * 11 as libc::c_int) << 8 as libc::c_uint)
                    + i * 11 as libc::c_int;
                i += 1;
                i;
            }
            while n < 256 as libc::c_int as libc::c_uint {
                let fresh2 = n;
                n = n.wrapping_add(1);
                (*image)
                    .palette[fresh2
                    as usize] = ((255 as libc::c_int) << 16 as libc::c_uint)
                    + ((255 as libc::c_int) << 8 as libc::c_uint) + 255 as libc::c_int;
            }
            status = 0 as libc::c_int;
        }
    }
    return status;
}
unsafe extern "C" fn image_buffer_resize(
    mut image: *mut image_buffer_t,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut bgindex: libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut size: size_t = 0;
    let mut alt_buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut n: libc::c_int = 0;
    let mut min_height: libc::c_int = 0;
    if width <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height <= 0 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: an invalid width parameter detected.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: given height parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if width > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: given width parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else if height > 1000000 as libc::c_int {
        sixel_helper_set_additional_message(
            b"image_buffer_init: given height parameter is too huge.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else {
        size = (width as size_t).wrapping_mul(height as size_t);
        alt_buffer = sixel_allocator_malloc(allocator, size) as *mut libc::c_uchar;
        if alt_buffer.is_null() || size == 0 as libc::c_int as libc::c_ulong {
            sixel_allocator_free(allocator, (*image).data as *mut libc::c_void);
            (*image).data = 0 as *mut libc::c_uchar;
            sixel_helper_set_additional_message(
                b"image_buffer_resize: sixel_allocator_malloc() failed.\0" as *const u8
                    as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
        } else {
            min_height = if height > (*image).height { (*image).height } else { height };
            if width > (*image).width {
                n = 0 as libc::c_int;
                while n < min_height {
                    memcpy(
                        alt_buffer
                            .offset((width as size_t).wrapping_mul(n as size_t) as isize)
                            as *mut libc::c_void,
                        ((*image).data)
                            .offset(
                                ((*image).width as size_t).wrapping_mul(n as size_t)
                                    as isize,
                            ) as *const libc::c_void,
                        (*image).width as size_t,
                    );
                    memset(
                        alt_buffer
                            .offset((width as size_t).wrapping_mul(n as size_t) as isize)
                            .offset((*image).width as size_t as isize)
                            as *mut libc::c_void,
                        bgindex,
                        (width - (*image).width) as size_t,
                    );
                    n += 1;
                    n;
                }
            } else {
                n = 0 as libc::c_int;
                while n < min_height {
                    memcpy(
                        alt_buffer
                            .offset((width as size_t).wrapping_mul(n as size_t) as isize)
                            as *mut libc::c_void,
                        ((*image).data)
                            .offset(
                                ((*image).width as size_t).wrapping_mul(n as size_t)
                                    as isize,
                            ) as *const libc::c_void,
                        width as size_t,
                    );
                    n += 1;
                    n;
                }
            }
            if height > (*image).height {
                memset(
                    alt_buffer
                        .offset(
                            (width as size_t).wrapping_mul((*image).height as size_t)
                                as isize,
                        ) as *mut libc::c_void,
                    bgindex,
                    (width as size_t).wrapping_mul((height - (*image).height) as size_t),
                );
            }
            sixel_allocator_free(allocator, (*image).data as *mut libc::c_void);
            (*image).data = alt_buffer;
            (*image).width = width;
            (*image).height = height;
            status = 0 as libc::c_int;
        }
    }
    return status;
}
unsafe extern "C" fn parser_context_init(
    mut context: *mut parser_context_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    (*context).state = PS_GROUND;
    (*context).pos_x = 0 as libc::c_int;
    (*context).pos_y = 0 as libc::c_int;
    (*context).max_x = 0 as libc::c_int;
    (*context).max_y = 0 as libc::c_int;
    (*context).attributed_pan = 2 as libc::c_int;
    (*context).attributed_pad = 1 as libc::c_int;
    (*context).attributed_ph = 0 as libc::c_int;
    (*context).attributed_pv = 0 as libc::c_int;
    (*context).repeat_count = 1 as libc::c_int;
    (*context).color_index = 15 as libc::c_int;
    (*context).bgindex = -(1 as libc::c_int);
    (*context).nparams = 0 as libc::c_int;
    (*context).param = 0 as libc::c_int;
    status = 0 as libc::c_int;
    return status;
}
unsafe extern "C" fn safe_addition_for_params(
    mut context: *mut parser_context_t,
    mut p: *mut libc::c_uchar,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut x: libc::c_int = 0;
    x = *p as libc::c_int - '0' as i32;
    if (*context).param > 2147483647 as libc::c_int / 10 as libc::c_int
        || x > 2147483647 as libc::c_int - (*context).param * 10 as libc::c_int
    {
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x4 as libc::c_int;
        sixel_helper_set_additional_message(
            b"safe_addition_for_params: ingeger overflow detected.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        (*context).param = (*context).param * 10 as libc::c_int + x;
        status = 0 as libc::c_int;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decode_raw_impl(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_int,
    mut image: *mut image_buffer_t,
    mut context: *mut parser_context_t,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut sixel_vertical_mask: libc::c_int = 0;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut pos: size_t = 0;
    let mut p0: *mut libc::c_uchar = p;
    loop {
        if !(p < p0.offset(len as isize)) {
            current_block = 12680962103047333219;
            break;
        }
        match (*context).state as libc::c_uint {
            0 => {
                match *p as libc::c_int {
                    27 => {
                        (*context).state = PS_ESC;
                        p = p.offset(1);
                        p;
                    }
                    144 => {
                        (*context).state = PS_DCS;
                        p = p.offset(1);
                        p;
                    }
                    156 => {
                        p = p.offset(1);
                        p;
                        current_block = 12680962103047333219;
                        break;
                    }
                    _ => {
                        p = p.offset(1);
                        p;
                    }
                }
            }
            1 => {
                match *p as libc::c_int {
                    92 | 156 => {
                        p = p.offset(1);
                        p;
                        current_block = 12680962103047333219;
                        break;
                    }
                    80 => {
                        (*context).param = -(1 as libc::c_int);
                        (*context).state = PS_DCS;
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        p = p.offset(1);
                        p;
                    }
                }
            }
            2 => {
                match *p as libc::c_int {
                    27 => {
                        (*context).state = PS_ESC;
                        p = p.offset(1);
                        p;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        if (*context).param < 0 as libc::c_int {
                            (*context).param = 0 as libc::c_int;
                        }
                        status = safe_addition_for_params(context, p);
                        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                            current_block = 7407205790562272386;
                            break;
                        }
                        p = p.offset(1);
                        p;
                    }
                    59 => {
                        if (*context).param < 0 as libc::c_int {
                            (*context).param = 0 as libc::c_int;
                        }
                        if (*context).nparams < 16 as libc::c_int {
                            let fresh3 = (*context).nparams;
                            (*context).nparams = (*context).nparams + 1;
                            (*context).params[fresh3 as usize] = (*context).param;
                        }
                        (*context).param = 0 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    113 => {
                        if (*context).param >= 0 as libc::c_int
                            && (*context).nparams < 16 as libc::c_int
                        {
                            let fresh4 = (*context).nparams;
                            (*context).nparams = (*context).nparams + 1;
                            (*context).params[fresh4 as usize] = (*context).param;
                        }
                        if (*context).nparams > 0 as libc::c_int {
                            match (*context).params[0 as libc::c_int as usize] {
                                0 | 1 => {
                                    (*context).attributed_pad = 2 as libc::c_int;
                                }
                                2 => {
                                    (*context).attributed_pad = 5 as libc::c_int;
                                }
                                3 | 4 => {
                                    (*context).attributed_pad = 4 as libc::c_int;
                                }
                                5 | 6 => {
                                    (*context).attributed_pad = 3 as libc::c_int;
                                }
                                7 | 8 => {
                                    (*context).attributed_pad = 2 as libc::c_int;
                                }
                                9 => {
                                    (*context).attributed_pad = 1 as libc::c_int;
                                }
                                _ => {
                                    (*context).attributed_pad = 2 as libc::c_int;
                                }
                            }
                        }
                        if (*context).nparams > 2 as libc::c_int {
                            if (*context).params[2 as libc::c_int as usize]
                                == 0 as libc::c_int
                            {
                                (*context)
                                    .params[2 as libc::c_int as usize] = 10 as libc::c_int;
                            }
                            (*context)
                                .attributed_pan = (*context).attributed_pan
                                * (*context).params[2 as libc::c_int as usize]
                                / 10 as libc::c_int;
                            (*context)
                                .attributed_pad = (*context).attributed_pad
                                * (*context).params[2 as libc::c_int as usize]
                                / 10 as libc::c_int;
                            if (*context).attributed_pan <= 0 as libc::c_int {
                                (*context).attributed_pan = 1 as libc::c_int;
                            }
                            if (*context).attributed_pad <= 0 as libc::c_int {
                                (*context).attributed_pad = 1 as libc::c_int;
                            }
                        }
                        (*context).nparams = 0 as libc::c_int;
                        (*context).state = PS_DECSIXEL;
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        p = p.offset(1);
                        p;
                    }
                }
            }
            3 => {
                match *p as libc::c_int {
                    27 => {
                        (*context).state = PS_ESC;
                        p = p.offset(1);
                        p;
                    }
                    34 => {
                        (*context).param = 0 as libc::c_int;
                        (*context).nparams = 0 as libc::c_int;
                        (*context).state = PS_DECGRA;
                        p = p.offset(1);
                        p;
                    }
                    33 => {
                        (*context).param = 0 as libc::c_int;
                        (*context).nparams = 0 as libc::c_int;
                        (*context).state = PS_DECGRI;
                        p = p.offset(1);
                        p;
                    }
                    35 => {
                        (*context).param = 0 as libc::c_int;
                        (*context).nparams = 0 as libc::c_int;
                        (*context).state = PS_DECGCI;
                        p = p.offset(1);
                        p;
                    }
                    36 => {
                        (*context).pos_x = 0 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    45 => {
                        (*context).pos_x = 0 as libc::c_int;
                        (*context).pos_y += 6 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        if *p as libc::c_int >= '?' as i32
                            && *p as libc::c_int <= '~' as i32
                        {
                            sx = (*image).width;
                            while sx < (*context).pos_x + (*context).repeat_count {
                                sx *= 2 as libc::c_int;
                            }
                            sy = (*image).height;
                            while sy < (*context).pos_y + 6 as libc::c_int {
                                sy *= 2 as libc::c_int;
                            }
                            if sx > (*image).width || sy > (*image).height {
                                status = image_buffer_resize(
                                    image,
                                    sx,
                                    sy,
                                    (*context).bgindex,
                                    allocator,
                                );
                                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                                    current_block = 7407205790562272386;
                                    break;
                                }
                            }
                            if (*context).color_index > (*image).ncolors {
                                (*image).ncolors = (*context).color_index;
                            }
                            if (*context).pos_x < 0 as libc::c_int
                                || (*context).pos_y < 0 as libc::c_int
                            {
                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                    | 0x3 as libc::c_int;
                                current_block = 7407205790562272386;
                                break;
                            } else {
                                bits = *p as libc::c_int - '?' as i32;
                                if bits == 0 as libc::c_int {
                                    (*context).pos_x += (*context).repeat_count;
                                } else {
                                    sixel_vertical_mask = 0x1 as libc::c_int;
                                    if (*context).repeat_count <= 1 as libc::c_int {
                                        i = 0 as libc::c_int;
                                        while i < 6 as libc::c_int {
                                            if bits & sixel_vertical_mask != 0 as libc::c_int {
                                                pos = ((*image).width as size_t)
                                                    .wrapping_mul(((*context).pos_y + i) as size_t)
                                                    .wrapping_add((*context).pos_x as size_t);
                                                *((*image).data)
                                                    .offset(
                                                        pos as isize,
                                                    ) = (*context).color_index as libc::c_uchar;
                                                if (*context).max_x < (*context).pos_x {
                                                    (*context).max_x = (*context).pos_x;
                                                }
                                                if (*context).max_y < (*context).pos_y + i {
                                                    (*context).max_y = (*context).pos_y + i;
                                                }
                                            }
                                            sixel_vertical_mask <<= 1 as libc::c_int;
                                            i += 1;
                                            i;
                                        }
                                        (*context).pos_x += 1 as libc::c_int;
                                    } else {
                                        i = 0 as libc::c_int;
                                        while i < 6 as libc::c_int {
                                            if bits & sixel_vertical_mask != 0 as libc::c_int {
                                                c = sixel_vertical_mask << 1 as libc::c_int;
                                                n = 1 as libc::c_int;
                                                while i + n < 6 as libc::c_int {
                                                    if bits & c == 0 as libc::c_int {
                                                        break;
                                                    }
                                                    c <<= 1 as libc::c_int;
                                                    n += 1;
                                                    n;
                                                }
                                                y = (*context).pos_y + i;
                                                while y < (*context).pos_y + i + n {
                                                    memset(
                                                        ((*image).data)
                                                            .offset(
                                                                ((*image).width as size_t).wrapping_mul(y as size_t)
                                                                    as isize,
                                                            )
                                                            .offset((*context).pos_x as size_t as isize)
                                                            as *mut libc::c_void,
                                                        (*context).color_index,
                                                        (*context).repeat_count as size_t,
                                                    );
                                                    y += 1;
                                                    y;
                                                }
                                                if (*context).max_x
                                                    < (*context).pos_x + (*context).repeat_count
                                                        - 1 as libc::c_int
                                                {
                                                    (*context)
                                                        .max_x = (*context).pos_x + (*context).repeat_count
                                                        - 1 as libc::c_int;
                                                }
                                                if (*context).max_y
                                                    < (*context).pos_y + i + n - 1 as libc::c_int
                                                {
                                                    (*context)
                                                        .max_y = (*context).pos_y + i + n - 1 as libc::c_int;
                                                }
                                                i += n - 1 as libc::c_int;
                                                sixel_vertical_mask <<= n - 1 as libc::c_int;
                                            }
                                            sixel_vertical_mask <<= 1 as libc::c_int;
                                            i += 1;
                                            i;
                                        }
                                        (*context).pos_x += (*context).repeat_count;
                                    }
                                }
                                (*context).repeat_count = 1 as libc::c_int;
                            }
                        }
                        p = p.offset(1);
                        p;
                    }
                }
            }
            4 => {
                match *p as libc::c_int {
                    27 => {
                        (*context).state = PS_ESC;
                        p = p.offset(1);
                        p;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        status = safe_addition_for_params(context, p);
                        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                            current_block = 7407205790562272386;
                            break;
                        }
                        p = p.offset(1);
                        p;
                    }
                    59 => {
                        if (*context).nparams < 16 as libc::c_int {
                            let fresh5 = (*context).nparams;
                            (*context).nparams = (*context).nparams + 1;
                            (*context).params[fresh5 as usize] = (*context).param;
                        }
                        (*context).param = 0 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        if (*context).nparams < 16 as libc::c_int {
                            let fresh6 = (*context).nparams;
                            (*context).nparams = (*context).nparams + 1;
                            (*context).params[fresh6 as usize] = (*context).param;
                        }
                        if (*context).nparams > 0 as libc::c_int {
                            (*context)
                                .attributed_pad = (*context)
                                .params[0 as libc::c_int as usize];
                        }
                        if (*context).nparams > 1 as libc::c_int {
                            (*context)
                                .attributed_pan = (*context)
                                .params[1 as libc::c_int as usize];
                        }
                        if (*context).nparams > 2 as libc::c_int
                            && (*context).params[2 as libc::c_int as usize]
                                > 0 as libc::c_int
                        {
                            (*context)
                                .attributed_ph = (*context)
                                .params[2 as libc::c_int as usize];
                        }
                        if (*context).nparams > 3 as libc::c_int
                            && (*context).params[3 as libc::c_int as usize]
                                > 0 as libc::c_int
                        {
                            (*context)
                                .attributed_pv = (*context)
                                .params[3 as libc::c_int as usize];
                        }
                        if (*context).attributed_pan <= 0 as libc::c_int {
                            (*context).attributed_pan = 1 as libc::c_int;
                        }
                        if (*context).attributed_pad <= 0 as libc::c_int {
                            (*context).attributed_pad = 1 as libc::c_int;
                        }
                        if (*image).width < (*context).attributed_ph
                            || (*image).height < (*context).attributed_pv
                        {
                            sx = (*context).attributed_ph;
                            if (*image).width > (*context).attributed_ph {
                                sx = (*image).width;
                            }
                            sy = (*context).attributed_pv;
                            if (*image).height > (*context).attributed_pv {
                                sy = (*image).height;
                            }
                            status = image_buffer_resize(
                                image,
                                sx,
                                sy,
                                (*context).bgindex,
                                allocator,
                            );
                            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                                current_block = 7407205790562272386;
                                break;
                            }
                        }
                        (*context).state = PS_DECSIXEL;
                        (*context).param = 0 as libc::c_int;
                        (*context).nparams = 0 as libc::c_int;
                    }
                }
            }
            5 => {
                match *p as libc::c_int {
                    27 => {
                        (*context).state = PS_ESC;
                        p = p.offset(1);
                        p;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        status = safe_addition_for_params(context, p);
                        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                            current_block = 7407205790562272386;
                            break;
                        }
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        (*context).repeat_count = (*context).param;
                        if (*context).repeat_count == 0 as libc::c_int {
                            (*context).repeat_count = 1 as libc::c_int;
                        }
                        if (*context).repeat_count > 0xffff as libc::c_int {
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                | 0x3 as libc::c_int;
                            sixel_helper_set_additional_message(
                                b"sixel_decode_raw_impl: detected too huge repeat parameter.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            current_block = 7407205790562272386;
                            break;
                        } else {
                            (*context).state = PS_DECSIXEL;
                            (*context).param = 0 as libc::c_int;
                            (*context).nparams = 0 as libc::c_int;
                        }
                    }
                }
            }
            6 => {
                match *p as libc::c_int {
                    27 => {
                        (*context).state = PS_ESC;
                        p = p.offset(1);
                        p;
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        status = safe_addition_for_params(context, p);
                        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                            current_block = 7407205790562272386;
                            break;
                        }
                        p = p.offset(1);
                        p;
                    }
                    59 => {
                        if (*context).nparams < 16 as libc::c_int {
                            let fresh7 = (*context).nparams;
                            (*context).nparams = (*context).nparams + 1;
                            (*context).params[fresh7 as usize] = (*context).param;
                        }
                        (*context).param = 0 as libc::c_int;
                        p = p.offset(1);
                        p;
                    }
                    _ => {
                        (*context).state = PS_DECSIXEL;
                        if (*context).nparams < 16 as libc::c_int {
                            let fresh8 = (*context).nparams;
                            (*context).nparams = (*context).nparams + 1;
                            (*context).params[fresh8 as usize] = (*context).param;
                        }
                        (*context).param = 0 as libc::c_int;
                        if (*context).nparams > 0 as libc::c_int {
                            (*context)
                                .color_index = (*context).params[0 as libc::c_int as usize];
                            if (*context).color_index < 0 as libc::c_int {
                                (*context).color_index = 0 as libc::c_int;
                            } else if (*context).color_index >= 256 as libc::c_int {
                                (*context)
                                    .color_index = 256 as libc::c_int - 1 as libc::c_int;
                            }
                        }
                        if (*context).nparams > 4 as libc::c_int {
                            if (*context).params[1 as libc::c_int as usize]
                                == 1 as libc::c_int
                            {
                                if (*context).params[2 as libc::c_int as usize]
                                    > 360 as libc::c_int
                                {
                                    (*context)
                                        .params[2 as libc::c_int as usize] = 360 as libc::c_int;
                                }
                                if (*context).params[3 as libc::c_int as usize]
                                    > 100 as libc::c_int
                                {
                                    (*context)
                                        .params[3 as libc::c_int as usize] = 100 as libc::c_int;
                                }
                                if (*context).params[4 as libc::c_int as usize]
                                    > 100 as libc::c_int
                                {
                                    (*context)
                                        .params[4 as libc::c_int as usize] = 100 as libc::c_int;
                                }
                                (*image)
                                    .palette[(*context).color_index
                                    as usize] = hls_to_rgb(
                                    (*context).params[2 as libc::c_int as usize],
                                    (*context).params[3 as libc::c_int as usize],
                                    (*context).params[4 as libc::c_int as usize],
                                );
                            } else if (*context).params[1 as libc::c_int as usize]
                                == 2 as libc::c_int
                            {
                                if (*context).params[2 as libc::c_int as usize]
                                    > 100 as libc::c_int
                                {
                                    (*context)
                                        .params[2 as libc::c_int as usize] = 100 as libc::c_int;
                                }
                                if (*context).params[3 as libc::c_int as usize]
                                    > 100 as libc::c_int
                                {
                                    (*context)
                                        .params[3 as libc::c_int as usize] = 100 as libc::c_int;
                                }
                                if (*context).params[4 as libc::c_int as usize]
                                    > 100 as libc::c_int
                                {
                                    (*context)
                                        .params[4 as libc::c_int as usize] = 100 as libc::c_int;
                                }
                                (*image)
                                    .palette[(*context).color_index
                                    as usize] = ((((*context).params[2 as libc::c_int as usize]
                                    * 255 as libc::c_int
                                    + 100 as libc::c_int / 2 as libc::c_int)
                                    / 100 as libc::c_int) << 16 as libc::c_uint)
                                    + ((((*context).params[3 as libc::c_int as usize]
                                        * 255 as libc::c_int
                                        + 100 as libc::c_int / 2 as libc::c_int)
                                        / 100 as libc::c_int) << 8 as libc::c_uint)
                                    + ((*context).params[4 as libc::c_int as usize]
                                        * 255 as libc::c_int
                                        + 100 as libc::c_int / 2 as libc::c_int)
                                        / 100 as libc::c_int;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    match current_block {
        12680962103047333219 => {
            (*context).max_x += 1;
            if (*context).max_x < (*context).attributed_ph {
                (*context).max_x = (*context).attributed_ph;
            }
            (*context).max_y += 1;
            if (*context).max_y < (*context).attributed_pv {
                (*context).max_y = (*context).attributed_pv;
            }
            if (*image).width > (*context).max_x || (*image).height > (*context).max_y {
                status = image_buffer_resize(
                    image,
                    (*context).max_x,
                    (*context).max_y,
                    (*context).bgindex,
                    allocator,
                );
                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                    current_block = 7407205790562272386;
                } else {
                    current_block = 219164688580890164;
                }
            } else {
                current_block = 219164688580890164;
            }
            match current_block {
                7407205790562272386 => {}
                _ => {
                    status = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decode_raw(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_int,
    mut pixels: *mut *mut libc::c_uchar,
    mut pwidth: *mut libc::c_int,
    mut pheight: *mut libc::c_int,
    mut palette: *mut *mut libc::c_uchar,
    mut ncolors: *mut libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut alloc_size: libc::c_int = 0;
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut context: parser_context_t = parser_context_t {
        state: PS_GROUND,
        pos_x: 0,
        pos_y: 0,
        max_x: 0,
        max_y: 0,
        attributed_pan: 0,
        attributed_pad: 0,
        attributed_ph: 0,
        attributed_pv: 0,
        repeat_count: 0,
        color_index: 0,
        bgindex: 0,
        param: 0,
        nparams: 0,
        params: [0; 16],
    };
    let mut image: image_buffer_t = image_buffer_t {
        data: 0 as *mut libc::c_uchar,
        width: 0,
        height: 0,
        palette: [0; 256],
        ncolors: 0,
    };
    let mut n: libc::c_int = 0;
    image.data = 0 as *mut libc::c_uchar;
    if !allocator.is_null() {
        sixel_allocator_ref(allocator);
        current_block = 7815301370352969686;
    } else {
        status = sixel_allocator_new(&mut allocator, None, None, None, None);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            allocator = 0 as *mut sixel_allocator_t;
            current_block = 2712539247478501854;
        } else {
            current_block = 7815301370352969686;
        }
    }
    match current_block {
        7815301370352969686 => {
            status = parser_context_init(&mut context);
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                current_block = 2712539247478501854;
            } else {
                status = image_buffer_init(
                    &mut image,
                    1 as libc::c_int,
                    1 as libc::c_int,
                    context.bgindex,
                    allocator,
                );
                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                    current_block = 2712539247478501854;
                } else {
                    status = sixel_decode_raw_impl(
                        p,
                        len,
                        &mut image,
                        &mut context,
                        allocator,
                    );
                    if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                        current_block = 2712539247478501854;
                    } else {
                        *ncolors = image.ncolors + 1 as libc::c_int;
                        alloc_size = *ncolors;
                        if alloc_size < 256 as libc::c_int {
                            alloc_size = 256 as libc::c_int;
                        }
                        *palette = sixel_allocator_malloc(
                            allocator,
                            (alloc_size * 3 as libc::c_int) as size_t,
                        ) as *mut libc::c_uchar;
                        if palette.is_null() {
                            sixel_allocator_free(
                                allocator,
                                image.data as *mut libc::c_void,
                            );
                            sixel_helper_set_additional_message(
                                b"sixel_deocde_raw: sixel_allocator_malloc() failed.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                | 0x1 as libc::c_int;
                            current_block = 2712539247478501854;
                        } else {
                            n = 0 as libc::c_int;
                            while n < *ncolors {
                                *(*palette)
                                    .offset(
                                        (n * 3 as libc::c_int + 0 as libc::c_int) as isize,
                                    ) = (image.palette[n as usize] >> 16 as libc::c_int
                                    & 0xff as libc::c_int) as libc::c_uchar;
                                *(*palette)
                                    .offset(
                                        (n * 3 as libc::c_int + 1 as libc::c_int) as isize,
                                    ) = (image.palette[n as usize] >> 8 as libc::c_int
                                    & 0xff as libc::c_int) as libc::c_uchar;
                                *(*palette)
                                    .offset(
                                        (n * 3 as libc::c_int + 2 as libc::c_int) as isize,
                                    ) = (image.palette[n as usize] & 0xff as libc::c_int)
                                    as libc::c_uchar;
                                n += 1;
                                n;
                            }
                            *pwidth = image.width;
                            *pheight = image.height;
                            *pixels = image.data;
                            status = 0 as libc::c_int;
                            current_block = 13708889583811487480;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        2712539247478501854 => {
            free(image.data as *mut libc::c_void);
            image.data = 0 as *mut libc::c_uchar;
        }
        _ => {}
    }
    sixel_allocator_unref(allocator);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decode(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_int,
    mut pixels: *mut *mut libc::c_uchar,
    mut pwidth: *mut libc::c_int,
    mut pheight: *mut libc::c_int,
    mut palette: *mut *mut libc::c_uchar,
    mut ncolors: *mut libc::c_int,
    mut fn_malloc: sixel_allocator_function,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut allocator: *mut sixel_allocator_t = 0 as *mut sixel_allocator_t;
    let mut context: parser_context_t = parser_context_t {
        state: PS_GROUND,
        pos_x: 0,
        pos_y: 0,
        max_x: 0,
        max_y: 0,
        attributed_pan: 0,
        attributed_pad: 0,
        attributed_ph: 0,
        attributed_pv: 0,
        repeat_count: 0,
        color_index: 0,
        bgindex: 0,
        param: 0,
        nparams: 0,
        params: [0; 16],
    };
    let mut image: image_buffer_t = image_buffer_t {
        data: 0 as *mut libc::c_uchar,
        width: 0,
        height: 0,
        palette: [0; 256],
        ncolors: 0,
    };
    let mut n: libc::c_int = 0;
    status = sixel_allocator_new(&mut allocator, fn_malloc, None, None, None);
    if status & 0x1000 as libc::c_int != 0 as libc::c_int {
        allocator = 0 as *mut sixel_allocator_t;
    } else {
        status = parser_context_init(&mut context);
        if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
            status = image_buffer_init(
                &mut image,
                2048 as libc::c_int,
                2048 as libc::c_int,
                context.bgindex,
                allocator,
            );
            if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                status = sixel_decode_raw_impl(
                    p,
                    len,
                    &mut image,
                    &mut context,
                    allocator,
                );
                if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                    *ncolors = image.ncolors + 1 as libc::c_int;
                    *palette = sixel_allocator_malloc(
                        allocator,
                        (*ncolors * 3 as libc::c_int) as size_t,
                    ) as *mut libc::c_uchar;
                    if palette.is_null() {
                        sixel_allocator_free(allocator, image.data as *mut libc::c_void);
                        sixel_helper_set_additional_message(
                            b"sixel_deocde_raw: sixel_allocator_malloc() failed.\0"
                                as *const u8 as *const libc::c_char,
                        );
                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                            | 0x1 as libc::c_int;
                    } else {
                        n = 0 as libc::c_int;
                        while n < *ncolors {
                            *(*palette)
                                .offset(
                                    (n * 3 as libc::c_int + 0 as libc::c_int) as isize,
                                ) = (image.palette[n as usize] >> 16 as libc::c_int
                                & 0xff as libc::c_int) as libc::c_uchar;
                            *(*palette)
                                .offset(
                                    (n * 3 as libc::c_int + 1 as libc::c_int) as isize,
                                ) = (image.palette[n as usize] >> 8 as libc::c_int
                                & 0xff as libc::c_int) as libc::c_uchar;
                            *(*palette)
                                .offset(
                                    (n * 3 as libc::c_int + 2 as libc::c_int) as isize,
                                ) = (image.palette[n as usize] & 0xff as libc::c_int)
                                as libc::c_uchar;
                            n += 1;
                            n;
                        }
                        *pwidth = image.width;
                        *pheight = image.height;
                        *pixels = image.data;
                        status = 0 as libc::c_int;
                    }
                }
            }
        }
    }
    sixel_allocator_unref(allocator);
    return status;
}
