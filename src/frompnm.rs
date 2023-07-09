#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type sixel_allocator;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sixel_allocator_malloc(
        allocator: *mut sixel_allocator_t,
        n: size_t,
    ) -> *mut libc::c_void;
    fn sixel_allocator_free(allocator: *mut sixel_allocator_t, p: *mut libc::c_void);
    fn sixel_helper_set_additional_message(message: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type SIXELSTATUS = libc::c_int;
pub type sixel_allocator_t = sixel_allocator;
unsafe extern "C" fn pnm_get_line(
    mut p: *mut libc::c_uchar,
    mut end: *mut libc::c_uchar,
    mut line: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut n: libc::c_int = 0;
    loop {
        n = 0 as libc::c_int;
        while p < end && *p as libc::c_int >= ' ' as i32 {
            if n < 255 as libc::c_int {
                let fresh0 = n;
                n = n + 1;
                *line.offset(fresh0 as isize) = *p;
            }
            p = p.offset(1);
            p;
        }
        if p < end && (*p as libc::c_int) < ' ' as i32 {
            p = p.offset(1);
            p;
        }
        *line.offset(n as isize) = '\0' as i32 as libc::c_uchar;
        if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32) {
            break;
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn load_pnm(
    mut p: *mut libc::c_uchar,
    mut length: libc::c_int,
    mut allocator: *mut sixel_allocator_t,
    mut result: *mut *mut libc::c_uchar,
    mut psx: *mut libc::c_int,
    mut psy: *mut libc::c_int,
    mut ppalette: *mut *mut libc::c_uchar,
    mut pncolors: *mut libc::c_int,
    mut ppixelformat: *mut libc::c_int,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut component: [libc::c_int; 3] = [0; 3];
    let mut ascii: libc::c_int = 0;
    let mut maps: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut deps: libc::c_int = 0;
    let mut message: [libc::c_char; 256] = [0; 256];
    let mut s: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: [libc::c_uchar; 256] = [0; 256];
    let mut size: size_t = 0;
    height = 0 as libc::c_int;
    width = height;
    deps = 1 as libc::c_int;
    end = p.offset(length as isize);
    p = pnm_get_line(p, end, tmp.as_mut_ptr());
    *result = 0 as *mut libc::c_uchar;
    if tmp[0 as libc::c_int as usize] as libc::c_int != 'P' as i32 {
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
        sixel_helper_set_additional_message(
            b"load_pnm: first character is not 'P'.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        match tmp[1 as libc::c_int as usize] as libc::c_int {
            49 => {
                ascii = 1 as libc::c_int;
                maps = 0 as libc::c_int;
                current_block = 15125582407903384992;
            }
            50 => {
                ascii = 1 as libc::c_int;
                maps = 1 as libc::c_int;
                current_block = 15125582407903384992;
            }
            51 => {
                ascii = 1 as libc::c_int;
                maps = 2 as libc::c_int;
                current_block = 15125582407903384992;
            }
            52 => {
                ascii = 0 as libc::c_int;
                maps = 0 as libc::c_int;
                current_block = 15125582407903384992;
            }
            53 => {
                ascii = 0 as libc::c_int;
                maps = 1 as libc::c_int;
                current_block = 15125582407903384992;
            }
            54 => {
                ascii = 0 as libc::c_int;
                maps = 2 as libc::c_int;
                current_block = 15125582407903384992;
            }
            _ => {
                current_block = 10702109779307661744;
            }
        }
        match current_block {
            15125582407903384992 => {
                p = pnm_get_line(p, end, tmp.as_mut_ptr());
                if p == end {
                    current_block = 11173504317902546298;
                } else {
                    s = tmp.as_mut_ptr();
                    width = 0 as libc::c_int;
                    loop {
                        if !(*s as libc::c_int >= '0' as i32
                            && *s as libc::c_int <= '9' as i32)
                        {
                            current_block = 2604890879466389055;
                            break;
                        }
                        width = width * 10 as libc::c_int
                            + (*s as libc::c_int - '0' as i32);
                        if width > (1 as libc::c_int) << 16 as libc::c_int {
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                            sprintf(
                                message.as_mut_ptr(),
                                b"load_pnm: image width exceeds the limit %d.\0"
                                    as *const u8 as *const libc::c_char,
                                (1 as libc::c_int) << 16 as libc::c_int,
                            );
                            sixel_helper_set_additional_message(message.as_mut_ptr());
                            current_block = 5979673174675504496;
                            break;
                        } else {
                            s = s.offset(1);
                            s;
                        }
                    }
                    match current_block {
                        5979673174675504496 => {}
                        _ => {
                            while *s as libc::c_int == ' ' as i32 {
                                s = s.offset(1);
                                s;
                            }
                            height = 0 as libc::c_int;
                            loop {
                                if !(*s as libc::c_int >= '0' as i32
                                    && *s as libc::c_int <= '9' as i32)
                                {
                                    current_block = 9441801433784995173;
                                    break;
                                }
                                height = height * 10 as libc::c_int
                                    + (*s as libc::c_int - '0' as i32);
                                if height > (1 as libc::c_int) << 16 as libc::c_int {
                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                                    sprintf(
                                        message.as_mut_ptr(),
                                        b"load_pnm: image height exceeds the limit %d.\0"
                                            as *const u8 as *const libc::c_char,
                                        (1 as libc::c_int) << 16 as libc::c_int,
                                    );
                                    sixel_helper_set_additional_message(message.as_mut_ptr());
                                    current_block = 5979673174675504496;
                                    break;
                                } else {
                                    s = s.offset(1);
                                    s;
                                }
                            }
                            match current_block {
                                5979673174675504496 => {}
                                _ => {
                                    while *s as libc::c_int != '\0' as i32 {
                                        s = s.offset(1);
                                        s;
                                    }
                                    if maps > 0 as libc::c_int {
                                        p = pnm_get_line(p, end, tmp.as_mut_ptr());
                                        if p == end {
                                            current_block = 11173504317902546298;
                                        } else {
                                            s = tmp.as_mut_ptr();
                                            deps = 0 as libc::c_int;
                                            while *s as libc::c_int >= '0' as i32
                                                && *s as libc::c_int <= '9' as i32
                                            {
                                                deps = deps * 10 as libc::c_int
                                                    + (*s as libc::c_int - '0' as i32);
                                                s = s.offset(1);
                                                s;
                                            }
                                            if deps > (1 as libc::c_int) << 16 as libc::c_int {
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                                                sprintf(
                                                    message.as_mut_ptr(),
                                                    b"load_pnm: image depth exceeds the limit %d.\0"
                                                        as *const u8 as *const libc::c_char,
                                                    (1 as libc::c_int) << 16 as libc::c_int,
                                                );
                                                sixel_helper_set_additional_message(message.as_mut_ptr());
                                                current_block = 5979673174675504496;
                                            } else {
                                                current_block = 6560072651652764009;
                                            }
                                        }
                                    } else {
                                        current_block = 6560072651652764009;
                                    }
                                    match current_block {
                                        11173504317902546298 => {}
                                        5979673174675504496 => {}
                                        _ => {
                                            if width < 1 as libc::c_int || height < 1 as libc::c_int
                                                || deps < 1 as libc::c_int
                                            {
                                                current_block = 11173504317902546298;
                                            } else {
                                                size = (width as size_t)
                                                    .wrapping_mul(height as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                                                *result = sixel_allocator_malloc(allocator, size)
                                                    as *mut libc::c_uchar;
                                                if (*result).is_null() {
                                                    sixel_helper_set_additional_message(
                                                        b"load_pnm: sixel_allocator_malloc() failed.\0" as *const u8
                                                            as *const libc::c_char,
                                                    );
                                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                        | 0x1 as libc::c_int;
                                                    current_block = 5979673174675504496;
                                                } else {
                                                    memset(
                                                        *result as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        size,
                                                    );
                                                    y = 0 as libc::c_int;
                                                    's_324: loop {
                                                        if !(y < height) {
                                                            current_block = 7019009297990327870;
                                                            break;
                                                        }
                                                        x = 0 as libc::c_int;
                                                        while x < width {
                                                            b = if maps == 2 as libc::c_int {
                                                                3 as libc::c_int
                                                            } else {
                                                                1 as libc::c_int
                                                            };
                                                            i = 0 as libc::c_int;
                                                            while i < b {
                                                                if ascii != 0 {
                                                                    while *s as libc::c_int == '\0' as i32 {
                                                                        if p >= end {
                                                                            break;
                                                                        }
                                                                        p = pnm_get_line(p, end, tmp.as_mut_ptr());
                                                                        s = tmp.as_mut_ptr();
                                                                    }
                                                                    n = 0 as libc::c_int;
                                                                    if maps == 0 as libc::c_int {
                                                                        n = (*s as libc::c_int == '0' as i32) as libc::c_int;
                                                                        if *s as libc::c_int != '\0' as i32 {
                                                                            s = s.offset(1);
                                                                            s;
                                                                        }
                                                                    } else {
                                                                        while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize)
                                                                            as libc::c_int
                                                                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                                                            != 0 && n >= 0 as libc::c_int
                                                                        {
                                                                            let fresh1 = s;
                                                                            s = s.offset(1);
                                                                            n = n * 10 as libc::c_int
                                                                                + (*fresh1 as libc::c_int - '0' as i32);
                                                                        }
                                                                        while *s as libc::c_int == ' ' as i32 {
                                                                            s = s.offset(1);
                                                                            s;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if p >= end {
                                                                        break;
                                                                    }
                                                                    if maps == 0 as libc::c_int {
                                                                        n = ((*p as libc::c_int) << (x & 0x7 as libc::c_int)
                                                                            >> 0x7 as libc::c_int & 1 as libc::c_int
                                                                            == 0 as libc::c_int) as libc::c_int;
                                                                        if x & 0x7 as libc::c_int == 0x7 as libc::c_int {
                                                                            p = p.offset(1);
                                                                            p;
                                                                        }
                                                                    } else {
                                                                        let fresh2 = p;
                                                                        p = p.offset(1);
                                                                        n = *fresh2 as libc::c_int;
                                                                    }
                                                                }
                                                                component[i as usize] = n;
                                                                i += 1;
                                                                i;
                                                            }
                                                            if i < b {
                                                                break;
                                                            }
                                                            match maps {
                                                                0 => {
                                                                    if component[0 as libc::c_int as usize] == 0 as libc::c_int
                                                                    {
                                                                        component[2 as libc::c_int as usize] = 0 as libc::c_int;
                                                                        component[1 as libc::c_int
                                                                            as usize] = component[2 as libc::c_int as usize];
                                                                        component[0 as libc::c_int
                                                                            as usize] = component[1 as libc::c_int as usize];
                                                                    } else {
                                                                        component[2 as libc::c_int as usize] = 255 as libc::c_int;
                                                                        component[1 as libc::c_int
                                                                            as usize] = component[2 as libc::c_int as usize];
                                                                        component[0 as libc::c_int
                                                                            as usize] = component[1 as libc::c_int as usize];
                                                                    }
                                                                }
                                                                1 => {
                                                                    component[2 as libc::c_int
                                                                        as usize] = component[0 as libc::c_int as usize]
                                                                        * 255 as libc::c_int / deps;
                                                                    component[1 as libc::c_int
                                                                        as usize] = component[2 as libc::c_int as usize];
                                                                    component[0 as libc::c_int
                                                                        as usize] = component[1 as libc::c_int as usize];
                                                                }
                                                                2 => {
                                                                    component[0 as libc::c_int
                                                                        as usize] = component[0 as libc::c_int as usize]
                                                                        * 255 as libc::c_int / deps;
                                                                    component[1 as libc::c_int
                                                                        as usize] = component[1 as libc::c_int as usize]
                                                                        * 255 as libc::c_int / deps;
                                                                    component[2 as libc::c_int
                                                                        as usize] = component[2 as libc::c_int as usize]
                                                                        * 255 as libc::c_int / deps;
                                                                }
                                                                _ => {
                                                                    current_block = 10702109779307661744;
                                                                    break 's_324;
                                                                }
                                                            }
                                                            *(*result)
                                                                .offset(((y * width + x) * 3 as libc::c_int) as isize)
                                                                .offset(
                                                                    0 as libc::c_int as isize,
                                                                ) = component[0 as libc::c_int as usize] as libc::c_uchar;
                                                            *(*result)
                                                                .offset(((y * width + x) * 3 as libc::c_int) as isize)
                                                                .offset(
                                                                    1 as libc::c_int as isize,
                                                                ) = component[1 as libc::c_int as usize] as libc::c_uchar;
                                                            *(*result)
                                                                .offset(((y * width + x) * 3 as libc::c_int) as isize)
                                                                .offset(
                                                                    2 as libc::c_int as isize,
                                                                ) = component[2 as libc::c_int as usize] as libc::c_uchar;
                                                            x += 1;
                                                            x;
                                                        }
                                                        y += 1;
                                                        y;
                                                    }
                                                    match current_block {
                                                        10702109779307661744 => {}
                                                        _ => {
                                                            *psx = width;
                                                            *psy = height;
                                                            *ppixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                                            status = 0 as libc::c_int;
                                                            current_block = 5979673174675504496;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                match current_block {
                    5979673174675504496 => {}
                    10702109779307661744 => {}
                    _ => {
                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                        sixel_helper_set_additional_message(
                            b"load_pnm: invalid data detected.\0" as *const u8
                                as *const libc::c_char,
                        );
                        sixel_allocator_free(allocator, *result as *mut libc::c_void);
                        *result = 0 as *mut libc::c_uchar;
                        current_block = 5979673174675504496;
                    }
                }
            }
            _ => {}
        }
        match current_block {
            5979673174675504496 => {}
            _ => {
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                sixel_helper_set_additional_message(
                    b"load_pnm: unknown ppm format.\0" as *const u8
                        as *const libc::c_char,
                );
                sixel_allocator_free(allocator, *result as *mut libc::c_void);
                *result = 0 as *mut libc::c_uchar;
            }
        }
    }
    return status;
}
