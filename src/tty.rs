#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn sixel_helper_set_additional_message(message: *const libc::c_char);
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type SIXELSTATUS = libc::c_int;
pub type sixel_write_function = Option::<
    unsafe extern "C" fn(
        *mut libc::c_char,
        libc::c_int,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn sixel_tty_wait_stdin(mut usec: libc::c_int) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ret: libc::c_int = 0 as libc::c_int;
    tv.tv_sec = (usec / 1000000 as libc::c_int) as __time_t;
    tv.tv_usec = (usec % 1000000 as libc::c_int) as __suseconds_t;
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut rfds;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1);
        __i;
    }
    rfds
        .__fds_bits[(0 as libc::c_int
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << 0 as libc::c_int
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    ret = select(
        0 as libc::c_int + 1 as libc::c_int,
        &mut rfds,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut tv,
    );
    if ret < 0 as libc::c_int {
        status = 0x1000 as libc::c_int | 0x400 as libc::c_int
            | *__errno_location() & 0xff as libc::c_int;
        sixel_helper_set_additional_message(
            b"sixel_tty_wait_stdin: select() failed.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        status = 0 as libc::c_int;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_tty_scroll(
    mut f_write: sixel_write_function,
    mut outfd: libc::c_int,
    mut height: libc::c_int,
    mut is_animation: libc::c_int,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut nwrite: libc::c_int = 0;
    nwrite = f_write
        .expect(
            "non-null function pointer",
        )(
        b"\x1B[H\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int,
        &mut outfd as *mut libc::c_int as *mut libc::c_void,
    );
    if nwrite < 0 as libc::c_int {
        status = 0x1000 as libc::c_int | 0x400 as libc::c_int
            | *__errno_location() & 0xff as libc::c_int;
        sixel_helper_set_additional_message(
            b"sixel_tty_scroll: f_write() failed.\0" as *const u8 as *const libc::c_char,
        );
    } else {
        status = 0 as libc::c_int;
    }
    return status;
}
