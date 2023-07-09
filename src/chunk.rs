#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
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
    fn sixel_helper_set_additional_message(message: *const libc::c_char);
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atime: __time_t,
    pub st_atimensec: __syscall_ulong_t,
    pub st_mtime: __time_t,
    pub st_mtimensec: __syscall_ulong_t,
    pub st_ctime: __time_t,
    pub st_ctimensec: __syscall_ulong_t,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
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
pub type sixel_malloc_t = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type sixel_calloc_t = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type sixel_realloc_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type sixel_free_t = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sixel_allocator {
    pub ref_0: libc::c_uint,
    pub fn_malloc: sixel_malloc_t,
    pub fn_calloc: sixel_calloc_t,
    pub fn_realloc: sixel_realloc_t,
    pub fn_free: sixel_free_t,
}
pub type sixel_allocator_t = sixel_allocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sixel_chunk {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub max_size: size_t,
    pub allocator: *mut sixel_allocator_t,
}
pub type sixel_chunk_t = sixel_chunk;
unsafe extern "C" fn sixel_chunk_init(
    pchunk: *mut sixel_chunk_t,
    mut initial_size: size_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    (*pchunk).max_size = initial_size;
    (*pchunk).size = 0 as libc::c_int as size_t;
    (*pchunk)
        .buffer = sixel_allocator_malloc((*pchunk).allocator, (*pchunk).max_size)
        as *mut libc::c_uchar;
    if ((*pchunk).buffer).is_null() {
        sixel_helper_set_additional_message(
            b"sixel_chunk_init: sixel_allocator_malloc() failed.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
    } else {
        status = 0 as libc::c_int;
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_chunk_destroy(pchunk: *mut sixel_chunk_t) {
    let mut allocator: *mut sixel_allocator_t = 0 as *mut sixel_allocator_t;
    if !pchunk.is_null() {
        allocator = (*pchunk).allocator;
        sixel_allocator_free(allocator, (*pchunk).buffer as *mut libc::c_void);
        sixel_allocator_free(allocator, pchunk as *mut libc::c_void);
        sixel_allocator_unref(allocator);
    }
}
unsafe extern "C" fn wait_file(
    mut fd: libc::c_int,
    mut usec: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut rfds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
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
        .__fds_bits[(fd
        / (8 as libc::c_int
            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << fd
                % (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    ret = select(
        fd + 1 as libc::c_int,
        &mut rfds,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut tv,
    );
    if ret == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if ret < 0 as libc::c_int {
        return ret;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn open_binary_file(
    mut f: *mut *mut FILE,
    mut filename: *const libc::c_char,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atime: 0,
        st_atimensec: 0,
        st_mtime: 0,
        st_mtimensec: 0,
        st_ctime: 0,
        st_ctimensec: 0,
        __glibc_reserved: [0; 3],
    };
    if filename.is_null()
        || strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        *f = stdin;
        status = 0 as libc::c_int;
    } else if stat(filename, &mut sb) != 0 as libc::c_int {
        status = 0x1000 as libc::c_int | 0x400 as libc::c_int
            | *__errno_location() & 0xff as libc::c_int;
        sixel_helper_set_additional_message(
            b"stat() failed.\0" as *const u8 as *const libc::c_char,
        );
    } else if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
        sixel_helper_set_additional_message(
            b"specified path is directory.\0" as *const u8 as *const libc::c_char,
        );
    } else {
        *f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
        if (*f).is_null() {
            status = 0x1000 as libc::c_int | 0x400 as libc::c_int
                | *__errno_location() & 0xff as libc::c_int;
            sixel_helper_set_additional_message(
                b"fopen() failed.\0" as *const u8 as *const libc::c_char,
            );
        } else {
            status = 0 as libc::c_int;
        }
    }
    return status;
}
unsafe extern "C" fn sixel_chunk_from_file(
    mut filename: *const libc::c_char,
    mut pchunk: *mut sixel_chunk_t,
    mut cancel_flag: *const libc::c_int,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut ret: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut n: size_t = 0;
    let bucket_size: size_t = 4096 as libc::c_int as size_t;
    status = open_binary_file(&mut f, filename);
    if !(status & 0x1000 as libc::c_int != 0 as libc::c_int || f.is_null()) {
        's_26: loop {
            if ((*pchunk).max_size).wrapping_sub((*pchunk).size) < bucket_size {
                (*pchunk)
                    .max_size = ((*pchunk).max_size as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                (*pchunk)
                    .buffer = sixel_allocator_realloc(
                    (*pchunk).allocator,
                    (*pchunk).buffer as *mut libc::c_void,
                    (*pchunk).max_size,
                ) as *mut libc::c_uchar;
                if ((*pchunk).buffer).is_null() {
                    sixel_helper_set_additional_message(
                        b"sixel_chunk_from_file: sixel_allocator_realloc() failed.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                        | 0x1 as libc::c_int;
                    current_block = 573777995569271548;
                    break;
                }
            }
            if isatty(fileno(f)) != 0 {
                loop {
                    if *cancel_flag != 0 {
                        status = 0 as libc::c_int | 0x1 as libc::c_int;
                        current_block = 573777995569271548;
                        break 's_26;
                    } else {
                        ret = wait_file(fileno(f), 10000 as libc::c_int);
                        if ret < 0 as libc::c_int {
                            sixel_helper_set_additional_message(
                                b"sixel_chunk_from_file: wait_file() failed.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            status = 0x1000 as libc::c_int | 0x100 as libc::c_int;
                            current_block = 573777995569271548;
                            break 's_26;
                        } else if ret == 0 as libc::c_int {
                            break;
                        }
                    }
                }
            }
            n = fread(
                ((*pchunk).buffer).offset((*pchunk).size as isize) as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                4096 as libc::c_int as libc::c_ulong,
                f,
            );
            if n == 0 as libc::c_int as libc::c_ulong {
                current_block = 11298138898191919651;
                break;
            }
            (*pchunk)
                .size = ((*pchunk).size as libc::c_ulong).wrapping_add(n) as size_t
                as size_t;
        }
        match current_block {
            573777995569271548 => {}
            _ => {
                if f != stdin {
                    fclose(f);
                }
                status = 0 as libc::c_int;
            }
        }
    }
    return status;
}
unsafe extern "C" fn sixel_chunk_from_url(
    mut url: *const libc::c_char,
    mut pchunk: *mut sixel_chunk_t,
    mut finsecure: libc::c_int,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    sixel_helper_set_additional_message(
        b"To specify URI schemes, you have to configure this program with --with-libcurl option at compile time.\n\0"
            as *const u8 as *const libc::c_char,
    );
    status = 0x1000 as libc::c_int | 0x300 as libc::c_int | 0x1 as libc::c_int;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_chunk_new(
    mut ppchunk: *mut *mut sixel_chunk_t,
    mut filename: *const libc::c_char,
    mut finsecure: libc::c_int,
    mut cancel_flag: *const libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    if ppchunk.is_null() {
        sixel_helper_set_additional_message(
            b"sixel_chunk_new: ppchunk is null.\0" as *const u8 as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x2 as libc::c_int;
    } else if allocator.is_null() {
        sixel_helper_set_additional_message(
            b"sixel_chunk_new: allocator is null.\0" as *const u8 as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x2 as libc::c_int;
    } else {
        *ppchunk = sixel_allocator_malloc(
            allocator,
            ::core::mem::size_of::<sixel_chunk_t>() as libc::c_ulong,
        ) as *mut sixel_chunk_t;
        if (*ppchunk).is_null() {
            sixel_helper_set_additional_message(
                b"sixel_chunk_new: sixel_allocator_malloc() failed.\0" as *const u8
                    as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
        } else {
            (**ppchunk).allocator = allocator;
            status = sixel_chunk_init(
                *ppchunk,
                (1024 as libc::c_int * 32 as libc::c_int) as size_t,
            );
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                sixel_allocator_free(allocator, *ppchunk as *mut libc::c_void);
                *ppchunk = 0 as *mut sixel_chunk_t;
            } else {
                sixel_allocator_ref(allocator);
                if !filename.is_null()
                    && !(strstr(filename, b"://\0" as *const u8 as *const libc::c_char))
                        .is_null()
                {
                    status = sixel_chunk_from_url(filename, *ppchunk, finsecure);
                } else {
                    status = sixel_chunk_from_file(filename, *ppchunk, cancel_flag);
                }
                if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                    sixel_chunk_destroy(*ppchunk);
                    *ppchunk = 0 as *mut sixel_chunk_t;
                } else {
                    status = 0 as libc::c_int;
                }
            }
        }
    }
    return status;
}
