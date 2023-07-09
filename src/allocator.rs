#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
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
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_new(
    mut ppallocator: *mut *mut sixel_allocator_t,
    mut fn_malloc: sixel_malloc_t,
    mut fn_calloc: sixel_calloc_t,
    mut fn_realloc: sixel_realloc_t,
    mut fn_free: sixel_free_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    if ppallocator.is_null() {
        sixel_helper_set_additional_message(
            b"sixel_allocator_new: given argument ppallocator is null.\0" as *const u8
                as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x2 as libc::c_int;
    } else {
        if fn_malloc.is_none() {
            fn_malloc = Some(
                malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void,
            );
        }
        if fn_calloc.is_none() {
            fn_calloc = Some(
                calloc
                    as unsafe extern "C" fn(
                        libc::c_ulong,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            );
        }
        if fn_realloc.is_none() {
            fn_realloc = Some(
                realloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_ulong,
                    ) -> *mut libc::c_void,
            );
        }
        if fn_free.is_none() {
            fn_free = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
        }
        *ppallocator = fn_malloc
            .expect(
                "non-null function pointer",
            )(::core::mem::size_of::<sixel_allocator_t>() as libc::c_ulong)
            as *mut sixel_allocator_t;
        if (*ppallocator).is_null() {
            sixel_helper_set_additional_message(
                b"sixel_allocator_new: fn_malloc() failed.\0" as *const u8
                    as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
        } else {
            (**ppallocator).ref_0 = 1 as libc::c_int as libc::c_uint;
            (**ppallocator).fn_malloc = fn_malloc;
            (**ppallocator).fn_calloc = fn_calloc;
            (**ppallocator).fn_realloc = fn_realloc;
            (**ppallocator).fn_free = fn_free;
            status = 0 as libc::c_int;
        }
    }
    return status;
}
unsafe extern "C" fn sixel_allocator_destroy(mut allocator: *mut sixel_allocator_t) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void sixel_allocator_destroy(sixel_allocator_t *)\0"))
                .as_ptr(),
        );
    };
    if ((*allocator).fn_free).is_some() {} else {
        __assert_fail(
            b"allocator->fn_free\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void sixel_allocator_destroy(sixel_allocator_t *)\0"))
                .as_ptr(),
        );
    };
    ((*allocator).fn_free)
        .expect("non-null function pointer")(allocator as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_ref(mut allocator: *mut sixel_allocator_t) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"void sixel_allocator_ref(sixel_allocator_t *)\0"))
                .as_ptr(),
        );
    };
    (*allocator).ref_0 = ((*allocator).ref_0).wrapping_add(1);
    (*allocator).ref_0;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_unref(mut allocator: *mut sixel_allocator_t) {
    if !allocator.is_null() {
        if (*allocator).ref_0 > 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"allocator->ref > 0\0" as *const u8 as *const libc::c_char,
                b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void sixel_allocator_unref(sixel_allocator_t *)\0"))
                    .as_ptr(),
            );
        };
        (*allocator).ref_0 = ((*allocator).ref_0).wrapping_sub(1);
        (*allocator).ref_0;
        if (*allocator).ref_0 == 0 as libc::c_int as libc::c_uint {
            sixel_allocator_destroy(allocator);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_malloc(
    mut allocator: *mut sixel_allocator_t,
    mut n: size_t,
) -> *mut libc::c_void {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void *sixel_allocator_malloc(sixel_allocator_t *, size_t)\0"))
                .as_ptr(),
        );
    };
    if ((*allocator).fn_malloc).is_some() {} else {
        __assert_fail(
            b"allocator->fn_malloc\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"void *sixel_allocator_malloc(sixel_allocator_t *, size_t)\0"))
                .as_ptr(),
        );
    };
    if n == 0 as libc::c_int as libc::c_ulong {
        sixel_helper_set_additional_message(
            b"sixel_allocator_malloc: called with n == 0\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if n
        > (10248 as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(128 as libc::c_ulong)
    {
        return 0 as *mut libc::c_void;
    }
    return ((*allocator).fn_malloc).expect("non-null function pointer")(n);
}
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_calloc(
    mut allocator: *mut sixel_allocator_t,
    mut nelm: size_t,
    mut elsize: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = 0;
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void *sixel_allocator_calloc(sixel_allocator_t *, size_t, size_t)\0"))
                .as_ptr(),
        );
    };
    if ((*allocator).fn_calloc).is_some() {} else {
        __assert_fail(
            b"allocator->fn_calloc\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 66],
                &[libc::c_char; 66],
            >(b"void *sixel_allocator_calloc(sixel_allocator_t *, size_t, size_t)\0"))
                .as_ptr(),
        );
    };
    n = nelm.wrapping_mul(elsize);
    if n == 0 as libc::c_int as libc::c_ulong {
        sixel_helper_set_additional_message(
            b"sixel_allocator_malloc: called with n == 0\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if n
        > (10248 as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(128 as libc::c_ulong)
    {
        return 0 as *mut libc::c_void;
    }
    return ((*allocator).fn_calloc).expect("non-null function pointer")(nelm, elsize);
}
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_realloc(
    mut allocator: *mut sixel_allocator_t,
    mut p: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void *sixel_allocator_realloc(sixel_allocator_t *, void *, size_t)\0"))
                .as_ptr(),
        );
    };
    if ((*allocator).fn_realloc).is_some() {} else {
        __assert_fail(
            b"allocator->fn_realloc\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"void *sixel_allocator_realloc(sixel_allocator_t *, void *, size_t)\0"))
                .as_ptr(),
        );
    };
    if n == 0 as libc::c_int as libc::c_ulong {
        sixel_helper_set_additional_message(
            b"sixel_allocator_malloc: called with n == 0\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    if n
        > (10248 as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(128 as libc::c_ulong)
    {
        return 0 as *mut libc::c_void;
    }
    return ((*allocator).fn_realloc).expect("non-null function pointer")(p, n);
}
#[no_mangle]
pub unsafe extern "C" fn sixel_allocator_free(
    mut allocator: *mut sixel_allocator_t,
    mut p: *mut libc::c_void,
) {
    if !allocator.is_null() {} else {
        __assert_fail(
            b"allocator\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void sixel_allocator_free(sixel_allocator_t *, void *)\0"))
                .as_ptr(),
        );
    };
    if ((*allocator).fn_free).is_some() {} else {
        __assert_fail(
            b"allocator->fn_free\0" as *const u8 as *const libc::c_char,
            b"../src/allocator.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void sixel_allocator_free(sixel_allocator_t *, void *)\0"))
                .as_ptr(),
        );
    };
    ((*allocator).fn_free).expect("non-null function pointer")(p);
}
