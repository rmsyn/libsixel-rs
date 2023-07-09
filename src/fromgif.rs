#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type sixel_allocator;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sixel_allocator_malloc(
        allocator: *mut sixel_allocator_t,
        n: size_t,
    ) -> *mut libc::c_void;
    fn sixel_allocator_free(allocator: *mut sixel_allocator_t, p: *mut libc::c_void);
    fn sixel_helper_set_additional_message(message: *const libc::c_char);
    fn sixel_frame_new(
        ppframe: *mut *mut sixel_frame_t,
        allocator: *mut sixel_allocator_t,
    ) -> SIXELSTATUS;
    fn sixel_frame_unref(frame: *mut sixel_frame_t);
}
pub type size_t = libc::c_ulong;
pub type SIXELSTATUS = libc::c_int;
pub type sixel_allocator_t = sixel_allocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sixel_frame {
    pub ref_0: libc::c_uint,
    pub pixels: *mut libc::c_uchar,
    pub palette: *mut libc::c_uchar,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub ncolors: libc::c_int,
    pub pixelformat: libc::c_int,
    pub delay: libc::c_int,
    pub frame_no: libc::c_int,
    pub loop_count: libc::c_int,
    pub multiframe: libc::c_int,
    pub transparent: libc::c_int,
    pub allocator: *mut sixel_allocator_t,
}
pub type sixel_frame_t = sixel_frame;
pub type sixel_load_image_function = Option::<
    unsafe extern "C" fn(*mut sixel_frame_t, *mut libc::c_void) -> SIXELSTATUS,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gif_t {
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub out: *mut libc::c_uchar,
    pub flags: libc::c_int,
    pub bgindex: libc::c_int,
    pub ratio: libc::c_int,
    pub transparent: libc::c_int,
    pub eflags: libc::c_int,
    pub pal: [[libc::c_uchar; 3]; 256],
    pub lpal: [[libc::c_uchar; 3]; 256],
    pub codes: [gif_lzw; 4096],
    pub color_table: *mut libc::c_uchar,
    pub parse: libc::c_int,
    pub step: libc::c_int,
    pub lflags: libc::c_int,
    pub start_x: libc::c_int,
    pub start_y: libc::c_int,
    pub max_x: libc::c_int,
    pub max_y: libc::c_int,
    pub cur_x: libc::c_int,
    pub cur_y: libc::c_int,
    pub actual_width: libc::c_int,
    pub actual_height: libc::c_int,
    pub line_size: libc::c_int,
    pub loop_count: libc::c_int,
    pub delay: libc::c_int,
    pub is_multiframe: libc::c_int,
    pub is_terminated: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gif_lzw {
    pub prefix: libc::c_short,
    pub first: libc::c_uchar,
    pub suffix: libc::c_uchar,
}
pub type fn_pointer = _fn_pointer;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _fn_pointer {
    pub fn_0: sixel_load_image_function,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gif_context_t {
    pub img_x: libc::c_uint,
    pub img_y: libc::c_uint,
    pub img_n: libc::c_int,
    pub img_out_n: libc::c_int,
    pub buflen: libc::c_int,
    pub buffer_start: [libc::c_uchar; 128],
    pub img_buffer: *mut libc::c_uchar,
    pub img_buffer_end: *mut libc::c_uchar,
    pub img_buffer_original: *mut libc::c_uchar,
}
unsafe extern "C" fn gif_get8(mut s: *mut gif_context_t) -> libc::c_uchar {
    if (*s).img_buffer < (*s).img_buffer_end {
        let fresh0 = (*s).img_buffer;
        (*s).img_buffer = ((*s).img_buffer).offset(1);
        return *fresh0;
    }
    return 0 as libc::c_int as libc::c_uchar;
}
unsafe extern "C" fn gif_get16le(mut s: *mut gif_context_t) -> libc::c_int {
    let mut z: libc::c_int = gif_get8(s) as libc::c_int;
    return z + ((gif_get8(s) as libc::c_int) << 8 as libc::c_int);
}
unsafe extern "C" fn gif_parse_colortable(
    mut s: *mut gif_context_t,
    mut pal: *mut [libc::c_uchar; 3],
    mut num_entries: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < num_entries {
        (*pal.offset(i as isize))[2 as libc::c_int as usize] = gif_get8(s);
        (*pal.offset(i as isize))[1 as libc::c_int as usize] = gif_get8(s);
        (*pal.offset(i as isize))[0 as libc::c_int as usize] = gif_get8(s);
        i += 1;
        i;
    }
}
unsafe extern "C" fn gif_load_header(
    mut s: *mut gif_context_t,
    mut g: *mut gif_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut version: libc::c_uchar = 0;
    if !(gif_get8(s) as libc::c_int != 'G' as i32) {
        if !(gif_get8(s) as libc::c_int != 'I' as i32) {
            if !(gif_get8(s) as libc::c_int != 'F' as i32) {
                if !(gif_get8(s) as libc::c_int != '8' as i32) {
                    version = gif_get8(s);
                    if !(version as libc::c_int != '7' as i32
                        && version as libc::c_int != '9' as i32)
                    {
                        if !(gif_get8(s) as libc::c_int != 'a' as i32) {
                            (*g).w = gif_get16le(s);
                            (*g).h = gif_get16le(s);
                            (*g).flags = gif_get8(s) as libc::c_int;
                            (*g).bgindex = gif_get8(s) as libc::c_int;
                            (*g).ratio = gif_get8(s) as libc::c_int;
                            (*g).transparent = -(1 as libc::c_int);
                            (*g).loop_count = -(1 as libc::c_int);
                            if (*g).flags & 0x80 as libc::c_int != 0 {
                                gif_parse_colortable(
                                    s,
                                    ((*g).pal).as_mut_ptr(),
                                    (2 as libc::c_int) << ((*g).flags & 7 as libc::c_int),
                                );
                            }
                            status = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    return status;
}
unsafe extern "C" fn gif_init_frame(
    mut frame: *mut sixel_frame_t,
    mut pg: *mut gif_t,
    mut bgcolor: *mut libc::c_uchar,
    mut reqcolors: libc::c_int,
    mut fuse_palette: libc::c_int,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut ncolors: libc::c_int = 0;
    let mut palette_size: size_t = 0;
    let mut frame_size: size_t = 0;
    (*frame).delay = (*pg).delay;
    ncolors = (2 as libc::c_int)
        << ((if (*pg).lflags & 0x80 as libc::c_int != 0 {
            (*pg).lflags
        } else {
            (*pg).flags
        }) & 7 as libc::c_int);
    palette_size = (ncolors as size_t).wrapping_mul(3 as libc::c_int as libc::c_ulong);
    if ((*frame).palette).is_null() {
        (*frame)
            .palette = sixel_allocator_malloc((*frame).allocator, palette_size)
            as *mut libc::c_uchar;
    } else if (*frame).ncolors < ncolors {
        sixel_allocator_free((*frame).allocator, (*frame).palette as *mut libc::c_void);
        (*frame)
            .palette = sixel_allocator_malloc((*frame).allocator, palette_size)
            as *mut libc::c_uchar;
    }
    if ((*frame).palette).is_null() {
        sixel_helper_set_additional_message(
            b"gif_init_frame: sixel_allocator_malloc() failed.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
    } else {
        (*frame).ncolors = ncolors;
        if (*frame).ncolors <= reqcolors && fuse_palette != 0 {
            (*frame)
                .pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                | 0x3 as libc::c_int;
            sixel_allocator_free(
                (*frame).allocator,
                (*frame).pixels as *mut libc::c_void,
            );
            frame_size = ((*frame).width as size_t)
                .wrapping_mul((*frame).height as size_t);
            (*frame)
                .pixels = sixel_allocator_malloc((*frame).allocator, frame_size)
                as *mut libc::c_uchar;
            if ((*frame).pixels).is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_allocator_malloc() failed in gif_init_frame().\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 9746649557726750241;
            } else {
                memcpy(
                    (*frame).pixels as *mut libc::c_void,
                    (*pg).out as *const libc::c_void,
                    frame_size,
                );
                i = 0 as libc::c_int;
                while i < (*frame).ncolors {
                    *((*frame).palette)
                        .offset(
                            (i * 3 as libc::c_int + 0 as libc::c_int) as isize,
                        ) = *((*pg).color_table)
                        .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize);
                    *((*frame).palette)
                        .offset(
                            (i * 3 as libc::c_int + 1 as libc::c_int) as isize,
                        ) = *((*pg).color_table)
                        .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize);
                    *((*frame).palette)
                        .offset(
                            (i * 3 as libc::c_int + 2 as libc::c_int) as isize,
                        ) = *((*pg).color_table)
                        .offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize);
                    i += 1;
                    i;
                }
                if (*pg).lflags & 0x80 as libc::c_int != 0 {
                    if (*pg).eflags & 0x1 as libc::c_int != 0 {
                        if !bgcolor.is_null() {
                            *((*frame).palette)
                                .offset(
                                    ((*pg).transparent * 3 as libc::c_int + 0 as libc::c_int)
                                        as isize,
                                ) = *bgcolor.offset(0 as libc::c_int as isize);
                            *((*frame).palette)
                                .offset(
                                    ((*pg).transparent * 3 as libc::c_int + 1 as libc::c_int)
                                        as isize,
                                ) = *bgcolor.offset(1 as libc::c_int as isize);
                            *((*frame).palette)
                                .offset(
                                    ((*pg).transparent * 3 as libc::c_int + 2 as libc::c_int)
                                        as isize,
                                ) = *bgcolor.offset(2 as libc::c_int as isize);
                        } else {
                            (*frame).transparent = (*pg).transparent;
                        }
                    }
                } else if (*pg).flags & 0x80 as libc::c_int != 0 {
                    if (*pg).eflags & 0x1 as libc::c_int != 0 {
                        if !bgcolor.is_null() {
                            *((*frame).palette)
                                .offset(
                                    ((*pg).transparent * 3 as libc::c_int + 0 as libc::c_int)
                                        as isize,
                                ) = *bgcolor.offset(0 as libc::c_int as isize);
                            *((*frame).palette)
                                .offset(
                                    ((*pg).transparent * 3 as libc::c_int + 1 as libc::c_int)
                                        as isize,
                                ) = *bgcolor.offset(1 as libc::c_int as isize);
                            *((*frame).palette)
                                .offset(
                                    ((*pg).transparent * 3 as libc::c_int + 2 as libc::c_int)
                                        as isize,
                                ) = *bgcolor.offset(2 as libc::c_int as isize);
                        } else {
                            (*frame).transparent = (*pg).transparent;
                        }
                    }
                }
                current_block = 10095721787123848864;
            }
        } else {
            (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            frame_size = ((*pg).w as size_t)
                .wrapping_mul((*pg).h as size_t)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong);
            (*frame)
                .pixels = sixel_allocator_malloc((*frame).allocator, frame_size)
                as *mut libc::c_uchar;
            if ((*frame).pixels).is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_allocator_malloc() failed in gif_init_frame().\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 9746649557726750241;
            } else {
                i = 0 as libc::c_int;
                while i < (*pg).w * (*pg).h {
                    *((*frame).pixels)
                        .offset(
                            (i * 3 as libc::c_int + 0 as libc::c_int) as isize,
                        ) = *((*pg).color_table)
                        .offset(
                            (*((*pg).out).offset(i as isize) as libc::c_int
                                * 3 as libc::c_int + 2 as libc::c_int) as isize,
                        );
                    *((*frame).pixels)
                        .offset(
                            (i * 3 as libc::c_int + 1 as libc::c_int) as isize,
                        ) = *((*pg).color_table)
                        .offset(
                            (*((*pg).out).offset(i as isize) as libc::c_int
                                * 3 as libc::c_int + 1 as libc::c_int) as isize,
                        );
                    *((*frame).pixels)
                        .offset(
                            (i * 3 as libc::c_int + 2 as libc::c_int) as isize,
                        ) = *((*pg).color_table)
                        .offset(
                            (*((*pg).out).offset(i as isize) as libc::c_int
                                * 3 as libc::c_int + 0 as libc::c_int) as isize,
                        );
                    i += 1;
                    i;
                }
                current_block = 10095721787123848864;
            }
        }
        match current_block {
            9746649557726750241 => {}
            _ => {
                (*frame)
                    .multiframe = ((*pg).loop_count != -(1 as libc::c_int))
                    as libc::c_int;
                status = 0 as libc::c_int;
            }
        }
    }
    return status;
}
unsafe extern "C" fn gif_out_code(
    mut g: *mut gif_t,
    mut code: libc::c_ushort,
) -> SIXELSTATUS {
    if code as libc::c_int > 0xfff as libc::c_int {
        sixel_helper_set_additional_message(
            b"gif_out_code() failed; GIF file corrupt\0" as *const u8
                as *const libc::c_char,
        );
        return 0x1000 as libc::c_int | 0x100 as libc::c_int;
    }
    if (*g).codes[code as usize].prefix as libc::c_int >= 0 as libc::c_int {
        gif_out_code(g, (*g).codes[code as usize].prefix as libc::c_ushort);
    }
    if (*g).cur_y >= (*g).max_y {
        return 0 as libc::c_int;
    }
    *((*g).out)
        .offset(
            ((*g).cur_x + (*g).cur_y * (*g).max_x) as isize,
        ) = (*g).codes[code as usize].suffix;
    if (*g).cur_x >= (*g).actual_width {
        (*g).actual_width = (*g).cur_x + 1 as libc::c_int;
    }
    if (*g).cur_y >= (*g).actual_height {
        (*g).actual_height = (*g).cur_y + 1 as libc::c_int;
    }
    (*g).cur_x += 1;
    (*g).cur_x;
    if (*g).cur_x >= (*g).max_x {
        (*g).cur_x = (*g).start_x;
        (*g).cur_y += (*g).step;
        while (*g).cur_y >= (*g).max_y && (*g).parse > 0 as libc::c_int {
            (*g).step = (1 as libc::c_int) << (*g).parse;
            (*g).cur_y = (*g).start_y + ((*g).step >> 1 as libc::c_int);
            (*g).parse -= 1;
            (*g).parse;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gif_process_raster(
    mut s: *mut gif_context_t,
    mut g: *mut gif_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut lzw_cs: libc::c_uchar = 0;
    let mut len: libc::c_int = 0;
    let mut code: libc::c_int = 0;
    let mut codesize: libc::c_int = 0;
    let mut codemask: libc::c_int = 0;
    let mut avail: libc::c_int = 0;
    let mut oldcode: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut valid_bits: libc::c_int = 0;
    let mut clear: libc::c_int = 0;
    let mut p: *mut gif_lzw = 0 as *mut gif_lzw;
    lzw_cs = gif_get8(s);
    if lzw_cs as libc::c_int > 12 as libc::c_int {
        sixel_helper_set_additional_message(
            b"Unsupported GIF (LZW code size)\0" as *const u8 as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
    } else {
        clear = (1 as libc::c_int) << lzw_cs as libc::c_int;
        codesize = lzw_cs as libc::c_int + 1 as libc::c_int;
        codemask = ((1 as libc::c_int) << codesize) - 1 as libc::c_int;
        bits = 0 as libc::c_int;
        valid_bits = 0 as libc::c_int;
        code = 0 as libc::c_int;
        while code < clear {
            (*g).codes[code as usize].prefix = -(1 as libc::c_int) as libc::c_short;
            (*g).codes[code as usize].first = code as libc::c_uchar;
            (*g).codes[code as usize].suffix = code as libc::c_uchar;
            code += 1;
            code;
        }
        avail = clear + 2 as libc::c_int;
        oldcode = -(1 as libc::c_int);
        len = 0 as libc::c_int;
        loop {
            if valid_bits < codesize {
                if len == 0 as libc::c_int {
                    len = gif_get8(s) as libc::c_int;
                    if len == 0 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                }
                len -= 1;
                len;
                bits |= (gif_get8(s) as libc::c_int) << valid_bits;
                valid_bits += 8 as libc::c_int;
            } else {
                code = bits & codemask;
                bits >>= codesize;
                valid_bits -= codesize;
                if code == clear {
                    codesize = lzw_cs as libc::c_int + 1 as libc::c_int;
                    codemask = ((1 as libc::c_int) << codesize) - 1 as libc::c_int;
                    avail = clear + 2 as libc::c_int;
                    oldcode = -(1 as libc::c_int);
                } else if code == clear + 1 as libc::c_int {
                    (*s).img_buffer = ((*s).img_buffer).offset(len as isize);
                    loop {
                        len = gif_get8(s) as libc::c_int;
                        if !(len > 0 as libc::c_int) {
                            break;
                        }
                        (*s).img_buffer = ((*s).img_buffer).offset(len as isize);
                    }
                    return 0 as libc::c_int;
                } else if code <= avail {
                    if oldcode >= 0 as libc::c_int {
                        if avail < (1 as libc::c_int) << 12 as libc::c_int {
                            let fresh1 = avail;
                            avail = avail + 1;
                            p = &mut *((*g).codes).as_mut_ptr().offset(fresh1 as isize)
                                as *mut gif_lzw;
                            (*p).prefix = oldcode as libc::c_short;
                            (*p).first = (*g).codes[oldcode as usize].first;
                            (*p)
                                .suffix = (if code == avail {
                                (*p).first as libc::c_int
                            } else {
                                (*g).codes[code as usize].first as libc::c_int
                            }) as libc::c_uchar;
                        }
                    } else if code == avail {
                        sixel_helper_set_additional_message(
                            b"corrupt GIF (reason: illegal code in raster).\0"
                                as *const u8 as *const libc::c_char,
                        );
                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    }
                    status = gif_out_code(g, code as libc::c_ushort);
                    if status != 0 as libc::c_int {
                        break;
                    }
                    if avail & codemask == 0 as libc::c_int
                        && avail <= 0xfff as libc::c_int
                    {
                        codesize += 1;
                        codesize;
                        codemask = ((1 as libc::c_int) << codesize) - 1 as libc::c_int;
                    }
                    oldcode = code;
                } else {
                    sixel_helper_set_additional_message(
                        b"corrupt GIF (reason: illegal code in raster).\0" as *const u8
                            as *const libc::c_char,
                    );
                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                    break;
                }
            }
        }
    }
    return status;
}
unsafe extern "C" fn gif_load_next(
    mut s: *mut gif_context_t,
    mut g: *mut gif_t,
    mut bgcolor: *mut libc::c_uchar,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut buffer: [libc::c_uchar; 256] = [0; 256];
    let mut c: libc::c_uchar = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    loop {
        c = gif_get8(s);
        match c as libc::c_int {
            44 => {
                x = gif_get16le(s);
                y = gif_get16le(s);
                w = gif_get16le(s);
                h = gif_get16le(s);
                if x >= (*g).w || y >= (*g).h || x + w > (*g).w || y + h > (*g).h {
                    sixel_helper_set_additional_message(
                        b"corrupt GIF (reason: bad Image Separator).\0" as *const u8
                            as *const libc::c_char,
                    );
                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                    break;
                } else {
                    (*g).line_size = (*g).w;
                    (*g).start_x = x;
                    (*g).start_y = y;
                    (*g).max_x = (*g).start_x + w;
                    (*g).max_y = (*g).start_y + h;
                    (*g).cur_x = (*g).start_x;
                    (*g).cur_y = (*g).start_y;
                    (*g).actual_width = (*g).start_x;
                    (*g).actual_height = (*g).start_y;
                    (*g).lflags = gif_get8(s) as libc::c_int;
                    if (*g).lflags & 0x40 as libc::c_int != 0 {
                        (*g).step = 8 as libc::c_int;
                        (*g).parse = 3 as libc::c_int;
                    } else {
                        (*g).step = 1 as libc::c_int;
                        (*g).parse = 0 as libc::c_int;
                    }
                    if (*g).lflags & 0x80 as libc::c_int != 0 {
                        gif_parse_colortable(
                            s,
                            ((*g).lpal).as_mut_ptr(),
                            (2 as libc::c_int) << ((*g).lflags & 7 as libc::c_int),
                        );
                        (*g)
                            .color_table = ((*g).lpal).as_mut_ptr()
                            as *mut libc::c_uchar;
                    } else if (*g).flags & 0x80 as libc::c_int != 0 {
                        if (*g).transparent >= 0 as libc::c_int
                            && (*g).eflags & 0x1 as libc::c_int != 0
                        {
                            if !bgcolor.is_null() {
                                (*g)
                                    .pal[(*g).transparent
                                    as usize][0 as libc::c_int
                                    as usize] = *bgcolor.offset(2 as libc::c_int as isize);
                                (*g)
                                    .pal[(*g).transparent
                                    as usize][1 as libc::c_int
                                    as usize] = *bgcolor.offset(1 as libc::c_int as isize);
                                (*g)
                                    .pal[(*g).transparent
                                    as usize][2 as libc::c_int
                                    as usize] = *bgcolor.offset(0 as libc::c_int as isize);
                            }
                        }
                        (*g).color_table = ((*g).pal).as_mut_ptr() as *mut libc::c_uchar;
                    } else {
                        sixel_helper_set_additional_message(
                            b"corrupt GIF (reason: missing color table).\0" as *const u8
                                as *const libc::c_char,
                        );
                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                        break;
                    }
                    status = gif_process_raster(s, g);
                    if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                        break;
                    } else {
                        break;
                    }
                }
            }
            33 => {
                match gif_get8(s) as libc::c_int {
                    1 => {}
                    33 => {}
                    249 => {
                        len = gif_get8(s) as libc::c_int;
                        if len == 4 as libc::c_int {
                            (*g).eflags = gif_get8(s) as libc::c_int;
                            (*g).delay = gif_get16le(s);
                            (*g).transparent = gif_get8(s) as libc::c_int;
                        } else if ((*s).img_buffer).offset(len as isize)
                            > (*s).img_buffer_end
                        {
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            (*s).img_buffer = ((*s).img_buffer).offset(len as isize);
                        }
                    }
                    255 => {
                        len = gif_get8(s) as libc::c_int;
                        if ((*s).img_buffer).offset(len as isize) > (*s).img_buffer_end {
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            memcpy(
                                buffer.as_mut_ptr() as *mut libc::c_void,
                                (*s).img_buffer as *const libc::c_void,
                                len as size_t,
                            );
                            (*s).img_buffer = ((*s).img_buffer).offset(len as isize);
                            buffer[len as usize] = 0 as libc::c_int as libc::c_uchar;
                            if len == 11 as libc::c_int
                                && strcmp(
                                    buffer.as_mut_ptr() as *mut libc::c_char,
                                    b"NETSCAPE2.0\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                            {
                                if gif_get8(s) as libc::c_int == 0x3 as libc::c_int {
                                    match gif_get8(s) as libc::c_int {
                                        0 => {
                                            (*g).loop_count = 1 as libc::c_int;
                                        }
                                        1 => {
                                            (*g).loop_count = gif_get16le(s);
                                        }
                                        _ => {
                                            (*g).loop_count = 1 as libc::c_int;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        len = gif_get8(s) as libc::c_int;
                        if ((*s).img_buffer).offset(len as isize) > (*s).img_buffer_end {
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                            break;
                        } else {
                            memcpy(
                                buffer.as_mut_ptr() as *mut libc::c_void,
                                (*s).img_buffer as *const libc::c_void,
                                len as size_t,
                            );
                            (*s).img_buffer = ((*s).img_buffer).offset(len as isize);
                        }
                    }
                }
                c = gif_get8(s);
                if !(c as libc::c_int != 0 as libc::c_int) {
                    continue;
                }
                sprintf(
                    buffer.as_mut_ptr() as *mut libc::c_char,
                    b"missing valid block terminator (unknown code %02x).\0" as *const u8
                        as *const libc::c_char,
                    c as libc::c_int,
                );
                sixel_helper_set_additional_message(
                    buffer.as_mut_ptr() as *mut libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                break;
            }
            59 => {
                (*g).is_terminated = 1 as libc::c_int;
                status = 0 as libc::c_int;
                break;
            }
            _ => {
                sprintf(
                    buffer.as_mut_ptr() as *mut libc::c_char,
                    b"corrupt GIF (reason: unknown code %02x).\0" as *const u8
                        as *const libc::c_char,
                    c as libc::c_int,
                );
                sixel_helper_set_additional_message(
                    buffer.as_mut_ptr() as *mut libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                break;
            }
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn load_gif(
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
    mut bgcolor: *mut libc::c_uchar,
    mut reqcolors: libc::c_int,
    mut fuse_palette: libc::c_int,
    mut fstatic: libc::c_int,
    mut loop_control: libc::c_int,
    mut fn_load: *mut libc::c_void,
    mut context: *mut libc::c_void,
    mut allocator: *mut sixel_allocator_t,
) -> libc::c_int {
    let mut bytes: size_t = 0;
    let mut s: gif_context_t = gif_context_t {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        buflen: 0,
        buffer_start: [0; 128],
        img_buffer: 0 as *mut libc::c_uchar,
        img_buffer_end: 0 as *mut libc::c_uchar,
        img_buffer_original: 0 as *mut libc::c_uchar,
    };
    let mut g: gif_t = gif_t {
        w: 0,
        h: 0,
        out: 0 as *mut libc::c_uchar,
        flags: 0,
        bgindex: 0,
        ratio: 0,
        transparent: 0,
        eflags: 0,
        pal: [[0; 3]; 256],
        lpal: [[0; 3]; 256],
        codes: [gif_lzw {
            prefix: 0,
            first: 0,
            suffix: 0,
        }; 4096],
        color_table: 0 as *mut libc::c_uchar,
        parse: 0,
        step: 0,
        lflags: 0,
        start_x: 0,
        start_y: 0,
        max_x: 0,
        max_y: 0,
        cur_x: 0,
        cur_y: 0,
        actual_width: 0,
        actual_height: 0,
        line_size: 0,
        loop_count: 0,
        delay: 0,
        is_multiframe: 0,
        is_terminated: 0,
    };
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut frame: *mut sixel_frame_t = 0 as *mut sixel_frame_t;
    let mut fnp: fn_pointer = _fn_pointer { fn_0: None };
    let mut message: [libc::c_char; 256] = [0; 256];
    fnp.p = fn_load;
    status = sixel_frame_new(&mut frame, allocator);
    if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
        s.img_buffer_original = buffer;
        s.img_buffer = s.img_buffer_original;
        s.img_buffer_end = buffer.offset(size as isize);
        memset(
            &mut g as *mut gif_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<gif_t>() as libc::c_ulong,
        );
        g.delay = 1 as libc::c_int;
        status = gif_load_header(&mut s, &mut g);
        if !(status != 0 as libc::c_int) {
            bytes = (g.w as size_t).wrapping_mul(g.h as size_t);
            g.out = sixel_allocator_malloc(allocator, bytes) as *mut libc::c_uchar;
            if (g.out).is_null() {
                sprintf(
                    message.as_mut_ptr(),
                    b"load_gif: sixel_allocator_malloc() failed. size=%zu.\0"
                        as *const u8 as *const libc::c_char,
                    bytes,
                );
                sixel_helper_set_additional_message(message.as_mut_ptr());
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
            } else {
                memset(g.out as *mut libc::c_void, 0 as libc::c_int, bytes);
                (*frame).loop_count = 0 as libc::c_int;
                's_80: loop {
                    (*frame).frame_no = 0 as libc::c_int;
                    s.img_buffer = s.img_buffer_original;
                    status = gif_load_header(&mut s, &mut g);
                    if status != 0 as libc::c_int {
                        break;
                    }
                    g.is_terminated = 0 as libc::c_int;
                    loop {
                        status = gif_load_next(&mut s, &mut g, bgcolor);
                        if status != 0 as libc::c_int {
                            break 's_80;
                        }
                        if g.is_terminated != 0 {
                            break;
                        }
                        (*frame).width = g.actual_width;
                        (*frame).height = g.actual_height;
                        status = gif_init_frame(
                            frame,
                            &mut g,
                            bgcolor,
                            reqcolors,
                            fuse_palette,
                        );
                        if status != 0 as libc::c_int {
                            break 's_80;
                        }
                        status = (fnp.fn_0)
                            .expect("non-null function pointer")(frame, context);
                        if status != 0 as libc::c_int {
                            break 's_80;
                        }
                        if fstatic != 0 {
                            break 's_80;
                        }
                        (*frame).frame_no += 1;
                        (*frame).frame_no;
                    }
                    (*frame).loop_count += 1;
                    (*frame).loop_count;
                    if g.loop_count < 0 as libc::c_int {
                        break;
                    }
                    if loop_control == 2 as libc::c_int
                        || (*frame).frame_no == 1 as libc::c_int
                    {
                        break;
                    }
                    if !(loop_control == 0 as libc::c_int) {
                        continue;
                    }
                    if (*frame).loop_count == g.loop_count {
                        break;
                    }
                }
            }
        }
    }
    sixel_allocator_free((*frame).allocator, g.out as *mut libc::c_void);
    sixel_frame_unref(frame);
    return status;
}
