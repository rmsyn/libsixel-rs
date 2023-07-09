#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type sixel_allocator;
    fn free(_: *mut libc::c_void);
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sixel_allocator_malloc(
        allocator: *mut sixel_allocator_t,
        n: size_t,
    ) -> *mut libc::c_void;
    fn sixel_allocator_free(allocator: *mut sixel_allocator_t, p: *mut libc::c_void);
    fn sixel_helper_compute_depth(pixelformat: libc::c_int) -> libc::c_int;
    fn sixel_helper_normalize_pixelformat(
        dst: *mut libc::c_uchar,
        dst_pixelformat: *mut libc::c_int,
        src: *const libc::c_uchar,
        src_pixelformat: libc::c_int,
        width: libc::c_int,
        height: libc::c_int,
    ) -> SIXELSTATUS;
}
pub type size_t = libc::c_ulong;
pub type SIXELSTATUS = libc::c_int;
pub type sixel_allocator_t = sixel_allocator;
pub type resample_fn_t = Option::<
    unsafe extern "C" fn(libc::c_double) -> libc::c_double,
>;
unsafe extern "C" fn bilinear(d: libc::c_double) -> libc::c_double {
    if d < 1.0f64 {
        return 1.0f64 - d;
    }
    return 0.0f64;
}
unsafe extern "C" fn welsh(d: libc::c_double) -> libc::c_double {
    if d < 1.0f64 {
        return 1.0f64 - d * d;
    }
    return 0.0f64;
}
unsafe extern "C" fn bicubic(d: libc::c_double) -> libc::c_double {
    if d <= 1.0f64 {
        return 1.0f64 + (d - 2.0f64) * d * d;
    }
    if d <= 2.0f64 {
        return 4.0f64 + d * (-8.0f64 + d * (5.0f64 - d));
    }
    return 0.0f64;
}
unsafe extern "C" fn sinc(x: libc::c_double) -> libc::c_double {
    return sin(3.14159265358979323846f64 * x) / (3.14159265358979323846f64 * x);
}
unsafe extern "C" fn lanczos2(d: libc::c_double) -> libc::c_double {
    if d == 0.0f64 {
        return 1.0f64;
    }
    if d < 2.0f64 {
        return sinc(d) * sinc(d / 2.0f64);
    }
    return 0.0f64;
}
unsafe extern "C" fn lanczos3(d: libc::c_double) -> libc::c_double {
    if d == 0.0f64 {
        return 1.0f64;
    }
    if d < 3.0f64 {
        return sinc(d) * sinc(d / 3.0f64);
    }
    return 0.0f64;
}
unsafe extern "C" fn lanczos4(d: libc::c_double) -> libc::c_double {
    if d == 0.0f64 {
        return 1.0f64;
    }
    if d < 4.0f64 {
        return sinc(d) * sinc(d / 4.0f64);
    }
    return 0.0f64;
}
unsafe extern "C" fn gaussian(d: libc::c_double) -> libc::c_double {
    return exp(-2.0f64 * d * d) * sqrt(2.0f64 / 3.14159265358979323846f64);
}
unsafe extern "C" fn hanning(d: libc::c_double) -> libc::c_double {
    return 0.5f64 + 0.5f64 * cos(d * 3.14159265358979323846f64);
}
unsafe extern "C" fn hamming(d: libc::c_double) -> libc::c_double {
    return 0.54f64 + 0.46f64 * cos(d * 3.14159265358979323846f64);
}
unsafe extern "C" fn normalize(
    mut x: libc::c_double,
    mut total: libc::c_double,
) -> libc::c_uchar {
    let mut result: libc::c_int = 0;
    result = floor(x / total) as libc::c_int;
    if result > 255 as libc::c_int {
        return 0xff as libc::c_int as libc::c_uchar;
    }
    if result < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_uchar;
    }
    return result as libc::c_uchar;
}
unsafe extern "C" fn scale_without_resampling(
    mut dst: *mut libc::c_uchar,
    mut src: *const libc::c_uchar,
    srcw: libc::c_int,
    srch: libc::c_int,
    dstw: libc::c_int,
    dsth: libc::c_int,
    depth: libc::c_int,
) {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    h = 0 as libc::c_int;
    while h < dsth {
        w = 0 as libc::c_int;
        while w < dstw {
            x = w * srcw / dstw;
            y = h * srch / dsth;
            i = 0 as libc::c_int;
            while i < depth {
                pos = (y * srcw + x) * depth + i;
                *dst
                    .offset(
                        ((h * dstw + w) * depth + i) as isize,
                    ) = *src.offset(pos as isize);
                i += 1;
                i;
            }
            w += 1;
            w;
        }
        h += 1;
        h;
    }
}
unsafe extern "C" fn scale_with_resampling(
    mut dst: *mut libc::c_uchar,
    mut src: *const libc::c_uchar,
    srcw: libc::c_int,
    srch: libc::c_int,
    dstw: libc::c_int,
    dsth: libc::c_int,
    depth: libc::c_int,
    f_resample: resample_fn_t,
    mut n: libc::c_double,
) {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut x_first: libc::c_int = 0;
    let mut x_last: libc::c_int = 0;
    let mut y_first: libc::c_int = 0;
    let mut y_last: libc::c_int = 0;
    let mut center_x: libc::c_double = 0.;
    let mut center_y: libc::c_double = 0.;
    let mut diff_x: libc::c_double = 0.;
    let mut diff_y: libc::c_double = 0.;
    let mut weight: libc::c_double = 0.;
    let mut total: libc::c_double = 0.;
    let mut offsets: [libc::c_double; 8] = [0.; 8];
    h = 0 as libc::c_int;
    while h < dsth {
        w = 0 as libc::c_int;
        while w < dstw {
            total = 0.0f64;
            i = 0 as libc::c_int;
            while i < depth {
                offsets[i as usize] = 0 as libc::c_int as libc::c_double;
                i += 1;
                i;
            }
            if dstw >= srcw {
                center_x = (w as libc::c_double + 0.5f64) * srcw as libc::c_double
                    / dstw as libc::c_double;
                x_first = (if center_x - n > 0 as libc::c_int as libc::c_double {
                    center_x - n
                } else {
                    0 as libc::c_int as libc::c_double
                }) as libc::c_int;
                x_last = (if center_x + n < (srcw - 1 as libc::c_int) as libc::c_double {
                    center_x + n
                } else {
                    (srcw - 1 as libc::c_int) as libc::c_double
                }) as libc::c_int;
            } else {
                center_x = w as libc::c_double + 0.5f64;
                x_first = (if floor(
                    (center_x - n) * srcw as libc::c_double / dstw as libc::c_double,
                ) > 0 as libc::c_int as libc::c_double
                {
                    floor(
                        (center_x - n) * srcw as libc::c_double / dstw as libc::c_double,
                    )
                } else {
                    0 as libc::c_int as libc::c_double
                }) as libc::c_int;
                x_last = (if floor(
                    (center_x + n) * srcw as libc::c_double / dstw as libc::c_double,
                ) < (srcw - 1 as libc::c_int) as libc::c_double
                {
                    floor(
                        (center_x + n) * srcw as libc::c_double / dstw as libc::c_double,
                    )
                } else {
                    (srcw - 1 as libc::c_int) as libc::c_double
                }) as libc::c_int;
            }
            if dsth >= srch {
                center_y = (h as libc::c_double + 0.5f64) * srch as libc::c_double
                    / dsth as libc::c_double;
                y_first = (if center_y - n > 0 as libc::c_int as libc::c_double {
                    center_y - n
                } else {
                    0 as libc::c_int as libc::c_double
                }) as libc::c_int;
                y_last = (if center_y + n < (srch - 1 as libc::c_int) as libc::c_double {
                    center_y + n
                } else {
                    (srch - 1 as libc::c_int) as libc::c_double
                }) as libc::c_int;
            } else {
                center_y = h as libc::c_double + 0.5f64;
                y_first = (if floor(
                    (center_y - n) * srch as libc::c_double / dsth as libc::c_double,
                ) > 0 as libc::c_int as libc::c_double
                {
                    floor(
                        (center_y - n) * srch as libc::c_double / dsth as libc::c_double,
                    )
                } else {
                    0 as libc::c_int as libc::c_double
                }) as libc::c_int;
                y_last = (if floor(
                    (center_y + n) * srch as libc::c_double / dsth as libc::c_double,
                ) < (srch - 1 as libc::c_int) as libc::c_double
                {
                    floor(
                        (center_y + n) * srch as libc::c_double / dsth as libc::c_double,
                    )
                } else {
                    (srch - 1 as libc::c_int) as libc::c_double
                }) as libc::c_int;
            }
            y = y_first;
            while y <= y_last {
                x = x_first;
                while x <= x_last {
                    if dstw >= srcw {
                        diff_x = x as libc::c_double + 0.5f64 - center_x;
                    } else {
                        diff_x = (x as libc::c_double + 0.5f64) * dstw as libc::c_double
                            / srcw as libc::c_double - center_x;
                    }
                    if dsth >= srch {
                        diff_y = y as libc::c_double + 0.5f64 - center_y;
                    } else {
                        diff_y = (y as libc::c_double + 0.5f64) * dsth as libc::c_double
                            / srch as libc::c_double - center_y;
                    }
                    weight = f_resample.expect("non-null function pointer")(fabs(diff_x))
                        * f_resample.expect("non-null function pointer")(fabs(diff_y));
                    i = 0 as libc::c_int;
                    while i < depth {
                        pos = (y * srcw + x) * depth + i;
                        offsets[i as usize]
                            += *src.offset(pos as isize) as libc::c_int as libc::c_double
                                * weight;
                        i += 1;
                        i;
                    }
                    total += weight;
                    x += 1;
                    x;
                }
                y += 1;
                y;
            }
            if total > 0.0f64 {
                i = 0 as libc::c_int;
                while i < depth {
                    pos = (h * dstw + w) * depth + i;
                    *dst.offset(pos as isize) = normalize(offsets[i as usize], total);
                    i += 1;
                    i;
                }
            }
            w += 1;
            w;
        }
        h += 1;
        h;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sixel_helper_scale_image(
    mut dst: *mut libc::c_uchar,
    mut src: *const libc::c_uchar,
    mut srcw: libc::c_int,
    mut srch: libc::c_int,
    mut pixelformat: libc::c_int,
    mut dstw: libc::c_int,
    mut dsth: libc::c_int,
    mut method_for_resampling: libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let depth: libc::c_int = sixel_helper_compute_depth(pixelformat);
    let mut new_src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut nret: libc::c_int = 0;
    let mut new_pixelformat: libc::c_int = 0;
    if depth != 3 as libc::c_int {
        new_src = sixel_allocator_malloc(
            allocator,
            (srcw * srch * 3 as libc::c_int) as size_t,
        ) as *mut libc::c_uchar;
        if new_src.is_null() {
            return -(1 as libc::c_int);
        }
        nret = sixel_helper_normalize_pixelformat(
            new_src,
            &mut new_pixelformat,
            src,
            pixelformat,
            srcw,
            srch,
        );
        if nret != 0 as libc::c_int {
            sixel_allocator_free(allocator, new_src as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        src = new_src;
    } else {
        new_pixelformat = pixelformat;
    }
    match method_for_resampling {
        0 => {
            scale_without_resampling(dst, src, srcw, srch, dstw, dsth, depth);
        }
        1 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(gaussian as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                1.0f64,
            );
        }
        2 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(hanning as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                1.0f64,
            );
        }
        3 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(hamming as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                1.0f64,
            );
        }
        5 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(welsh as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                1.0f64,
            );
        }
        6 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(bicubic as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                2.0f64,
            );
        }
        7 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(lanczos2 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                3.0f64,
            );
        }
        8 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(lanczos3 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                3.0f64,
            );
        }
        9 => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(lanczos4 as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                4.0f64,
            );
        }
        4 | _ => {
            scale_with_resampling(
                dst,
                src,
                srcw,
                srch,
                dstw,
                dsth,
                depth,
                Some(bilinear as unsafe extern "C" fn(libc::c_double) -> libc::c_double),
                1.0f64,
            );
        }
    }
    free(new_src as *mut libc::c_void);
    return 0 as libc::c_int;
}
