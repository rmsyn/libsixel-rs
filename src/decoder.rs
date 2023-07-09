#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sixel_allocator;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
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
    fn sixel_allocator_realloc(
        allocator: *mut sixel_allocator_t,
        p: *mut libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn sixel_allocator_free(allocator: *mut sixel_allocator_t, p: *mut libc::c_void);
    fn sixel_decode_raw(
        p: *mut libc::c_uchar,
        len: libc::c_int,
        pixels: *mut *mut libc::c_uchar,
        pwidth: *mut libc::c_int,
        pheight: *mut libc::c_int,
        palette: *mut *mut libc::c_uchar,
        ncolors: *mut libc::c_int,
        allocator: *mut sixel_allocator_t,
    ) -> SIXELSTATUS;
    fn sixel_helper_set_additional_message(message: *const libc::c_char);
    fn sixel_helper_write_image_file(
        data: *mut libc::c_uchar,
        width: libc::c_int,
        height: libc::c_int,
        palette: *mut libc::c_uchar,
        pixelformat: libc::c_int,
        filename: *const libc::c_char,
        imageformat: libc::c_int,
        allocator: *mut sixel_allocator_t,
    ) -> SIXELSTATUS;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sixel_decoder {
    pub ref_0: libc::c_uint,
    pub input: *mut libc::c_char,
    pub output: *mut libc::c_char,
    pub allocator: *mut sixel_allocator_t,
}
pub type sixel_decoder_t = sixel_decoder;
unsafe extern "C" fn strdup_with_allocator(
    mut s: *const libc::c_char,
    mut allocator: *mut sixel_allocator_t,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = sixel_allocator_malloc(
        allocator,
        (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if !p.is_null() {
        strcpy(p, s);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decoder_new(
    mut ppdecoder: *mut *mut sixel_decoder_t,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    if allocator.is_null() {
        status = sixel_allocator_new(&mut allocator, None, None, None, None);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            current_block = 9831426743560484716;
        } else {
            current_block = 7095457783677275021;
        }
    } else {
        sixel_allocator_ref(allocator);
        current_block = 7095457783677275021;
    }
    match current_block {
        7095457783677275021 => {
            *ppdecoder = sixel_allocator_malloc(
                allocator,
                ::core::mem::size_of::<sixel_decoder_t>() as libc::c_ulong,
            ) as *mut sixel_decoder_t;
            if (*ppdecoder).is_null() {
                sixel_allocator_unref(allocator);
                sixel_helper_set_additional_message(
                    b"sixel_decoder_new: sixel_allocator_malloc() failed.\0" as *const u8
                        as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
            } else {
                (**ppdecoder).ref_0 = 1 as libc::c_int as libc::c_uint;
                (**ppdecoder)
                    .output = strdup_with_allocator(
                    b"-\0" as *const u8 as *const libc::c_char,
                    allocator,
                );
                (**ppdecoder)
                    .input = strdup_with_allocator(
                    b"-\0" as *const u8 as *const libc::c_char,
                    allocator,
                );
                (**ppdecoder).allocator = allocator;
                if ((**ppdecoder).output).is_null() || ((**ppdecoder).input).is_null() {
                    sixel_decoder_unref(*ppdecoder);
                    *ppdecoder = 0 as *mut sixel_decoder_t;
                    sixel_helper_set_additional_message(
                        b"sixel_decoder_new: strdup_with_allocator() failed.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                        | 0x1 as libc::c_int;
                    sixel_allocator_unref(allocator);
                } else {
                    status = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decoder_create() -> *mut sixel_decoder_t {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut decoder: *mut sixel_decoder_t = 0 as *mut sixel_decoder_t;
    status = sixel_decoder_new(&mut decoder, 0 as *mut sixel_allocator_t);
    status & 0x1000 as libc::c_int != 0 as libc::c_int;
    return decoder;
}
unsafe extern "C" fn sixel_decoder_destroy(mut decoder: *mut sixel_decoder_t) {
    let mut allocator: *mut sixel_allocator_t = 0 as *mut sixel_allocator_t;
    if !decoder.is_null() {
        allocator = (*decoder).allocator;
        sixel_allocator_free(allocator, (*decoder).input as *mut libc::c_void);
        sixel_allocator_free(allocator, (*decoder).output as *mut libc::c_void);
        sixel_allocator_free(allocator, decoder as *mut libc::c_void);
        sixel_allocator_unref(allocator);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decoder_ref(mut decoder: *mut sixel_decoder_t) {
    (*decoder).ref_0 = ((*decoder).ref_0).wrapping_add(1);
    (*decoder).ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decoder_unref(mut decoder: *mut sixel_decoder_t) {
    if !decoder.is_null()
        && {
            (*decoder).ref_0 = ((*decoder).ref_0).wrapping_sub(1);
            (*decoder).ref_0 == 0 as libc::c_int as libc::c_uint
        }
    {
        sixel_decoder_destroy(decoder);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decoder_setopt(
    mut decoder: *mut sixel_decoder_t,
    mut arg: libc::c_int,
    mut value: *const libc::c_char,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    sixel_decoder_ref(decoder);
    match arg {
        105 => {
            free((*decoder).input as *mut libc::c_void);
            (*decoder).input = strdup_with_allocator(value, (*decoder).allocator);
            if ((*decoder).input).is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_decoder_setopt: strdup_with_allocator() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 1644243570004545979;
            } else {
                current_block = 2979737022853876585;
            }
        }
        111 => {
            free((*decoder).output as *mut libc::c_void);
            (*decoder).output = strdup_with_allocator(value, (*decoder).allocator);
            if ((*decoder).output).is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_decoder_setopt: strdup_with_allocator() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
                current_block = 1644243570004545979;
            } else {
                current_block = 2979737022853876585;
            }
        }
        63 | _ => {
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x2 as libc::c_int;
            current_block = 1644243570004545979;
        }
    }
    match current_block {
        2979737022853876585 => {
            status = 0 as libc::c_int;
        }
        _ => {}
    }
    sixel_decoder_unref(decoder);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_decoder_decode(
    mut decoder: *mut sixel_decoder_t,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut raw_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sx: libc::c_int = 0;
    let mut sy: libc::c_int = 0;
    let mut raw_len: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut input_fp: *mut FILE = 0 as *mut FILE;
    let mut indexed_pixels: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut palette: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ncolors: libc::c_int = 0;
    sixel_decoder_ref(decoder);
    if strcmp((*decoder).input, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        input_fp = stdin;
        current_block = 13536709405535804910;
    } else {
        input_fp = fopen((*decoder).input, b"rb\0" as *const u8 as *const libc::c_char);
        if input_fp.is_null() {
            sixel_helper_set_additional_message(
                b"sixel_decoder_decode: fopen() failed.\0" as *const u8
                    as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x400 as libc::c_int
                | *__errno_location() & 0xff as libc::c_int;
            current_block = 2825366516446051848;
        } else {
            current_block = 13536709405535804910;
        }
    }
    match current_block {
        13536709405535804910 => {
            raw_len = 0 as libc::c_int;
            max = 64 as libc::c_int * 1024 as libc::c_int;
            raw_data = sixel_allocator_malloc((*decoder).allocator, max as size_t)
                as *mut libc::c_uchar;
            if raw_data.is_null() {
                sixel_helper_set_additional_message(
                    b"sixel_decoder_decode: sixel_allocator_malloc() failed.\0"
                        as *const u8 as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
            } else {
                loop {
                    if max - raw_len < 4096 as libc::c_int {
                        max *= 2 as libc::c_int;
                        raw_data = sixel_allocator_realloc(
                            (*decoder).allocator,
                            raw_data as *mut libc::c_void,
                            max as size_t,
                        ) as *mut libc::c_uchar;
                        if raw_data.is_null() {
                            sixel_helper_set_additional_message(
                                b"sixel_decoder_decode: sixel_allocator_realloc() failed.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                | 0x1 as libc::c_int;
                            current_block = 2825366516446051848;
                            break;
                        }
                    }
                    n = fread(
                        raw_data.offset(raw_len as isize) as *mut libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        4096 as libc::c_int as libc::c_ulong,
                        input_fp,
                    ) as libc::c_int;
                    if n <= 0 as libc::c_int {
                        current_block = 17478428563724192186;
                        break;
                    }
                    raw_len += n;
                }
                match current_block {
                    2825366516446051848 => {}
                    _ => {
                        if input_fp != stdout {
                            fclose(input_fp);
                        }
                        status = sixel_decode_raw(
                            raw_data,
                            raw_len,
                            &mut indexed_pixels,
                            &mut sx,
                            &mut sy,
                            &mut palette,
                            &mut ncolors,
                            (*decoder).allocator,
                        );
                        if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                            if sx > 1000000 as libc::c_int || sy > 1000000 as libc::c_int
                            {
                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                    | 0x3 as libc::c_int;
                            } else {
                                status = sixel_helper_write_image_file(
                                    indexed_pixels,
                                    sx,
                                    sy,
                                    palette,
                                    (1 as libc::c_int) << 7 as libc::c_int | 0x3 as libc::c_int,
                                    (*decoder).output,
                                    0x1 as libc::c_int,
                                    (*decoder).allocator,
                                );
                                status & 0x1000 as libc::c_int != 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    sixel_allocator_free((*decoder).allocator, raw_data as *mut libc::c_void);
    sixel_allocator_free((*decoder).allocator, indexed_pixels as *mut libc::c_void);
    sixel_allocator_free((*decoder).allocator, palette as *mut libc::c_void);
    sixel_decoder_unref(decoder);
    return status;
}
