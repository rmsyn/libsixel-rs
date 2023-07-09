#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value, stdsimd, thread_local)]
#[cfg(target_arch = "x86")]
pub use core::arch::x86::{
    __m128i, _mm_load_si128, _mm_packs_epi32, _mm_sub_epi32, _mm_srai_epi32,
    _mm_set_epi32, _mm_set1_epi32, _mm_add_epi32, _mm_madd_epi16, _mm_setr_epi16,
    _mm_storel_epi64, _mm_setzero_si128, _mm_shuffle_epi32, _mm_unpackhi_epi8,
    _mm_srai_epi16, _mm_mulhi_epi16, _mm_xor_si128, _mm_set_epi8, _mm_set1_epi8,
    _mm_storeu_si128, _mm_packus_epi16, _mm_unpacklo_epi16, _mm_srli_epi16,
    _mm_unpackhi_epi16, _mm_add_epi16, _mm_slli_epi16, _mm_sub_epi16, _mm_unpacklo_epi8,
    _mm_loadl_epi64, _mm_set_epi16, _mm_set1_epi16, _mm_slli_si128,
};
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::{
    __m128i, _mm_load_si128, _mm_packs_epi32, _mm_sub_epi32, _mm_srai_epi32,
    _mm_set_epi32, _mm_set1_epi32, _mm_add_epi32, _mm_madd_epi16, _mm_setr_epi16,
    _mm_storel_epi64, _mm_setzero_si128, _mm_shuffle_epi32, _mm_unpackhi_epi8,
    _mm_srai_epi16, _mm_mulhi_epi16, _mm_xor_si128, _mm_set_epi8, _mm_set1_epi8,
    _mm_storeu_si128, _mm_packus_epi16, _mm_unpacklo_epi16, _mm_srli_epi16,
    _mm_unpackhi_epi16, _mm_add_epi16, _mm_slli_epi16, _mm_sub_epi16, _mm_unpacklo_epi8,
    _mm_loadl_epi64, _mm_set_epi16, _mm_set1_epi16, _mm_slli_si128,
};
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    pub type jvirt_barray_control;
    pub type jvirt_sarray_control;
    pub type jpeg_color_quantizer;
    pub type jpeg_color_deconverter;
    pub type jpeg_upsampler;
    pub type jpeg_inverse_dct;
    pub type jpeg_entropy_decoder;
    pub type jpeg_marker_reader;
    pub type jpeg_input_controller;
    pub type jpeg_d_post_controller;
    pub type jpeg_d_coef_controller;
    pub type jpeg_d_main_controller;
    pub type jpeg_decomp_master;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn png_sig_cmp(
        sig: png_const_bytep,
        start: size_t,
        num_to_check: size_t,
    ) -> libc::c_int;
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_set_longjmp_fn(
        png_ptr: png_structrp,
        longjmp_fn: png_longjmp_ptr,
        jmp_buf_size: size_t,
    ) -> *mut jmp_buf;
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_set_palette_to_rgb(png_ptr: png_structrp);
    fn png_set_gray_to_rgb(png_ptr: png_structrp);
    fn png_set_strip_alpha(png_ptr: png_structrp);
    fn png_set_background(
        png_ptr: png_structrp,
        background_color: png_const_color_16p,
        background_gamma_code: libc::c_int,
        need_expand: libc::c_int,
        background_gamma: libc::c_double,
    );
    fn png_set_strip_16(png_ptr: png_structrp);
    fn png_set_read_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        read_data_fn: png_rw_ptr,
    );
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_get_image_width(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_image_height(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn png_get_bit_depth(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_byte;
    fn png_get_color_type(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_byte;
    fn png_get_bKGD(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        background: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut libc::c_int,
    ) -> png_uint_32;
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut libc::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn jpeg_std_error(err: *mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    fn jpeg_CreateDecompress(
        cinfo: j_decompress_ptr,
        version: libc::c_int,
        structsize: size_t,
    );
    fn jpeg_destroy_decompress(cinfo: j_decompress_ptr);
    fn jpeg_mem_src(
        cinfo: j_decompress_ptr,
        inbuffer: *const libc::c_uchar,
        insize: libc::c_ulong,
    );
    fn jpeg_read_header(cinfo: j_decompress_ptr, require_image: boolean) -> libc::c_int;
    fn jpeg_start_decompress(cinfo: j_decompress_ptr) -> boolean;
    fn jpeg_read_scanlines(
        cinfo: j_decompress_ptr,
        scanlines: JSAMPARRAY,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    fn jpeg_finish_decompress(cinfo: j_decompress_ptr) -> boolean;
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
    fn sixel_helper_compute_depth(pixelformat: libc::c_int) -> libc::c_int;
    fn sixel_frame_new(
        ppframe: *mut *mut sixel_frame_t,
        allocator: *mut sixel_allocator_t,
    ) -> SIXELSTATUS;
    fn sixel_frame_unref(frame: *mut sixel_frame_t);
    fn sixel_frame_strip_alpha(
        frame: *mut sixel_frame_t,
        bgcolor: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn sixel_chunk_new(
        ppchunk: *mut *mut sixel_chunk_t,
        filename: *const libc::c_char,
        finsecure: libc::c_int,
        cancel_flag: *const libc::c_int,
        allocator: *mut sixel_allocator_t,
    ) -> SIXELSTATUS;
    fn sixel_chunk_destroy(pchunk: *mut sixel_chunk_t);
    fn load_pnm(
        p: *mut libc::c_uchar,
        len: libc::c_int,
        allocator: *mut sixel_allocator_t,
        result: *mut *mut libc::c_uchar,
        psx: *mut libc::c_int,
        psy: *mut libc::c_int,
        ppalette: *mut *mut libc::c_uchar,
        pncolors: *mut libc::c_int,
        ppixelformat: *mut libc::c_int,
    ) -> SIXELSTATUS;
    fn load_gif(
        buffer: *mut libc::c_uchar,
        size: libc::c_int,
        bgcolor: *mut libc::c_uchar,
        reqcolors: libc::c_int,
        fuse_palette: libc::c_int,
        fstatic: libc::c_int,
        loop_control: libc::c_int,
        fn_load: *mut libc::c_void,
        context: *mut libc::c_void,
        allocator: *mut sixel_allocator_t,
    ) -> libc::c_int;
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type png_byte = libc::c_uchar;
pub type png_uint_16 = libc::c_ushort;
pub type png_uint_32 = libc::c_uint;
pub type png_size_t = size_t;
pub type png_voidp = *mut libc::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_bytep = *const png_byte;
pub type png_const_charp = *const libc::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_structpp = *mut *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_infopp = *mut *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_color = png_color_struct;
pub type png_colorp = *mut png_color;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_color_16 = png_color_16_struct;
pub type png_color_16p = *mut png_color_16;
pub type png_const_color_16p = *const png_color_16;
pub type png_error_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_const_charp) -> (),
>;
pub type png_rw_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_bytep, size_t) -> (),
>;
pub type png_longjmp_ptr = Option::<
    unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> (),
>;
pub type JSAMPLE = libc::c_uchar;
pub type JCOEF = libc::c_short;
pub type JOCTET = libc::c_uchar;
pub type UINT8 = libc::c_uchar;
pub type UINT16 = libc::c_ushort;
pub type JDIMENSION = libc::c_uint;
pub type boolean = libc::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JBLOCK = [JCOEF; 64];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_component_info {
    pub component_id: libc::c_int,
    pub component_index: libc::c_int,
    pub h_samp_factor: libc::c_int,
    pub v_samp_factor: libc::c_int,
    pub quant_tbl_no: libc::c_int,
    pub dc_tbl_no: libc::c_int,
    pub ac_tbl_no: libc::c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_h_scaled_size: libc::c_int,
    pub DCT_v_scaled_size: libc::c_int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: libc::c_int,
    pub MCU_height: libc::c_int,
    pub MCU_blocks: libc::c_int,
    pub MCU_sample_width: libc::c_int,
    pub last_col_width: libc::c_int,
    pub last_row_height: libc::c_int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_marker_struct {
    pub next: jpeg_saved_marker_ptr,
    pub marker: UINT8,
    pub original_length: libc::c_uint,
    pub data_length: libc::c_uint,
    pub data: *mut JOCTET,
}
pub type jpeg_saved_marker_ptr = *mut jpeg_marker_struct;
pub type J_COLOR_SPACE = libc::c_uint;
pub const JCS_RGB565: J_COLOR_SPACE = 16;
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
pub const JCS_YCCK: J_COLOR_SPACE = 5;
pub const JCS_CMYK: J_COLOR_SPACE = 4;
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
pub const JCS_RGB: J_COLOR_SPACE = 2;
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
pub type J_DCT_METHOD = libc::c_uint;
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub const JDCT_IFAST: J_DCT_METHOD = 1;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
pub type J_DITHER_MODE = libc::c_uint;
pub const JDITHER_FS: J_DITHER_MODE = 2;
pub const JDITHER_ORDERED: J_DITHER_MODE = 1;
pub const JDITHER_NONE: J_DITHER_MODE = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: boolean,
    pub global_state: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option::<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub pass_counter: libc::c_long,
    pub pass_limit: libc::c_long,
    pub completed_passes: libc::c_int,
    pub total_passes: libc::c_int,
}
pub type j_common_ptr = *mut jpeg_common_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option::<
        unsafe extern "C" fn(j_common_ptr, libc::c_int, size_t) -> *mut libc::c_void,
    >,
    pub alloc_large: Option::<
        unsafe extern "C" fn(j_common_ptr, libc::c_int, size_t) -> *mut libc::c_void,
    >,
    pub alloc_sarray: Option::<
        unsafe extern "C" fn(
            j_common_ptr,
            libc::c_int,
            JDIMENSION,
            JDIMENSION,
        ) -> JSAMPARRAY,
    >,
    pub alloc_barray: Option::<
        unsafe extern "C" fn(
            j_common_ptr,
            libc::c_int,
            JDIMENSION,
            JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option::<
        unsafe extern "C" fn(
            j_common_ptr,
            libc::c_int,
            boolean,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option::<
        unsafe extern "C" fn(
            j_common_ptr,
            libc::c_int,
            boolean,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option::<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub access_virt_sarray: Option::<
        unsafe extern "C" fn(
            j_common_ptr,
            jvirt_sarray_ptr,
            JDIMENSION,
            JDIMENSION,
            boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: Option::<
        unsafe extern "C" fn(
            j_common_ptr,
            jvirt_barray_ptr,
            JDIMENSION,
            JDIMENSION,
            boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: Option::<unsafe extern "C" fn(j_common_ptr, libc::c_int) -> ()>,
    pub self_destruct: Option::<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub max_memory_to_use: libc::c_long,
    pub max_alloc_chunk: libc::c_long,
}
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_error_mgr {
    pub error_exit: Option::<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub emit_message: Option::<unsafe extern "C" fn(j_common_ptr, libc::c_int) -> ()>,
    pub output_message: Option::<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub format_message: Option::<
        unsafe extern "C" fn(j_common_ptr, *mut libc::c_char) -> (),
    >,
    pub reset_error_mgr: Option::<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub msg_code: libc::c_int,
    pub msg_parm: C2RustUnnamed,
    pub trace_level: libc::c_int,
    pub num_warnings: libc::c_long,
    pub jpeg_message_table: *const *const libc::c_char,
    pub last_jpeg_message: libc::c_int,
    pub addon_message_table: *const *const libc::c_char,
    pub first_addon_message: libc::c_int,
    pub last_addon_message: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: [libc::c_int; 8],
    pub s: [libc::c_char; 80],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_decompress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut libc::c_void,
    pub is_decompressor: boolean,
    pub global_state: libc::c_int,
    pub src: *mut jpeg_source_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: libc::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: libc::c_uint,
    pub scale_denom: libc::c_uint,
    pub output_gamma: libc::c_double,
    pub buffered_image: boolean,
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    pub quantize_colors: boolean,
    pub dither_mode: J_DITHER_MODE,
    pub two_pass_quantize: boolean,
    pub desired_number_of_colors: libc::c_int,
    pub enable_1pass_quant: boolean,
    pub enable_external_quant: boolean,
    pub enable_2pass_quant: boolean,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: libc::c_int,
    pub output_components: libc::c_int,
    pub rec_outbuf_height: libc::c_int,
    pub actual_number_of_colors: libc::c_int,
    pub colormap: JSAMPARRAY,
    pub output_scanline: JDIMENSION,
    pub input_scan_number: libc::c_int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: libc::c_int,
    pub output_iMCU_row: JDIMENSION,
    pub coef_bits: *mut [libc::c_int; 64],
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub data_precision: libc::c_int,
    pub comp_info: *mut jpeg_component_info,
    pub is_baseline: boolean,
    pub progressive_mode: boolean,
    pub arith_code: boolean,
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub restart_interval: libc::c_uint,
    pub saw_JFIF_marker: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub saw_Adobe_marker: boolean,
    pub Adobe_transform: UINT8,
    pub CCIR601_sampling: boolean,
    pub marker_list: jpeg_saved_marker_ptr,
    pub max_h_samp_factor: libc::c_int,
    pub max_v_samp_factor: libc::c_int,
    pub min_DCT_h_scaled_size: libc::c_int,
    pub min_DCT_v_scaled_size: libc::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub sample_range_limit: *mut JSAMPLE,
    pub comps_in_scan: libc::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: libc::c_int,
    pub MCU_membership: [libc::c_int; 10],
    pub Ss: libc::c_int,
    pub Se: libc::c_int,
    pub Ah: libc::c_int,
    pub Al: libc::c_int,
    pub block_size: libc::c_int,
    pub natural_order: *const libc::c_int,
    pub lim_Se: libc::c_int,
    pub unread_marker: libc::c_int,
    pub master: *mut jpeg_decomp_master,
    pub main: *mut jpeg_d_main_controller,
    pub coef: *mut jpeg_d_coef_controller,
    pub post: *mut jpeg_d_post_controller,
    pub inputctl: *mut jpeg_input_controller,
    pub marker: *mut jpeg_marker_reader,
    pub entropy: *mut jpeg_entropy_decoder,
    pub idct: *mut jpeg_inverse_dct,
    pub upsample: *mut jpeg_upsampler,
    pub cconvert: *mut jpeg_color_deconverter,
    pub cquantize: *mut jpeg_color_quantizer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const JOCTET,
    pub bytes_in_buffer: size_t,
    pub init_source: Option::<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
    pub fill_input_buffer: Option::<unsafe extern "C" fn(j_decompress_ptr) -> boolean>,
    pub skip_input_data: Option::<
        unsafe extern "C" fn(j_decompress_ptr, libc::c_long) -> (),
    >,
    pub resync_to_restart: Option::<
        unsafe extern "C" fn(j_decompress_ptr, libc::c_int) -> boolean,
    >,
    pub term_source: Option::<unsafe extern "C" fn(j_decompress_ptr) -> ()>,
}
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
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
pub type sixel_chunk_t = sixel_chunk;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sixel_chunk {
    pub buffer: *mut libc::c_uchar,
    pub size: size_t,
    pub max_size: size_t,
    pub allocator: *mut sixel_allocator_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__context {
    pub img_x: stbi__uint32,
    pub img_y: stbi__uint32,
    pub img_n: libc::c_int,
    pub img_out_n: libc::c_int,
    pub io: stbi_io_callbacks,
    pub io_user_data: *mut libc::c_void,
    pub read_from_callbacks: libc::c_int,
    pub buflen: libc::c_int,
    pub buffer_start: [stbi_uc; 128],
    pub callback_already_read: libc::c_int,
    pub img_buffer: *mut stbi_uc,
    pub img_buffer_end: *mut stbi_uc,
    pub img_buffer_original: *mut stbi_uc,
    pub img_buffer_original_end: *mut stbi_uc,
}
pub type stbi_uc = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi_io_callbacks {
    pub read: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub skip: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
    pub eof: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
}
pub type stbi__uint32 = uint32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__result_info {
    pub bits_per_channel: libc::c_int,
    pub num_channels: libc::c_int,
    pub channel_order: libc::c_int,
}
pub type stbi__uint16 = uint16_t;
pub type uint16_t = __uint16_t;
pub const STBI_rgb: C2RustUnnamed_1 = 3;
pub const STBI_grey_alpha: C2RustUnnamed_1 = 2;
pub const STBI_grey: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__jpeg {
    pub s: *mut stbi__context,
    pub huff_dc: [stbi__huffman; 4],
    pub huff_ac: [stbi__huffman; 4],
    pub dequant: [[stbi__uint16; 64]; 4],
    pub fast_ac: [[stbi__int16; 512]; 4],
    pub img_h_max: libc::c_int,
    pub img_v_max: libc::c_int,
    pub img_mcu_x: libc::c_int,
    pub img_mcu_y: libc::c_int,
    pub img_mcu_w: libc::c_int,
    pub img_mcu_h: libc::c_int,
    pub img_comp: [C2RustUnnamed_0; 4],
    pub code_buffer: stbi__uint32,
    pub code_bits: libc::c_int,
    pub marker: libc::c_uchar,
    pub nomore: libc::c_int,
    pub progressive: libc::c_int,
    pub spec_start: libc::c_int,
    pub spec_end: libc::c_int,
    pub succ_high: libc::c_int,
    pub succ_low: libc::c_int,
    pub eob_run: libc::c_int,
    pub jfif: libc::c_int,
    pub app14_color_transform: libc::c_int,
    pub rgb: libc::c_int,
    pub scan_n: libc::c_int,
    pub order: [libc::c_int; 4],
    pub restart_interval: libc::c_int,
    pub todo: libc::c_int,
    pub idct_block_kernel: Option::<
        unsafe extern "C" fn(*mut stbi_uc, libc::c_int, *mut libc::c_short) -> (),
    >,
    pub YCbCr_to_RGB_kernel: Option::<
        unsafe extern "C" fn(
            *mut stbi_uc,
            *const stbi_uc,
            *const stbi_uc,
            *const stbi_uc,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub resample_row_hv_2_kernel: Option::<
        unsafe extern "C" fn(
            *mut stbi_uc,
            *mut stbi_uc,
            *mut stbi_uc,
            libc::c_int,
            libc::c_int,
        ) -> *mut stbi_uc,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub id: libc::c_int,
    pub h: libc::c_int,
    pub v: libc::c_int,
    pub tq: libc::c_int,
    pub hd: libc::c_int,
    pub ha: libc::c_int,
    pub dc_pred: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub w2: libc::c_int,
    pub h2: libc::c_int,
    pub data: *mut stbi_uc,
    pub raw_data: *mut libc::c_void,
    pub raw_coeff: *mut libc::c_void,
    pub linebuf: *mut stbi_uc,
    pub coeff: *mut libc::c_short,
    pub coeff_w: libc::c_int,
    pub coeff_h: libc::c_int,
}
pub type stbi__int16 = int16_t;
pub type int16_t = __int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__huffman {
    pub fast: [stbi_uc; 512],
    pub code: [stbi__uint16; 256],
    pub values: [stbi_uc; 256],
    pub size: [stbi_uc; 257],
    pub maxcode: [libc::c_uint; 18],
    pub delta: [libc::c_int; 17],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__resample {
    pub resample: resample_row_func,
    pub line0: *mut stbi_uc,
    pub line1: *mut stbi_uc,
    pub hs: libc::c_int,
    pub vs: libc::c_int,
    pub w_lores: libc::c_int,
    pub ystep: libc::c_int,
    pub ypos: libc::c_int,
}
pub type resample_row_func = Option::<
    unsafe extern "C" fn(
        *mut stbi_uc,
        *mut stbi_uc,
        *mut stbi_uc,
        libc::c_int,
        libc::c_int,
    ) -> *mut stbi_uc,
>;
pub const STBI__SCAN_load: C2RustUnnamed_3 = 0;
pub const STBI__SCAN_type: C2RustUnnamed_3 = 1;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __mm_loadl_epi64_struct {
    pub __u: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si128 {
    pub __v: __m128i_u,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __mm_storel_epi64_struct {
    pub __u: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__pic_packet {
    pub size: stbi_uc,
    pub type_0: stbi_uc,
    pub channel: stbi_uc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__bmp_data {
    pub bpp: libc::c_int,
    pub offset: libc::c_int,
    pub hsz: libc::c_int,
    pub mr: libc::c_uint,
    pub mg: libc::c_uint,
    pub mb: libc::c_uint,
    pub ma: libc::c_uint,
    pub all_a: libc::c_uint,
    pub extra_read: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__png {
    pub s: *mut stbi__context,
    pub idata: *mut stbi_uc,
    pub expanded: *mut stbi_uc,
    pub out: *mut stbi_uc,
    pub depth: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__pngchunk {
    pub length: stbi__uint32,
    pub type_0: stbi__uint32,
}
pub const STBI__F_paeth_first: C2RustUnnamed_4 = 6;
pub const STBI__F_avg_first: C2RustUnnamed_4 = 5;
pub const STBI__F_paeth: C2RustUnnamed_4 = 4;
pub const STBI__F_avg: C2RustUnnamed_4 = 3;
pub const STBI__F_up: C2RustUnnamed_4 = 2;
pub const STBI__F_sub: C2RustUnnamed_4 = 1;
pub const STBI__F_none: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__zbuf {
    pub zbuffer: *mut stbi_uc,
    pub zbuffer_end: *mut stbi_uc,
    pub num_bits: libc::c_int,
    pub code_buffer: stbi__uint32,
    pub zout: *mut libc::c_char,
    pub zout_start: *mut libc::c_char,
    pub zout_end: *mut libc::c_char,
    pub z_expandable: libc::c_int,
    pub z_length: stbi__zhuffman,
    pub z_distance: stbi__zhuffman,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stbi__zhuffman {
    pub fast: [stbi__uint16; 512],
    pub firstcode: [stbi__uint16; 16],
    pub maxcode: [libc::c_int; 17],
    pub firstsymbol: [stbi__uint16; 16],
    pub size: [stbi_uc; 288],
    pub value: [stbi__uint16; 288],
}
pub const STBI__SCAN_header: C2RustUnnamed_3 = 2;
pub const STBI_ORDER_RGB: C2RustUnnamed_2 = 0;
pub type fn_pointer = _fn_pointer;
#[derive(Copy, Clone)]
#[repr(C)]
pub union _fn_pointer {
    pub fn_0: sixel_load_image_function,
    pub p: *mut libc::c_void,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const STBI_rgb_alpha: C2RustUnnamed_1 = 4;
pub const STBI_default: C2RustUnnamed_1 = 0;
pub type stbi_us = libc::c_ushort;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const STBI_ORDER_BGR: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
#[no_mangle]
pub static mut stbi_allocator: *mut sixel_allocator_t = 0 as *const sixel_allocator_t
    as *mut sixel_allocator_t;
#[no_mangle]
pub unsafe extern "C" fn stbi_malloc(mut n: size_t) -> *mut libc::c_void {
    return sixel_allocator_malloc(stbi_allocator, n);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_realloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    return sixel_allocator_realloc(stbi_allocator, p, n);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_free(mut p: *mut libc::c_void) {
    sixel_allocator_free(stbi_allocator, p);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_set_flip_vertically_on_load(
    mut flag_true_if_should_flip: libc::c_int,
) {
    stbi__vertically_flip_on_load_global = flag_true_if_should_flip;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_convert_iphone_png_to_rgb_thread(
    mut flag_true_if_should_convert: libc::c_int,
) {
    stbi__de_iphone_flag_local = flag_true_if_should_convert;
    stbi__de_iphone_flag_set = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_set_flip_vertically_on_load_thread(
    mut flag_true_if_should_flip: libc::c_int,
) {
    stbi__vertically_flip_on_load_local = flag_true_if_should_flip;
    stbi__vertically_flip_on_load_set = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_zlib_decode_malloc_guesssize(
    mut buffer: *const libc::c_char,
    mut len: libc::c_int,
    mut initial_size: libc::c_int,
    mut outlen: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut a: stbi__zbuf = stbi__zbuf {
        zbuffer: 0 as *mut stbi_uc,
        zbuffer_end: 0 as *mut stbi_uc,
        num_bits: 0,
        code_buffer: 0,
        zout: 0 as *mut libc::c_char,
        zout_start: 0 as *mut libc::c_char,
        zout_end: 0 as *mut libc::c_char,
        z_expandable: 0,
        z_length: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
        z_distance: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
    };
    let mut p: *mut libc::c_char = stbi__malloc(initial_size as size_t)
        as *mut libc::c_char;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    a.zbuffer = buffer as *mut stbi_uc;
    a.zbuffer_end = (buffer as *mut stbi_uc).offset(len as isize);
    if stbi__do_zlib(&mut a, p, initial_size, 1 as libc::c_int, 1 as libc::c_int) != 0 {
        if !outlen.is_null() {
            *outlen = (a.zout).offset_from(a.zout_start) as libc::c_long as libc::c_int;
        }
        return a.zout_start;
    } else {
        stbi_free(a.zout_start as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn stbi_zlib_decode_malloc(
    mut buffer: *const libc::c_char,
    mut len: libc::c_int,
    mut outlen: *mut libc::c_int,
) -> *mut libc::c_char {
    return stbi_zlib_decode_malloc_guesssize(buffer, len, 16384 as libc::c_int, outlen);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_zlib_decode_buffer(
    mut obuffer: *mut libc::c_char,
    mut olen: libc::c_int,
    mut ibuffer: *const libc::c_char,
    mut ilen: libc::c_int,
) -> libc::c_int {
    let mut a: stbi__zbuf = stbi__zbuf {
        zbuffer: 0 as *mut stbi_uc,
        zbuffer_end: 0 as *mut stbi_uc,
        num_bits: 0,
        code_buffer: 0,
        zout: 0 as *mut libc::c_char,
        zout_start: 0 as *mut libc::c_char,
        zout_end: 0 as *mut libc::c_char,
        z_expandable: 0,
        z_length: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
        z_distance: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
    };
    a.zbuffer = ibuffer as *mut stbi_uc;
    a.zbuffer_end = (ibuffer as *mut stbi_uc).offset(ilen as isize);
    if stbi__do_zlib(&mut a, obuffer, olen, 0 as libc::c_int, 1 as libc::c_int) != 0 {
        return (a.zout).offset_from(a.zout_start) as libc::c_long as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn stbi_zlib_decode_noheader_malloc(
    mut buffer: *const libc::c_char,
    mut len: libc::c_int,
    mut outlen: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut a: stbi__zbuf = stbi__zbuf {
        zbuffer: 0 as *mut stbi_uc,
        zbuffer_end: 0 as *mut stbi_uc,
        num_bits: 0,
        code_buffer: 0,
        zout: 0 as *mut libc::c_char,
        zout_start: 0 as *mut libc::c_char,
        zout_end: 0 as *mut libc::c_char,
        z_expandable: 0,
        z_length: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
        z_distance: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
    };
    let mut p: *mut libc::c_char = stbi__malloc(16384 as libc::c_int as size_t)
        as *mut libc::c_char;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    a.zbuffer = buffer as *mut stbi_uc;
    a.zbuffer_end = (buffer as *mut stbi_uc).offset(len as isize);
    if stbi__do_zlib(&mut a, p, 16384 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int)
        != 0
    {
        if !outlen.is_null() {
            *outlen = (a.zout).offset_from(a.zout_start) as libc::c_long as libc::c_int;
        }
        return a.zout_start;
    } else {
        stbi_free(a.zout_start as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn stbi_zlib_decode_noheader_buffer(
    mut obuffer: *mut libc::c_char,
    mut olen: libc::c_int,
    mut ibuffer: *const libc::c_char,
    mut ilen: libc::c_int,
) -> libc::c_int {
    let mut a: stbi__zbuf = stbi__zbuf {
        zbuffer: 0 as *mut stbi_uc,
        zbuffer_end: 0 as *mut stbi_uc,
        num_bits: 0,
        code_buffer: 0,
        zout: 0 as *mut libc::c_char,
        zout_start: 0 as *mut libc::c_char,
        zout_end: 0 as *mut libc::c_char,
        z_expandable: 0,
        z_length: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
        z_distance: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
    };
    a.zbuffer = ibuffer as *mut stbi_uc;
    a.zbuffer_end = (ibuffer as *mut stbi_uc).offset(ilen as isize);
    if stbi__do_zlib(&mut a, obuffer, olen, 0 as libc::c_int, 0 as libc::c_int) != 0 {
        return (a.zout).offset_from(a.zout_start) as libc::c_long as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn stbi_ldr_to_hdr_scale(mut scale: libc::c_float) {
    stbi__l2h_scale = scale;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_is_hdr_from_callbacks(
    mut clbk: *const stbi_io_callbacks,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_callbacks(&mut s, clbk as *mut stbi_io_callbacks, user);
    return stbi__hdr_test(&mut s);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_is_hdr_from_memory(
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_mem(&mut s, buffer, len);
    return stbi__hdr_test(&mut s);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_image_free(mut retval_from_stbi_load: *mut libc::c_void) {
    stbi_free(retval_from_stbi_load);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_info_from_memory(
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_mem(&mut s, buffer, len);
    return stbi__info_main(&mut s, x, y, comp);
}
unsafe extern "C" fn stbi__info_main(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    if stbi__jpeg_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__png_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__bmp_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__psd_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__pic_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__hdr_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__tga_info(s, x, y, comp) != 0 {
        return 1 as libc::c_int;
    }
    return stbi__err(
        b"Image not of any known type, or corrupt\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn stbi__tga_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut tga_w: libc::c_int = 0;
    let mut tga_h: libc::c_int = 0;
    let mut tga_comp: libc::c_int = 0;
    let mut tga_image_type: libc::c_int = 0;
    let mut tga_bits_per_pixel: libc::c_int = 0;
    let mut tga_colormap_bpp: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    let mut tga_colormap_type: libc::c_int = 0;
    stbi__get8(s);
    tga_colormap_type = stbi__get8(s) as libc::c_int;
    if tga_colormap_type > 1 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    tga_image_type = stbi__get8(s) as libc::c_int;
    if tga_colormap_type == 1 as libc::c_int {
        if tga_image_type != 1 as libc::c_int && tga_image_type != 9 as libc::c_int {
            stbi__rewind(s);
            return 0 as libc::c_int;
        }
        stbi__skip(s, 4 as libc::c_int);
        sz = stbi__get8(s) as libc::c_int;
        if sz != 8 as libc::c_int && sz != 15 as libc::c_int && sz != 16 as libc::c_int
            && sz != 24 as libc::c_int && sz != 32 as libc::c_int
        {
            stbi__rewind(s);
            return 0 as libc::c_int;
        }
        stbi__skip(s, 4 as libc::c_int);
        tga_colormap_bpp = sz;
    } else {
        if tga_image_type != 2 as libc::c_int && tga_image_type != 3 as libc::c_int
            && tga_image_type != 10 as libc::c_int && tga_image_type != 11 as libc::c_int
        {
            stbi__rewind(s);
            return 0 as libc::c_int;
        }
        stbi__skip(s, 9 as libc::c_int);
        tga_colormap_bpp = 0 as libc::c_int;
    }
    tga_w = stbi__get16le(s);
    if tga_w < 1 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    tga_h = stbi__get16le(s);
    if tga_h < 1 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    tga_bits_per_pixel = stbi__get8(s) as libc::c_int;
    stbi__get8(s);
    if tga_colormap_bpp != 0 as libc::c_int {
        if tga_bits_per_pixel != 8 as libc::c_int
            && tga_bits_per_pixel != 16 as libc::c_int
        {
            stbi__rewind(s);
            return 0 as libc::c_int;
        }
        tga_comp = stbi__tga_get_comp(
            tga_colormap_bpp,
            0 as libc::c_int,
            0 as *mut libc::c_int,
        );
    } else {
        tga_comp = stbi__tga_get_comp(
            tga_bits_per_pixel,
            (tga_image_type == 3 as libc::c_int || tga_image_type == 11 as libc::c_int)
                as libc::c_int,
            0 as *mut libc::c_int,
        );
    }
    if tga_comp == 0 {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    if !x.is_null() {
        *x = tga_w;
    }
    if !y.is_null() {
        *y = tga_h;
    }
    if !comp.is_null() {
        *comp = tga_comp;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__hdr_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valid: libc::c_int = 0 as libc::c_int;
    let mut dummy: libc::c_int = 0;
    if x.is_null() {
        x = &mut dummy;
    }
    if y.is_null() {
        y = &mut dummy;
    }
    if comp.is_null() {
        comp = &mut dummy;
    }
    if stbi__hdr_test(s) == 0 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    loop {
        token = stbi__hdr_gettoken(s, buffer.as_mut_ptr());
        if *token.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            break;
        }
        if strcmp(token, b"FORMAT=32-bit_rle_rgbe\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            valid = 1 as libc::c_int;
        }
    }
    if valid == 0 {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    token = stbi__hdr_gettoken(s, buffer.as_mut_ptr());
    if strncmp(
        token,
        b"-Y \0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    token = token.offset(3 as libc::c_int as isize);
    *y = strtol(token, &mut token, 10 as libc::c_int) as libc::c_int;
    while *token as libc::c_int == ' ' as i32 {
        token = token.offset(1);
        token;
    }
    if strncmp(
        token,
        b"+X \0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    token = token.offset(3 as libc::c_int as isize);
    *x = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int) as libc::c_int;
    *comp = 3 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__pic_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut act_comp: libc::c_int = 0 as libc::c_int;
    let mut num_packets: libc::c_int = 0 as libc::c_int;
    let mut chained: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut packets: [stbi__pic_packet; 10] = [stbi__pic_packet {
        size: 0,
        type_0: 0,
        channel: 0,
    }; 10];
    if x.is_null() {
        x = &mut dummy;
    }
    if y.is_null() {
        y = &mut dummy;
    }
    if comp.is_null() {
        comp = &mut dummy;
    }
    if stbi__pic_is4(s, b"S\x80\xF64\0" as *const u8 as *const libc::c_char) == 0 {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    stbi__skip(s, 88 as libc::c_int);
    *x = stbi__get16be(s);
    *y = stbi__get16be(s);
    if stbi__at_eof(s) != 0 {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    if *x != 0 as libc::c_int && ((1 as libc::c_int) << 28 as libc::c_int) / *x < *y {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    stbi__skip(s, 8 as libc::c_int);
    loop {
        let mut packet: *mut stbi__pic_packet = 0 as *mut stbi__pic_packet;
        if num_packets as libc::c_ulong
            == (::core::mem::size_of::<[stbi__pic_packet; 10]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<stbi__pic_packet>() as libc::c_ulong,
                )
        {
            return 0 as libc::c_int;
        }
        let fresh0 = num_packets;
        num_packets = num_packets + 1;
        packet = &mut *packets.as_mut_ptr().offset(fresh0 as isize)
            as *mut stbi__pic_packet;
        chained = stbi__get8(s) as libc::c_int;
        (*packet).size = stbi__get8(s);
        (*packet).type_0 = stbi__get8(s);
        (*packet).channel = stbi__get8(s);
        act_comp |= (*packet).channel as libc::c_int;
        if stbi__at_eof(s) != 0 {
            stbi__rewind(s);
            return 0 as libc::c_int;
        }
        if (*packet).size as libc::c_int != 8 as libc::c_int {
            stbi__rewind(s);
            return 0 as libc::c_int;
        }
        if !(chained != 0) {
            break;
        }
    }
    *comp = if act_comp & 0x10 as libc::c_int != 0 {
        4 as libc::c_int
    } else {
        3 as libc::c_int
    };
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__psd_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut channelCount: libc::c_int = 0;
    let mut dummy: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    if x.is_null() {
        x = &mut dummy;
    }
    if y.is_null() {
        y = &mut dummy;
    }
    if comp.is_null() {
        comp = &mut dummy;
    }
    if stbi__get32be(s) != 0x38425053 as libc::c_int as libc::c_uint {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    if stbi__get16be(s) != 1 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    stbi__skip(s, 6 as libc::c_int);
    channelCount = stbi__get16be(s);
    if channelCount < 0 as libc::c_int || channelCount > 16 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    *y = stbi__get32be(s) as libc::c_int;
    *x = stbi__get32be(s) as libc::c_int;
    depth = stbi__get16be(s);
    if depth != 8 as libc::c_int && depth != 16 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    if stbi__get16be(s) != 3 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    *comp = 4 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__bmp_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut info: stbi__bmp_data = stbi__bmp_data {
        bpp: 0,
        offset: 0,
        hsz: 0,
        mr: 0,
        mg: 0,
        mb: 0,
        ma: 0,
        all_a: 0,
        extra_read: 0,
    };
    info.all_a = 255 as libc::c_int as libc::c_uint;
    p = stbi__bmp_parse_header(s, &mut info);
    if p.is_null() {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    if !x.is_null() {
        *x = (*s).img_x as libc::c_int;
    }
    if !y.is_null() {
        *y = (*s).img_y as libc::c_int;
    }
    if !comp.is_null() {
        if info.bpp == 24 as libc::c_int && info.ma == 0xff000000 as libc::c_uint {
            *comp = 3 as libc::c_int;
        } else {
            *comp = if info.ma != 0 { 4 as libc::c_int } else { 3 as libc::c_int };
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__png_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut p: stbi__png = stbi__png {
        s: 0 as *mut stbi__context,
        idata: 0 as *mut stbi_uc,
        expanded: 0 as *mut stbi_uc,
        out: 0 as *mut stbi_uc,
        depth: 0,
    };
    p.s = s;
    return stbi__png_info_raw(&mut p, x, y, comp);
}
unsafe extern "C" fn stbi__png_info_raw(
    mut p: *mut stbi__png,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    if stbi__parse_png_file(p, STBI__SCAN_header as libc::c_int, 0 as libc::c_int) == 0 {
        stbi__rewind((*p).s);
        return 0 as libc::c_int;
    }
    if !x.is_null() {
        *x = (*(*p).s).img_x as libc::c_int;
    }
    if !y.is_null() {
        *y = (*(*p).s).img_y as libc::c_int;
    }
    if !comp.is_null() {
        *comp = (*(*p).s).img_n;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_info(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut j: *mut stbi__jpeg = stbi__malloc(
        ::core::mem::size_of::<stbi__jpeg>() as libc::c_ulong,
    ) as *mut stbi__jpeg;
    if j.is_null() {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    (*j).s = s;
    result = stbi__jpeg_info_raw(j, x, y, comp);
    stbi_free(j as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn stbi__jpeg_info_raw(
    mut j: *mut stbi__jpeg,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    if stbi__decode_jpeg_header(j, STBI__SCAN_header as libc::c_int) == 0 {
        stbi__rewind((*j).s);
        return 0 as libc::c_int;
    }
    if !x.is_null() {
        *x = (*(*j).s).img_x as libc::c_int;
    }
    if !y.is_null() {
        *y = (*(*j).s).img_y as libc::c_int;
    }
    if !comp.is_null() {
        *comp = if (*(*j).s).img_n >= 3 as libc::c_int {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_info_from_callbacks(
    mut c: *const stbi_io_callbacks,
    mut user: *mut libc::c_void,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
) -> libc::c_int {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_callbacks(&mut s, c as *mut stbi_io_callbacks, user);
    return stbi__info_main(&mut s, x, y, comp);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_is_16_bit_from_memory(
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_mem(&mut s, buffer, len);
    return stbi__is_16_main(&mut s);
}
unsafe extern "C" fn stbi__is_16_main(mut s: *mut stbi__context) -> libc::c_int {
    if stbi__png_is16(s) != 0 {
        return 1 as libc::c_int;
    }
    if stbi__psd_is16(s) != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stbi__psd_is16(mut s: *mut stbi__context) -> libc::c_int {
    let mut channelCount: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    if stbi__get32be(s) != 0x38425053 as libc::c_int as libc::c_uint {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    if stbi__get16be(s) != 1 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    stbi__skip(s, 6 as libc::c_int);
    channelCount = stbi__get16be(s);
    if channelCount < 0 as libc::c_int || channelCount > 16 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    depth = stbi__get16be(s);
    if depth != 16 as libc::c_int {
        stbi__rewind(s);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__png_is16(mut s: *mut stbi__context) -> libc::c_int {
    let mut p: stbi__png = stbi__png {
        s: 0 as *mut stbi__context,
        idata: 0 as *mut stbi_uc,
        expanded: 0 as *mut stbi_uc,
        out: 0 as *mut stbi_uc,
        depth: 0,
    };
    p.s = s;
    if stbi__png_info_raw(
        &mut p,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if p.depth != 16 as libc::c_int {
        stbi__rewind(p.s);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_is_16_bit_from_callbacks(
    mut c: *const stbi_io_callbacks,
    mut user: *mut libc::c_void,
) -> libc::c_int {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_callbacks(&mut s, c as *mut stbi_io_callbacks, user);
    return stbi__is_16_main(&mut s);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_set_unpremultiply_on_load(
    mut flag_true_if_should_unpremultiply: libc::c_int,
) {
    stbi__unpremultiply_on_load_global = flag_true_if_should_unpremultiply;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_hdr_to_ldr_scale(mut scale: libc::c_float) {
    stbi__h2l_scale_i = 1 as libc::c_int as libc::c_float / scale;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_ldr_to_hdr_gamma(mut gamma: libc::c_float) {
    stbi__l2h_gamma = gamma;
}
unsafe extern "C" fn stbi__do_zlib(
    mut a: *mut stbi__zbuf,
    mut obuf: *mut libc::c_char,
    mut olen: libc::c_int,
    mut exp: libc::c_int,
    mut parse_header: libc::c_int,
) -> libc::c_int {
    (*a).zout_start = obuf;
    (*a).zout = obuf;
    (*a).zout_end = obuf.offset(olen as isize);
    (*a).z_expandable = exp;
    return stbi__parse_zlib(a, parse_header);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_hdr_to_ldr_gamma(mut gamma: libc::c_float) {
    stbi__h2l_gamma_i = 1 as libc::c_int as libc::c_float / gamma;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_loadf_from_callbacks(
    mut clbk: *const stbi_io_callbacks,
    mut user: *mut libc::c_void,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut libc::c_float {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_callbacks(&mut s, clbk as *mut stbi_io_callbacks, user);
    return stbi__loadf_main(&mut s, x, y, comp, req_comp);
}
unsafe extern "C" fn stbi__float_postprocess(
    mut result: *mut libc::c_float,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) {
    if (if stbi__vertically_flip_on_load_set != 0 {
        stbi__vertically_flip_on_load_local
    } else {
        stbi__vertically_flip_on_load_global
    }) != 0 && !result.is_null()
    {
        let mut channels: libc::c_int = if req_comp != 0 { req_comp } else { *comp };
        stbi__vertical_flip(
            result as *mut libc::c_void,
            *x,
            *y,
            (channels as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
                as libc::c_int,
        );
    }
}
static mut stbi__l2h_gamma: libc::c_float = 2.2f32;
static mut stbi__l2h_scale: libc::c_float = 1.0f32;
unsafe extern "C" fn stbi__ldr_to_hdr(
    mut data: *mut stbi_uc,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut comp: libc::c_int,
) -> *mut libc::c_float {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut output: *mut libc::c_float = 0 as *mut libc::c_float;
    if data.is_null() {
        return 0 as *mut libc::c_float;
    }
    output = stbi__malloc_mad4(
        x,
        y,
        comp,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong as libc::c_int,
        0 as libc::c_int,
    ) as *mut libc::c_float;
    if output.is_null() {
        stbi_free(data as *mut libc::c_void);
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    if comp & 1 as libc::c_int != 0 {
        n = comp;
    } else {
        n = comp - 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < x * y {
        k = 0 as libc::c_int;
        while k < n {
            *output
                .offset(
                    (i * comp + k) as isize,
                ) = (pow(
                (*data.offset((i * comp + k) as isize) as libc::c_int as libc::c_float
                    / 255.0f32) as libc::c_double,
                stbi__l2h_gamma as libc::c_double,
            ) * stbi__l2h_scale as libc::c_double) as libc::c_float;
            k += 1;
            k;
        }
        i += 1;
        i;
    }
    if n < comp {
        i = 0 as libc::c_int;
        while i < x * y {
            *output
                .offset(
                    (i * comp + n) as isize,
                ) = *data.offset((i * comp + n) as isize) as libc::c_int as libc::c_float
                / 255.0f32;
            i += 1;
            i;
        }
    }
    stbi_free(data as *mut libc::c_void);
    return output;
}
unsafe extern "C" fn stbi__loadf_main(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut libc::c_float {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if stbi__hdr_test(s) != 0 {
        let mut ri: stbi__result_info = stbi__result_info {
            bits_per_channel: 0,
            num_channels: 0,
            channel_order: 0,
        };
        let mut hdr_data: *mut libc::c_float = stbi__hdr_load(
            s,
            x,
            y,
            comp,
            req_comp,
            &mut ri,
        );
        if !hdr_data.is_null() {
            stbi__float_postprocess(hdr_data, x, y, comp, req_comp);
        }
        return hdr_data;
    }
    data = stbi__load_and_postprocess_8bit(s, x, y, comp, req_comp);
    if !data.is_null() {
        return stbi__ldr_to_hdr(
            data,
            *x,
            *y,
            if req_comp != 0 { req_comp } else { *comp },
        );
    }
    return (if stbi__err(
        b"Image not of any known type, or corrupt\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        0 as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as size_t as *mut libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_loadf_from_memory(
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut libc::c_float {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_mem(&mut s, buffer, len);
    return stbi__loadf_main(&mut s, x, y, comp, req_comp);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_load_16_from_callbacks(
    mut clbk: *const stbi_io_callbacks,
    mut user: *mut libc::c_void,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut channels_in_file: *mut libc::c_int,
    mut desired_channels: libc::c_int,
) -> *mut stbi_us {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_callbacks(&mut s, clbk as *mut stbi_io_callbacks, user);
    return stbi__load_and_postprocess_16bit(
        &mut s,
        x,
        y,
        channels_in_file,
        desired_channels,
    );
}
unsafe extern "C" fn stbi__convert_8_to_16(
    mut orig: *mut stbi_uc,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut channels: libc::c_int,
) -> *mut stbi__uint16 {
    let mut i: libc::c_int = 0;
    let mut img_len: libc::c_int = w * h * channels;
    let mut enlarged: *mut stbi__uint16 = 0 as *mut stbi__uint16;
    enlarged = stbi__malloc((img_len * 2 as libc::c_int) as size_t) as *mut stbi__uint16;
    if enlarged.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut stbi__uint16;
    }
    i = 0 as libc::c_int;
    while i < img_len {
        *enlarged
            .offset(
                i as isize,
            ) = (((*orig.offset(i as isize) as libc::c_int) << 8 as libc::c_int)
            + *orig.offset(i as isize) as libc::c_int) as stbi__uint16;
        i += 1;
        i;
    }
    stbi_free(orig as *mut libc::c_void);
    return enlarged;
}
unsafe extern "C" fn stbi__load_and_postprocess_16bit(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut stbi__uint16 {
    let mut ri: stbi__result_info = stbi__result_info {
        bits_per_channel: 0,
        num_channels: 0,
        channel_order: 0,
    };
    let mut result: *mut libc::c_void = stbi__load_main(
        s,
        x,
        y,
        comp,
        req_comp,
        &mut ri,
        16 as libc::c_int,
    );
    if result.is_null() {
        return 0 as *mut stbi__uint16;
    }
    if ri.bits_per_channel == 8 as libc::c_int
        || ri.bits_per_channel == 16 as libc::c_int
    {} else {
        __assert_fail(
            b"ri.bits_per_channel == 8 || ri.bits_per_channel == 16\0" as *const u8
                as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            1275 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"stbi__uint16 *stbi__load_and_postprocess_16bit(stbi__context *, int *, int *, int *, int)\0",
            ))
                .as_ptr(),
        );
    };
    if ri.bits_per_channel != 16 as libc::c_int {
        result = stbi__convert_8_to_16(
            result as *mut stbi_uc,
            *x,
            *y,
            if req_comp == 0 as libc::c_int { *comp } else { req_comp },
        ) as *mut libc::c_void;
        ri.bits_per_channel = 16 as libc::c_int;
    }
    if if stbi__vertically_flip_on_load_set != 0 {
        stbi__vertically_flip_on_load_local
    } else {
        stbi__vertically_flip_on_load_global
    } != 0
    {
        let mut channels: libc::c_int = if req_comp != 0 { req_comp } else { *comp };
        stbi__vertical_flip(
            result,
            *x,
            *y,
            (channels as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<stbi__uint16>() as libc::c_ulong)
                as libc::c_int,
        );
    }
    return result as *mut stbi__uint16;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_load_16_from_memory(
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut channels_in_file: *mut libc::c_int,
    mut desired_channels: libc::c_int,
) -> *mut stbi_us {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_mem(&mut s, buffer, len);
    return stbi__load_and_postprocess_16bit(
        &mut s,
        x,
        y,
        channels_in_file,
        desired_channels,
    );
}
unsafe extern "C" fn stbi__start_callbacks(
    mut s: *mut stbi__context,
    mut c: *mut stbi_io_callbacks,
    mut user: *mut libc::c_void,
) {
    (*s).io = *c;
    (*s).io_user_data = user;
    (*s)
        .buflen = ::core::mem::size_of::<[stbi_uc; 128]>() as libc::c_ulong
        as libc::c_int;
    (*s).read_from_callbacks = 1 as libc::c_int;
    (*s).callback_already_read = 0 as libc::c_int;
    (*s).img_buffer_original = ((*s).buffer_start).as_mut_ptr();
    (*s).img_buffer = (*s).img_buffer_original;
    stbi__refill_buffer(s);
    (*s).img_buffer_original_end = (*s).img_buffer_end;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_load_from_callbacks(
    mut clbk: *const stbi_io_callbacks,
    mut user: *mut libc::c_void,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut stbi_uc {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_callbacks(&mut s, clbk as *mut stbi_io_callbacks, user);
    return stbi__load_and_postprocess_8bit(&mut s, x, y, comp, req_comp);
}
#[no_mangle]
pub unsafe extern "C" fn stbi_load_from_memory(
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut stbi_uc {
    let mut s: stbi__context = stbi__context {
        img_x: 0,
        img_y: 0,
        img_n: 0,
        img_out_n: 0,
        io: stbi_io_callbacks {
            read: None,
            skip: None,
            eof: None,
        },
        io_user_data: 0 as *mut libc::c_void,
        read_from_callbacks: 0,
        buflen: 0,
        buffer_start: [0; 128],
        callback_already_read: 0,
        img_buffer: 0 as *mut stbi_uc,
        img_buffer_end: 0 as *mut stbi_uc,
        img_buffer_original: 0 as *mut stbi_uc,
        img_buffer_original_end: 0 as *mut stbi_uc,
    };
    stbi__start_mem(&mut s, buffer, len);
    return stbi__load_and_postprocess_8bit(&mut s, x, y, comp, req_comp);
}
unsafe extern "C" fn stbi__start_mem(
    mut s: *mut stbi__context,
    mut buffer: *const stbi_uc,
    mut len: libc::c_int,
) {
    (*s).io.read = None;
    (*s).read_from_callbacks = 0 as libc::c_int;
    (*s).callback_already_read = 0 as libc::c_int;
    (*s).img_buffer_original = buffer as *mut stbi_uc;
    (*s).img_buffer = (*s).img_buffer_original;
    (*s).img_buffer_original_end = (buffer as *mut stbi_uc).offset(len as isize);
    (*s).img_buffer_end = (*s).img_buffer_original_end;
}
unsafe extern "C" fn stbi__convert_16_to_8(
    mut orig: *mut stbi__uint16,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut channels: libc::c_int,
) -> *mut stbi_uc {
    let mut i: libc::c_int = 0;
    let mut img_len: libc::c_int = w * h * channels;
    let mut reduced: *mut stbi_uc = 0 as *mut stbi_uc;
    reduced = stbi__malloc(img_len as size_t) as *mut stbi_uc;
    if reduced.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar;
    }
    i = 0 as libc::c_int;
    while i < img_len {
        *reduced
            .offset(
                i as isize,
            ) = (*orig.offset(i as isize) as libc::c_int >> 8 as libc::c_int
            & 0xff as libc::c_int) as stbi_uc;
        i += 1;
        i;
    }
    stbi_free(orig as *mut libc::c_void);
    return reduced;
}
#[thread_local]
static mut stbi__vertically_flip_on_load_set: libc::c_int = 0;
#[thread_local]
static mut stbi__vertically_flip_on_load_local: libc::c_int = 0;
static mut stbi__vertically_flip_on_load_global: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn stbi__vertical_flip(
    mut image: *mut libc::c_void,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut bytes_per_pixel: libc::c_int,
) {
    let mut row: libc::c_int = 0;
    let mut bytes_per_row: size_t = (w as size_t)
        .wrapping_mul(bytes_per_pixel as libc::c_ulong);
    let mut temp: [stbi_uc; 2048] = [0; 2048];
    let mut bytes: *mut stbi_uc = image as *mut stbi_uc;
    row = 0 as libc::c_int;
    while row < h >> 1 as libc::c_int {
        let mut row0: *mut stbi_uc = bytes
            .offset((row as libc::c_ulong).wrapping_mul(bytes_per_row) as isize);
        let mut row1: *mut stbi_uc = bytes
            .offset(
                ((h - row - 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(bytes_per_row) as isize,
            );
        let mut bytes_left: size_t = bytes_per_row;
        while bytes_left != 0 {
            let mut bytes_copy: size_t = if bytes_left
                < ::core::mem::size_of::<[stbi_uc; 2048]>() as libc::c_ulong
            {
                bytes_left
            } else {
                ::core::mem::size_of::<[stbi_uc; 2048]>() as libc::c_ulong
            };
            memcpy(
                temp.as_mut_ptr() as *mut libc::c_void,
                row0 as *const libc::c_void,
                bytes_copy,
            );
            memcpy(row0 as *mut libc::c_void, row1 as *const libc::c_void, bytes_copy);
            memcpy(
                row1 as *mut libc::c_void,
                temp.as_mut_ptr() as *const libc::c_void,
                bytes_copy,
            );
            row0 = row0.offset(bytes_copy as isize);
            row1 = row1.offset(bytes_copy as isize);
            bytes_left = (bytes_left as libc::c_ulong).wrapping_sub(bytes_copy) as size_t
                as size_t;
        }
        row += 1;
        row;
    }
}
unsafe extern "C" fn stbi__png_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = stbi__check_png_header(s);
    stbi__rewind(s);
    return r;
}
unsafe extern "C" fn stbi__check_png_header(mut s: *mut stbi__context) -> libc::c_int {
    static mut png_sig: [stbi_uc; 8] = [
        137 as libc::c_int as stbi_uc,
        80 as libc::c_int as stbi_uc,
        78 as libc::c_int as stbi_uc,
        71 as libc::c_int as stbi_uc,
        13 as libc::c_int as stbi_uc,
        10 as libc::c_int as stbi_uc,
        26 as libc::c_int as stbi_uc,
        10 as libc::c_int as stbi_uc,
    ];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if stbi__get8(s) as libc::c_int != png_sig[i as usize] as libc::c_int {
            return stbi__err(b"Not a PNG\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__parse_zlib_header(mut a: *mut stbi__zbuf) -> libc::c_int {
    let mut cmf: libc::c_int = stbi__zget8(a) as libc::c_int;
    let mut cm: libc::c_int = cmf & 15 as libc::c_int;
    let mut flg: libc::c_int = stbi__zget8(a) as libc::c_int;
    if stbi__zeof(a) != 0 {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    if (cmf * 256 as libc::c_int + flg) % 31 as libc::c_int != 0 as libc::c_int {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    if flg & 32 as libc::c_int != 0 {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    if cm != 8 as libc::c_int {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__parse_uncompressed_block(
    mut a: *mut stbi__zbuf,
) -> libc::c_int {
    let mut header: [stbi_uc; 4] = [0; 4];
    let mut len: libc::c_int = 0;
    let mut nlen: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*a).num_bits & 7 as libc::c_int != 0 {
        stbi__zreceive(a, (*a).num_bits & 7 as libc::c_int);
    }
    k = 0 as libc::c_int;
    while (*a).num_bits > 0 as libc::c_int {
        let fresh1 = k;
        k = k + 1;
        header[fresh1
            as usize] = ((*a).code_buffer & 255 as libc::c_int as libc::c_uint)
            as stbi_uc;
        (*a).code_buffer >>= 8 as libc::c_int;
        (*a).num_bits -= 8 as libc::c_int;
    }
    if (*a).num_bits < 0 as libc::c_int {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    while k < 4 as libc::c_int {
        let fresh2 = k;
        k = k + 1;
        header[fresh2 as usize] = stbi__zget8(a);
    }
    len = header[1 as libc::c_int as usize] as libc::c_int * 256 as libc::c_int
        + header[0 as libc::c_int as usize] as libc::c_int;
    nlen = header[3 as libc::c_int as usize] as libc::c_int * 256 as libc::c_int
        + header[2 as libc::c_int as usize] as libc::c_int;
    if nlen != len ^ 0xffff as libc::c_int {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    if ((*a).zbuffer).offset(len as isize) > (*a).zbuffer_end {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    if ((*a).zout).offset(len as isize) > (*a).zout_end {
        if stbi__zexpand(a, (*a).zout, len) == 0 {
            return 0 as libc::c_int;
        }
    }
    memcpy(
        (*a).zout as *mut libc::c_void,
        (*a).zbuffer as *const libc::c_void,
        len as libc::c_ulong,
    );
    (*a).zbuffer = ((*a).zbuffer).offset(len as isize);
    (*a).zout = ((*a).zout).offset(len as isize);
    return 1 as libc::c_int;
}
static mut stbi__zdefault_length: [stbi_uc; 288] = [
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
];
static mut stbi__zdefault_distance: [stbi_uc; 32] = [
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
];
unsafe extern "C" fn stbi__zbuild_huffman(
    mut z: *mut stbi__zhuffman,
    mut sizelist: *const stbi_uc,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut code: libc::c_int = 0;
    let mut next_code: [libc::c_int; 16] = [0; 16];
    let mut sizes: [libc::c_int; 17] = [0; 17];
    memset(
        sizes.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 17]>() as libc::c_ulong,
    );
    memset(
        ((*z).fast).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[stbi__uint16; 512]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < num {
        sizes[*sizelist.offset(i as isize) as usize] += 1;
        sizes[*sizelist.offset(i as isize) as usize];
        i += 1;
        i;
    }
    sizes[0 as libc::c_int as usize] = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 16 as libc::c_int {
        if sizes[i as usize] > (1 as libc::c_int) << i {
            return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    code = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 16 as libc::c_int {
        next_code[i as usize] = code;
        (*z).firstcode[i as usize] = code as stbi__uint16;
        (*z).firstsymbol[i as usize] = k as stbi__uint16;
        code = code + sizes[i as usize];
        if sizes[i as usize] != 0 {
            if code - 1 as libc::c_int >= (1 as libc::c_int) << i {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
            }
        }
        (*z).maxcode[i as usize] = code << 16 as libc::c_int - i;
        code <<= 1 as libc::c_int;
        k += sizes[i as usize];
        i += 1;
        i;
    }
    (*z).maxcode[16 as libc::c_int as usize] = 0x10000 as libc::c_int;
    i = 0 as libc::c_int;
    while i < num {
        let mut s: libc::c_int = *sizelist.offset(i as isize) as libc::c_int;
        if s != 0 {
            let mut c: libc::c_int = next_code[s as usize]
                - (*z).firstcode[s as usize] as libc::c_int
                + (*z).firstsymbol[s as usize] as libc::c_int;
            let mut fastv: stbi__uint16 = (s << 9 as libc::c_int | i) as stbi__uint16;
            (*z).size[c as usize] = s as stbi_uc;
            (*z).value[c as usize] = i as stbi__uint16;
            if s <= 9 as libc::c_int {
                let mut j: libc::c_int = stbi__bit_reverse(next_code[s as usize], s);
                while j < (1 as libc::c_int) << 9 as libc::c_int {
                    (*z).fast[j as usize] = fastv;
                    j += (1 as libc::c_int) << s;
                }
            }
            next_code[s as usize] += 1;
            next_code[s as usize];
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__compute_huffman_codes(mut a: *mut stbi__zbuf) -> libc::c_int {
    static mut length_dezigzag: [stbi_uc; 19] = [
        16 as libc::c_int as stbi_uc,
        17 as libc::c_int as stbi_uc,
        18 as libc::c_int as stbi_uc,
        0 as libc::c_int as stbi_uc,
        8 as libc::c_int as stbi_uc,
        7 as libc::c_int as stbi_uc,
        9 as libc::c_int as stbi_uc,
        6 as libc::c_int as stbi_uc,
        10 as libc::c_int as stbi_uc,
        5 as libc::c_int as stbi_uc,
        11 as libc::c_int as stbi_uc,
        4 as libc::c_int as stbi_uc,
        12 as libc::c_int as stbi_uc,
        3 as libc::c_int as stbi_uc,
        13 as libc::c_int as stbi_uc,
        2 as libc::c_int as stbi_uc,
        14 as libc::c_int as stbi_uc,
        1 as libc::c_int as stbi_uc,
        15 as libc::c_int as stbi_uc,
    ];
    let mut z_codelength: stbi__zhuffman = stbi__zhuffman {
        fast: [0; 512],
        firstcode: [0; 16],
        maxcode: [0; 17],
        firstsymbol: [0; 16],
        size: [0; 288],
        value: [0; 288],
    };
    let mut lencodes: [stbi_uc; 455] = [0; 455];
    let mut codelength_sizes: [stbi_uc; 19] = [0; 19];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut hlit: libc::c_int = (stbi__zreceive(a, 5 as libc::c_int))
        .wrapping_add(257 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut hdist: libc::c_int = (stbi__zreceive(a, 5 as libc::c_int))
        .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut hclen: libc::c_int = (stbi__zreceive(a, 4 as libc::c_int))
        .wrapping_add(4 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut ntot: libc::c_int = hlit + hdist;
    memset(
        codelength_sizes.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[stbi_uc; 19]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < hclen {
        let mut s: libc::c_int = stbi__zreceive(a, 3 as libc::c_int) as libc::c_int;
        codelength_sizes[length_dezigzag[i as usize] as usize] = s as stbi_uc;
        i += 1;
        i;
    }
    if stbi__zbuild_huffman(
        &mut z_codelength,
        codelength_sizes.as_mut_ptr(),
        19 as libc::c_int,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    n = 0 as libc::c_int;
    while n < ntot {
        let mut c: libc::c_int = stbi__zhuffman_decode(a, &mut z_codelength);
        if c < 0 as libc::c_int || c >= 19 as libc::c_int {
            return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
        }
        if c < 16 as libc::c_int {
            let fresh3 = n;
            n = n + 1;
            lencodes[fresh3 as usize] = c as stbi_uc;
        } else {
            let mut fill: stbi_uc = 0 as libc::c_int as stbi_uc;
            if c == 16 as libc::c_int {
                c = (stbi__zreceive(a, 2 as libc::c_int))
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as libc::c_int;
                if n == 0 as libc::c_int {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                fill = lencodes[(n - 1 as libc::c_int) as usize];
            } else if c == 17 as libc::c_int {
                c = (stbi__zreceive(a, 3 as libc::c_int))
                    .wrapping_add(3 as libc::c_int as libc::c_uint) as libc::c_int;
            } else if c == 18 as libc::c_int {
                c = (stbi__zreceive(a, 7 as libc::c_int))
                    .wrapping_add(11 as libc::c_int as libc::c_uint) as libc::c_int;
            } else {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char)
            }
            if ntot - n < c {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
            }
            memset(
                lencodes.as_mut_ptr().offset(n as isize) as *mut libc::c_void,
                fill as libc::c_int,
                c as libc::c_ulong,
            );
            n += c;
        }
    }
    if n != ntot {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    if stbi__zbuild_huffman(&mut (*a).z_length, lencodes.as_mut_ptr(), hlit) == 0 {
        return 0 as libc::c_int;
    }
    if stbi__zbuild_huffman(
        &mut (*a).z_distance,
        lencodes.as_mut_ptr().offset(hlit as isize),
        hdist,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
static mut stbi__zlength_base: [libc::c_int; 31] = [
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    13 as libc::c_int,
    15 as libc::c_int,
    17 as libc::c_int,
    19 as libc::c_int,
    23 as libc::c_int,
    27 as libc::c_int,
    31 as libc::c_int,
    35 as libc::c_int,
    43 as libc::c_int,
    51 as libc::c_int,
    59 as libc::c_int,
    67 as libc::c_int,
    83 as libc::c_int,
    99 as libc::c_int,
    115 as libc::c_int,
    131 as libc::c_int,
    163 as libc::c_int,
    195 as libc::c_int,
    227 as libc::c_int,
    258 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
static mut stbi__zlength_extra: [libc::c_int; 31] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
static mut stbi__zdist_base: [libc::c_int; 32] = [
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    7 as libc::c_int,
    9 as libc::c_int,
    13 as libc::c_int,
    17 as libc::c_int,
    25 as libc::c_int,
    33 as libc::c_int,
    49 as libc::c_int,
    65 as libc::c_int,
    97 as libc::c_int,
    129 as libc::c_int,
    193 as libc::c_int,
    257 as libc::c_int,
    385 as libc::c_int,
    513 as libc::c_int,
    769 as libc::c_int,
    1025 as libc::c_int,
    1537 as libc::c_int,
    2049 as libc::c_int,
    3073 as libc::c_int,
    4097 as libc::c_int,
    6145 as libc::c_int,
    8193 as libc::c_int,
    12289 as libc::c_int,
    16385 as libc::c_int,
    24577 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
unsafe extern "C" fn stbi__zreceive(
    mut z: *mut stbi__zbuf,
    mut n: libc::c_int,
) -> libc::c_uint {
    let mut k: libc::c_uint = 0;
    if (*z).num_bits < n {
        stbi__fill_bits(z);
    }
    k = (*z).code_buffer
        & (((1 as libc::c_int) << n) - 1 as libc::c_int) as libc::c_uint;
    (*z).code_buffer >>= n;
    (*z).num_bits -= n;
    return k;
}
static mut stbi__zdist_extra: [libc::c_int; 32] = [
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    9 as libc::c_int,
    10 as libc::c_int,
    10 as libc::c_int,
    11 as libc::c_int,
    11 as libc::c_int,
    12 as libc::c_int,
    12 as libc::c_int,
    13 as libc::c_int,
    13 as libc::c_int,
    0,
    0,
];
unsafe extern "C" fn stbi__zeof(mut z: *mut stbi__zbuf) -> libc::c_int {
    return ((*z).zbuffer >= (*z).zbuffer_end) as libc::c_int;
}
unsafe extern "C" fn stbi__zget8(mut z: *mut stbi__zbuf) -> stbi_uc {
    return (if stbi__zeof(z) != 0 {
        0 as libc::c_int
    } else {
        let fresh4 = (*z).zbuffer;
        (*z).zbuffer = ((*z).zbuffer).offset(1);
        *fresh4 as libc::c_int
    }) as stbi_uc;
}
unsafe extern "C" fn stbi__fill_bits(mut z: *mut stbi__zbuf) {
    loop {
        if (*z).code_buffer >= (1 as libc::c_uint) << (*z).num_bits {
            (*z).zbuffer = (*z).zbuffer_end;
            return;
        }
        (*z).code_buffer |= (stbi__zget8(z) as libc::c_uint) << (*z).num_bits;
        (*z).num_bits += 8 as libc::c_int;
        if !((*z).num_bits <= 24 as libc::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn stbi__bitreverse16(mut n: libc::c_int) -> libc::c_int {
    n = (n & 0xaaaa as libc::c_int) >> 1 as libc::c_int
        | (n & 0x5555 as libc::c_int) << 1 as libc::c_int;
    n = (n & 0xcccc as libc::c_int) >> 2 as libc::c_int
        | (n & 0x3333 as libc::c_int) << 2 as libc::c_int;
    n = (n & 0xf0f0 as libc::c_int) >> 4 as libc::c_int
        | (n & 0xf0f as libc::c_int) << 4 as libc::c_int;
    n = (n & 0xff00 as libc::c_int) >> 8 as libc::c_int
        | (n & 0xff as libc::c_int) << 8 as libc::c_int;
    return n;
}
unsafe extern "C" fn stbi__bit_reverse(
    mut v: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    if bits <= 16 as libc::c_int {} else {
        __assert_fail(
            b"bits <= 16\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            4061 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"int stbi__bit_reverse(int, int)\0"))
                .as_ptr(),
        );
    };
    return stbi__bitreverse16(v) >> 16 as libc::c_int - bits;
}
unsafe extern "C" fn stbi__zhuffman_decode_slowpath(
    mut a: *mut stbi__zbuf,
    mut z: *mut stbi__zhuffman,
) -> libc::c_int {
    let mut b: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = stbi__bit_reverse((*a).code_buffer as libc::c_int, 16 as libc::c_int);
    s = 9 as libc::c_int + 1 as libc::c_int;
    while !(k < (*z).maxcode[s as usize]) {
        s += 1;
        s;
    }
    if s >= 16 as libc::c_int {
        return -(1 as libc::c_int);
    }
    b = (k >> 16 as libc::c_int - s) - (*z).firstcode[s as usize] as libc::c_int
        + (*z).firstsymbol[s as usize] as libc::c_int;
    if b >= 288 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*z).size[b as usize] as libc::c_int != s {
        return -(1 as libc::c_int);
    }
    (*a).code_buffer >>= s;
    (*a).num_bits -= s;
    return (*z).value[b as usize] as libc::c_int;
}
unsafe extern "C" fn stbi__zhuffman_decode(
    mut a: *mut stbi__zbuf,
    mut z: *mut stbi__zhuffman,
) -> libc::c_int {
    let mut b: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    if (*a).num_bits < 16 as libc::c_int {
        if stbi__zeof(a) != 0 {
            return -(1 as libc::c_int);
        }
        stbi__fill_bits(a);
    }
    b = (*z)
        .fast[((*a).code_buffer
        & (((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        as usize] as libc::c_int;
    if b != 0 {
        s = b >> 9 as libc::c_int;
        (*a).code_buffer >>= s;
        (*a).num_bits -= s;
        return b & 511 as libc::c_int;
    }
    return stbi__zhuffman_decode_slowpath(a, z);
}
unsafe extern "C" fn stbi__zexpand(
    mut z: *mut stbi__zbuf,
    mut zout: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur: libc::c_uint = 0;
    let mut limit: libc::c_uint = 0;
    let mut old_limit: libc::c_uint = 0;
    (*z).zout = zout;
    if (*z).z_expandable == 0 {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    cur = ((*z).zout).offset_from((*z).zout_start) as libc::c_long as libc::c_uint;
    old_limit = ((*z).zout_end).offset_from((*z).zout_start) as libc::c_long
        as libc::c_uint;
    limit = old_limit;
    if (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint)
        .wrapping_sub(cur) < n as libc::c_uint
    {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    while cur.wrapping_add(n as libc::c_uint) > limit {
        if limit
            > (2147483647 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_uint)
                .wrapping_add(1 as libc::c_uint)
                .wrapping_div(2 as libc::c_int as libc::c_uint)
        {
            return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
        }
        limit = limit.wrapping_mul(2 as libc::c_int as libc::c_uint);
    }
    q = stbi_realloc((*z).zout_start as *mut libc::c_void, limit as size_t)
        as *mut libc::c_char;
    if q.is_null() {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    (*z).zout_start = q;
    (*z).zout = q.offset(cur as isize);
    (*z).zout_end = q.offset(limit as isize);
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__parse_huffman_block(mut a: *mut stbi__zbuf) -> libc::c_int {
    let mut zout: *mut libc::c_char = (*a).zout;
    loop {
        let mut z: libc::c_int = stbi__zhuffman_decode(a, &mut (*a).z_length);
        if z < 256 as libc::c_int {
            if z < 0 as libc::c_int {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
            }
            if zout >= (*a).zout_end {
                if stbi__zexpand(a, zout, 1 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                zout = (*a).zout;
            }
            let fresh5 = zout;
            zout = zout.offset(1);
            *fresh5 = z as libc::c_char;
        } else {
            let mut p: *mut stbi_uc = 0 as *mut stbi_uc;
            let mut len: libc::c_int = 0;
            let mut dist: libc::c_int = 0;
            if z == 256 as libc::c_int {
                (*a).zout = zout;
                return 1 as libc::c_int;
            }
            z -= 257 as libc::c_int;
            len = stbi__zlength_base[z as usize];
            if stbi__zlength_extra[z as usize] != 0 {
                len = (len as libc::c_uint)
                    .wrapping_add(stbi__zreceive(a, stbi__zlength_extra[z as usize]))
                    as libc::c_int as libc::c_int;
            }
            z = stbi__zhuffman_decode(a, &mut (*a).z_distance);
            if z < 0 as libc::c_int {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
            }
            dist = stbi__zdist_base[z as usize];
            if stbi__zdist_extra[z as usize] != 0 {
                dist = (dist as libc::c_uint)
                    .wrapping_add(stbi__zreceive(a, stbi__zdist_extra[z as usize]))
                    as libc::c_int as libc::c_int;
            }
            if (zout.offset_from((*a).zout_start) as libc::c_long) < dist as libc::c_long
            {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
            }
            if zout.offset(len as isize) > (*a).zout_end {
                if stbi__zexpand(a, zout, len) == 0 {
                    return 0 as libc::c_int;
                }
                zout = (*a).zout;
            }
            p = zout.offset(-(dist as isize)) as *mut stbi_uc;
            if dist == 1 as libc::c_int {
                let mut v: stbi_uc = *p;
                if len != 0 {
                    loop {
                        let fresh6 = zout;
                        zout = zout.offset(1);
                        *fresh6 = v as libc::c_char;
                        len -= 1;
                        if !(len != 0) {
                            break;
                        }
                    }
                }
            } else if len != 0 {
                loop {
                    let fresh7 = p;
                    p = p.offset(1);
                    let fresh8 = zout;
                    zout = zout.offset(1);
                    *fresh8 = *fresh7 as libc::c_char;
                    len -= 1;
                    if !(len != 0) {
                        break;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn stbi__parse_zlib(
    mut a: *mut stbi__zbuf,
    mut parse_header: libc::c_int,
) -> libc::c_int {
    let mut final_0: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    if parse_header != 0 {
        if stbi__parse_zlib_header(a) == 0 {
            return 0 as libc::c_int;
        }
    }
    (*a).num_bits = 0 as libc::c_int;
    (*a).code_buffer = 0 as libc::c_int as stbi__uint32;
    loop {
        final_0 = stbi__zreceive(a, 1 as libc::c_int) as libc::c_int;
        type_0 = stbi__zreceive(a, 2 as libc::c_int) as libc::c_int;
        if type_0 == 0 as libc::c_int {
            if stbi__parse_uncompressed_block(a) == 0 {
                return 0 as libc::c_int;
            }
        } else if type_0 == 3 as libc::c_int {
            return 0 as libc::c_int
        } else {
            if type_0 == 1 as libc::c_int {
                if stbi__zbuild_huffman(
                    &mut (*a).z_length,
                    stbi__zdefault_length.as_ptr(),
                    288 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if stbi__zbuild_huffman(
                    &mut (*a).z_distance,
                    stbi__zdefault_distance.as_ptr(),
                    32 as libc::c_int,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if stbi__compute_huffman_codes(a) == 0 {
                return 0 as libc::c_int
            }
            if stbi__parse_huffman_block(a) == 0 {
                return 0 as libc::c_int;
            }
        }
        if !(final_0 == 0) {
            break;
        }
    }
    return 1 as libc::c_int;
}
static mut first_row_filter: [stbi_uc; 5] = [
    STBI__F_none as libc::c_int as stbi_uc,
    STBI__F_sub as libc::c_int as stbi_uc,
    STBI__F_none as libc::c_int as stbi_uc,
    STBI__F_avg_first as libc::c_int as stbi_uc,
    STBI__F_paeth_first as libc::c_int as stbi_uc,
];
#[no_mangle]
pub unsafe extern "C" fn stbi_zlib_decode_malloc_guesssize_headerflag(
    mut buffer: *const libc::c_char,
    mut len: libc::c_int,
    mut initial_size: libc::c_int,
    mut outlen: *mut libc::c_int,
    mut parse_header: libc::c_int,
) -> *mut libc::c_char {
    let mut a: stbi__zbuf = stbi__zbuf {
        zbuffer: 0 as *mut stbi_uc,
        zbuffer_end: 0 as *mut stbi_uc,
        num_bits: 0,
        code_buffer: 0,
        zout: 0 as *mut libc::c_char,
        zout_start: 0 as *mut libc::c_char,
        zout_end: 0 as *mut libc::c_char,
        z_expandable: 0,
        z_length: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
        z_distance: stbi__zhuffman {
            fast: [0; 512],
            firstcode: [0; 16],
            maxcode: [0; 17],
            firstsymbol: [0; 16],
            size: [0; 288],
            value: [0; 288],
        },
    };
    let mut p: *mut libc::c_char = stbi__malloc(initial_size as size_t)
        as *mut libc::c_char;
    if p.is_null() {
        return 0 as *mut libc::c_char;
    }
    a.zbuffer = buffer as *mut stbi_uc;
    a.zbuffer_end = (buffer as *mut stbi_uc).offset(len as isize);
    if stbi__do_zlib(&mut a, p, initial_size, 1 as libc::c_int, parse_header) != 0 {
        if !outlen.is_null() {
            *outlen = (a.zout).offset_from(a.zout_start) as libc::c_long as libc::c_int;
        }
        return a.zout_start;
    } else {
        stbi_free(a.zout_start as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    };
}
unsafe extern "C" fn stbi__paeth(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut p: libc::c_int = a + b - c;
    let mut pa: libc::c_int = abs(p - a);
    let mut pb: libc::c_int = abs(p - b);
    let mut pc: libc::c_int = abs(p - c);
    if pa <= pb && pa <= pc {
        return a;
    }
    if pb <= pc {
        return b;
    }
    return c;
}
static mut stbi__depth_scale_table: [stbi_uc; 9] = [
    0 as libc::c_int as stbi_uc,
    0xff as libc::c_int as stbi_uc,
    0x55 as libc::c_int as stbi_uc,
    0 as libc::c_int as stbi_uc,
    0x11 as libc::c_int as stbi_uc,
    0 as libc::c_int as stbi_uc,
    0 as libc::c_int as stbi_uc,
    0 as libc::c_int as stbi_uc,
    0x1 as libc::c_int as stbi_uc,
];
unsafe extern "C" fn stbi__create_png_image_raw(
    mut a: *mut stbi__png,
    mut raw: *mut stbi_uc,
    mut raw_len: stbi__uint32,
    mut out_n: libc::c_int,
    mut x: stbi__uint32,
    mut y: stbi__uint32,
    mut depth: libc::c_int,
    mut color: libc::c_int,
) -> libc::c_int {
    let mut bytes: libc::c_int = if depth == 16 as libc::c_int {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut s: *mut stbi__context = (*a).s;
    let mut i: stbi__uint32 = 0;
    let mut j: stbi__uint32 = 0;
    let mut stride: stbi__uint32 = x
        .wrapping_mul(out_n as libc::c_uint)
        .wrapping_mul(bytes as libc::c_uint);
    let mut img_len: stbi__uint32 = 0;
    let mut img_width_bytes: stbi__uint32 = 0;
    let mut k: libc::c_int = 0;
    let mut img_n: libc::c_int = (*s).img_n;
    let mut output_bytes: libc::c_int = out_n * bytes;
    let mut filter_bytes: libc::c_int = img_n * bytes;
    let mut width: libc::c_int = x as libc::c_int;
    if out_n == (*s).img_n || out_n == (*s).img_n + 1 as libc::c_int {} else {
        __assert_fail(
            b"out_n == s->img_n || out_n == s->img_n+1\0" as *const u8
                as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            4607 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 112],
                &[libc::c_char; 112],
            >(
                b"int stbi__create_png_image_raw(stbi__png *, stbi_uc *, stbi__uint32, int, stbi__uint32, stbi__uint32, int, int)\0",
            ))
                .as_ptr(),
        );
    };
    (*a)
        .out = stbi__malloc_mad3(
        x as libc::c_int,
        y as libc::c_int,
        output_bytes,
        0 as libc::c_int,
    ) as *mut stbi_uc;
    if ((*a).out).is_null() {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    if stbi__mad3sizes_valid(img_n, x as libc::c_int, depth, 7 as libc::c_int) == 0 {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    img_width_bytes = (img_n as libc::c_uint)
        .wrapping_mul(x)
        .wrapping_mul(depth as libc::c_uint)
        .wrapping_add(7 as libc::c_int as libc::c_uint) >> 3 as libc::c_int;
    img_len = img_width_bytes
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_mul(y);
    if raw_len < img_len {
        return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
    }
    j = 0 as libc::c_int as stbi__uint32;
    while j < y {
        let mut cur: *mut stbi_uc = ((*a).out).offset(stride.wrapping_mul(j) as isize);
        let mut prior: *mut stbi_uc = 0 as *mut stbi_uc;
        let fresh9 = raw;
        raw = raw.offset(1);
        let mut filter: libc::c_int = *fresh9 as libc::c_int;
        if filter > 4 as libc::c_int {
            return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
        }
        if depth < 8 as libc::c_int {
            if img_width_bytes > x {
                return stbi__err(b"Corrupt PNG\0" as *const u8 as *const libc::c_char);
            }
            cur = cur
                .offset(
                    x.wrapping_mul(out_n as libc::c_uint).wrapping_sub(img_width_bytes)
                        as isize,
                );
            filter_bytes = 1 as libc::c_int;
            width = img_width_bytes as libc::c_int;
        }
        prior = cur.offset(-(stride as isize));
        if j == 0 as libc::c_int as libc::c_uint {
            filter = first_row_filter[filter as usize] as libc::c_int;
        }
        k = 0 as libc::c_int;
        while k < filter_bytes {
            match filter {
                0 => {
                    *cur.offset(k as isize) = *raw.offset(k as isize);
                }
                1 => {
                    *cur.offset(k as isize) = *raw.offset(k as isize);
                }
                2 => {
                    *cur
                        .offset(
                            k as isize,
                        ) = (*raw.offset(k as isize) as libc::c_int
                        + *prior.offset(k as isize) as libc::c_int & 255 as libc::c_int)
                        as stbi_uc;
                }
                3 => {
                    *cur
                        .offset(
                            k as isize,
                        ) = (*raw.offset(k as isize) as libc::c_int
                        + (*prior.offset(k as isize) as libc::c_int >> 1 as libc::c_int)
                        & 255 as libc::c_int) as stbi_uc;
                }
                4 => {
                    *cur
                        .offset(
                            k as isize,
                        ) = (*raw.offset(k as isize) as libc::c_int
                        + stbi__paeth(
                            0 as libc::c_int,
                            *prior.offset(k as isize) as libc::c_int,
                            0 as libc::c_int,
                        ) & 255 as libc::c_int) as stbi_uc;
                }
                5 => {
                    *cur.offset(k as isize) = *raw.offset(k as isize);
                }
                6 => {
                    *cur.offset(k as isize) = *raw.offset(k as isize);
                }
                _ => {}
            }
            k += 1;
            k;
        }
        if depth == 8 as libc::c_int {
            if img_n != out_n {
                *cur.offset(img_n as isize) = 255 as libc::c_int as stbi_uc;
            }
            raw = raw.offset(img_n as isize);
            cur = cur.offset(out_n as isize);
            prior = prior.offset(out_n as isize);
        } else if depth == 16 as libc::c_int {
            if img_n != out_n {
                *cur.offset(filter_bytes as isize) = 255 as libc::c_int as stbi_uc;
                *cur
                    .offset(
                        (filter_bytes + 1 as libc::c_int) as isize,
                    ) = 255 as libc::c_int as stbi_uc;
            }
            raw = raw.offset(filter_bytes as isize);
            cur = cur.offset(output_bytes as isize);
            prior = prior.offset(output_bytes as isize);
        } else {
            raw = raw.offset(1 as libc::c_int as isize);
            cur = cur.offset(1 as libc::c_int as isize);
            prior = prior.offset(1 as libc::c_int as isize);
        }
        if depth < 8 as libc::c_int || img_n == out_n {
            let mut nk: libc::c_int = (width - 1 as libc::c_int) * filter_bytes;
            match filter {
                0 => {
                    memcpy(
                        cur as *mut libc::c_void,
                        raw as *const libc::c_void,
                        nk as libc::c_ulong,
                    );
                }
                1 => {
                    k = 0 as libc::c_int;
                    while k < nk {
                        *cur
                            .offset(
                                k as isize,
                            ) = (*raw.offset(k as isize) as libc::c_int
                            + *cur.offset((k - filter_bytes) as isize) as libc::c_int
                            & 255 as libc::c_int) as stbi_uc;
                        k += 1;
                        k;
                    }
                }
                2 => {
                    k = 0 as libc::c_int;
                    while k < nk {
                        *cur
                            .offset(
                                k as isize,
                            ) = (*raw.offset(k as isize) as libc::c_int
                            + *prior.offset(k as isize) as libc::c_int
                            & 255 as libc::c_int) as stbi_uc;
                        k += 1;
                        k;
                    }
                }
                3 => {
                    k = 0 as libc::c_int;
                    while k < nk {
                        *cur
                            .offset(
                                k as isize,
                            ) = (*raw.offset(k as isize) as libc::c_int
                            + (*prior.offset(k as isize) as libc::c_int
                                + *cur.offset((k - filter_bytes) as isize) as libc::c_int
                                >> 1 as libc::c_int) & 255 as libc::c_int) as stbi_uc;
                        k += 1;
                        k;
                    }
                }
                4 => {
                    k = 0 as libc::c_int;
                    while k < nk {
                        *cur
                            .offset(
                                k as isize,
                            ) = (*raw.offset(k as isize) as libc::c_int
                            + stbi__paeth(
                                *cur.offset((k - filter_bytes) as isize) as libc::c_int,
                                *prior.offset(k as isize) as libc::c_int,
                                *prior.offset((k - filter_bytes) as isize) as libc::c_int,
                            ) & 255 as libc::c_int) as stbi_uc;
                        k += 1;
                        k;
                    }
                }
                5 => {
                    k = 0 as libc::c_int;
                    while k < nk {
                        *cur
                            .offset(
                                k as isize,
                            ) = (*raw.offset(k as isize) as libc::c_int
                            + (*cur.offset((k - filter_bytes) as isize) as libc::c_int
                                >> 1 as libc::c_int) & 255 as libc::c_int) as stbi_uc;
                        k += 1;
                        k;
                    }
                }
                6 => {
                    k = 0 as libc::c_int;
                    while k < nk {
                        *cur
                            .offset(
                                k as isize,
                            ) = (*raw.offset(k as isize) as libc::c_int
                            + stbi__paeth(
                                *cur.offset((k - filter_bytes) as isize) as libc::c_int,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            ) & 255 as libc::c_int) as stbi_uc;
                        k += 1;
                        k;
                    }
                }
                _ => {}
            }
            raw = raw.offset(nk as isize);
        } else {
            if img_n + 1 as libc::c_int == out_n {} else {
                __assert_fail(
                    b"img_n+1 == out_n\0" as *const u8 as *const libc::c_char,
                    b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                    4691 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 112],
                        &[libc::c_char; 112],
                    >(
                        b"int stbi__create_png_image_raw(stbi__png *, stbi_uc *, stbi__uint32, int, stbi__uint32, stbi__uint32, int, int)\0",
                    ))
                        .as_ptr(),
                );
            };
            match filter {
                0 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur.offset(k as isize) = *raw.offset(k as isize);
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                1 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur
                                .offset(
                                    k as isize,
                                ) = (*raw.offset(k as isize) as libc::c_int
                                + *cur.offset((k - output_bytes) as isize) as libc::c_int
                                & 255 as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                2 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur
                                .offset(
                                    k as isize,
                                ) = (*raw.offset(k as isize) as libc::c_int
                                + *prior.offset(k as isize) as libc::c_int
                                & 255 as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                3 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur
                                .offset(
                                    k as isize,
                                ) = (*raw.offset(k as isize) as libc::c_int
                                + (*prior.offset(k as isize) as libc::c_int
                                    + *cur.offset((k - output_bytes) as isize) as libc::c_int
                                    >> 1 as libc::c_int) & 255 as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                4 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur
                                .offset(
                                    k as isize,
                                ) = (*raw.offset(k as isize) as libc::c_int
                                + stbi__paeth(
                                    *cur.offset((k - output_bytes) as isize) as libc::c_int,
                                    *prior.offset(k as isize) as libc::c_int,
                                    *prior.offset((k - output_bytes) as isize) as libc::c_int,
                                ) & 255 as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                5 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur
                                .offset(
                                    k as isize,
                                ) = (*raw.offset(k as isize) as libc::c_int
                                + (*cur.offset((k - output_bytes) as isize) as libc::c_int
                                    >> 1 as libc::c_int) & 255 as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                6 => {
                    i = x.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    while i >= 1 as libc::c_int as libc::c_uint {
                        k = 0 as libc::c_int;
                        while k < filter_bytes {
                            *cur
                                .offset(
                                    k as isize,
                                ) = (*raw.offset(k as isize) as libc::c_int
                                + stbi__paeth(
                                    *cur.offset((k - output_bytes) as isize) as libc::c_int,
                                    0 as libc::c_int,
                                    0 as libc::c_int,
                                ) & 255 as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                        i = i.wrapping_sub(1);
                        i;
                        *cur
                            .offset(
                                filter_bytes as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        raw = raw.offset(filter_bytes as isize);
                        cur = cur.offset(output_bytes as isize);
                        prior = prior.offset(output_bytes as isize);
                    }
                }
                _ => {}
            }
            if depth == 16 as libc::c_int {
                cur = ((*a).out).offset(stride.wrapping_mul(j) as isize);
                i = 0 as libc::c_int as stbi__uint32;
                while i < x {
                    *cur
                        .offset(
                            (filter_bytes + 1 as libc::c_int) as isize,
                        ) = 255 as libc::c_int as stbi_uc;
                    i = i.wrapping_add(1);
                    i;
                    cur = cur.offset(output_bytes as isize);
                }
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    if depth < 8 as libc::c_int {
        j = 0 as libc::c_int as stbi__uint32;
        while j < y {
            let mut cur_0: *mut stbi_uc = ((*a).out)
                .offset(stride.wrapping_mul(j) as isize);
            let mut in_0: *mut stbi_uc = ((*a).out)
                .offset(stride.wrapping_mul(j) as isize)
                .offset(x.wrapping_mul(out_n as libc::c_uint) as isize)
                .offset(-(img_width_bytes as isize));
            let mut scale: stbi_uc = (if color == 0 as libc::c_int {
                stbi__depth_scale_table[depth as usize] as libc::c_int
            } else {
                1 as libc::c_int
            }) as stbi_uc;
            if depth == 4 as libc::c_int {
                k = x.wrapping_mul(img_n as libc::c_uint) as libc::c_int;
                while k >= 2 as libc::c_int {
                    let fresh10 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh10 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 4 as libc::c_int)) as stbi_uc;
                    let fresh11 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh11 = (scale as libc::c_int
                        * (*in_0 as libc::c_int & 0xf as libc::c_int)) as stbi_uc;
                    k -= 2 as libc::c_int;
                    in_0 = in_0.offset(1);
                    in_0;
                }
                if k > 0 as libc::c_int {
                    let fresh12 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh12 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 4 as libc::c_int)) as stbi_uc;
                }
            } else if depth == 2 as libc::c_int {
                k = x.wrapping_mul(img_n as libc::c_uint) as libc::c_int;
                while k >= 4 as libc::c_int {
                    let fresh13 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh13 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 6 as libc::c_int)) as stbi_uc;
                    let fresh14 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh14 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 4 as libc::c_int
                            & 0x3 as libc::c_int)) as stbi_uc;
                    let fresh15 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh15 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 2 as libc::c_int
                            & 0x3 as libc::c_int)) as stbi_uc;
                    let fresh16 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh16 = (scale as libc::c_int
                        * (*in_0 as libc::c_int & 0x3 as libc::c_int)) as stbi_uc;
                    k -= 4 as libc::c_int;
                    in_0 = in_0.offset(1);
                    in_0;
                }
                if k > 0 as libc::c_int {
                    let fresh17 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh17 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 6 as libc::c_int)) as stbi_uc;
                }
                if k > 1 as libc::c_int {
                    let fresh18 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh18 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 4 as libc::c_int
                            & 0x3 as libc::c_int)) as stbi_uc;
                }
                if k > 2 as libc::c_int {
                    let fresh19 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh19 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 2 as libc::c_int
                            & 0x3 as libc::c_int)) as stbi_uc;
                }
            } else if depth == 1 as libc::c_int {
                k = x.wrapping_mul(img_n as libc::c_uint) as libc::c_int;
                while k >= 8 as libc::c_int {
                    let fresh20 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh20 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 7 as libc::c_int)) as stbi_uc;
                    let fresh21 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh21 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 6 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                    let fresh22 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh22 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 5 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                    let fresh23 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh23 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 4 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                    let fresh24 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh24 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 3 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                    let fresh25 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh25 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 2 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                    let fresh26 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh26 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 1 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                    let fresh27 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh27 = (scale as libc::c_int
                        * (*in_0 as libc::c_int & 0x1 as libc::c_int)) as stbi_uc;
                    k -= 8 as libc::c_int;
                    in_0 = in_0.offset(1);
                    in_0;
                }
                if k > 0 as libc::c_int {
                    let fresh28 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh28 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 7 as libc::c_int)) as stbi_uc;
                }
                if k > 1 as libc::c_int {
                    let fresh29 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh29 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 6 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                }
                if k > 2 as libc::c_int {
                    let fresh30 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh30 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 5 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                }
                if k > 3 as libc::c_int {
                    let fresh31 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh31 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 4 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                }
                if k > 4 as libc::c_int {
                    let fresh32 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh32 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 3 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                }
                if k > 5 as libc::c_int {
                    let fresh33 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh33 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 2 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                }
                if k > 6 as libc::c_int {
                    let fresh34 = cur_0;
                    cur_0 = cur_0.offset(1);
                    *fresh34 = (scale as libc::c_int
                        * (*in_0 as libc::c_int >> 1 as libc::c_int
                            & 0x1 as libc::c_int)) as stbi_uc;
                }
            }
            if img_n != out_n {
                let mut q: libc::c_int = 0;
                cur_0 = ((*a).out).offset(stride.wrapping_mul(j) as isize);
                if img_n == 1 as libc::c_int {
                    q = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    while q >= 0 as libc::c_int {
                        *cur_0
                            .offset(
                                (q * 2 as libc::c_int + 1 as libc::c_int) as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        *cur_0
                            .offset(
                                (q * 2 as libc::c_int + 0 as libc::c_int) as isize,
                            ) = *cur_0.offset(q as isize);
                        q -= 1;
                        q;
                    }
                } else {
                    if img_n == 3 as libc::c_int {} else {
                        __assert_fail(
                            b"img_n == 3\0" as *const u8 as *const libc::c_char,
                            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                            4780 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 112],
                                &[libc::c_char; 112],
                            >(
                                b"int stbi__create_png_image_raw(stbi__png *, stbi_uc *, stbi__uint32, int, stbi__uint32, stbi__uint32, int, int)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    q = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                    while q >= 0 as libc::c_int {
                        *cur_0
                            .offset(
                                (q * 4 as libc::c_int + 3 as libc::c_int) as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        *cur_0
                            .offset(
                                (q * 4 as libc::c_int + 2 as libc::c_int) as isize,
                            ) = *cur_0
                            .offset((q * 3 as libc::c_int + 2 as libc::c_int) as isize);
                        *cur_0
                            .offset(
                                (q * 4 as libc::c_int + 1 as libc::c_int) as isize,
                            ) = *cur_0
                            .offset((q * 3 as libc::c_int + 1 as libc::c_int) as isize);
                        *cur_0
                            .offset(
                                (q * 4 as libc::c_int + 0 as libc::c_int) as isize,
                            ) = *cur_0
                            .offset((q * 3 as libc::c_int + 0 as libc::c_int) as isize);
                        q -= 1;
                        q;
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
    } else if depth == 16 as libc::c_int {
        let mut cur_1: *mut stbi_uc = (*a).out;
        let mut cur16: *mut stbi__uint16 = cur_1 as *mut stbi__uint16;
        i = 0 as libc::c_int as stbi__uint32;
        while i < x.wrapping_mul(y).wrapping_mul(out_n as libc::c_uint) {
            *cur16 = ((*cur_1.offset(0 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int
                | *cur_1.offset(1 as libc::c_int as isize) as libc::c_int)
                as stbi__uint16;
            i = i.wrapping_add(1);
            i;
            cur16 = cur16.offset(1);
            cur16;
            cur_1 = cur_1.offset(2 as libc::c_int as isize);
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__create_png_image(
    mut a: *mut stbi__png,
    mut image_data: *mut stbi_uc,
    mut image_data_len: stbi__uint32,
    mut out_n: libc::c_int,
    mut depth: libc::c_int,
    mut color: libc::c_int,
    mut interlaced: libc::c_int,
) -> libc::c_int {
    let mut bytes: libc::c_int = if depth == 16 as libc::c_int {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut out_bytes: libc::c_int = out_n * bytes;
    let mut final_0: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut p: libc::c_int = 0;
    if interlaced == 0 {
        return stbi__create_png_image_raw(
            a,
            image_data,
            image_data_len,
            out_n,
            (*(*a).s).img_x,
            (*(*a).s).img_y,
            depth,
            color,
        );
    }
    final_0 = stbi__malloc_mad3(
        (*(*a).s).img_x as libc::c_int,
        (*(*a).s).img_y as libc::c_int,
        out_bytes,
        0 as libc::c_int,
    ) as *mut stbi_uc;
    if final_0.is_null() {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    p = 0 as libc::c_int;
    while p < 7 as libc::c_int {
        let mut xorig: [libc::c_int; 7] = [
            0 as libc::c_int,
            4 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        ];
        let mut yorig: [libc::c_int; 7] = [
            0 as libc::c_int,
            0 as libc::c_int,
            4 as libc::c_int,
            0 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
            1 as libc::c_int,
        ];
        let mut xspc: [libc::c_int; 7] = [
            8 as libc::c_int,
            8 as libc::c_int,
            4 as libc::c_int,
            4 as libc::c_int,
            2 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
        ];
        let mut yspc: [libc::c_int; 7] = [
            8 as libc::c_int,
            8 as libc::c_int,
            8 as libc::c_int,
            4 as libc::c_int,
            4 as libc::c_int,
            2 as libc::c_int,
            2 as libc::c_int,
        ];
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        x = ((*(*a).s).img_x)
            .wrapping_sub(xorig[p as usize] as libc::c_uint)
            .wrapping_add(xspc[p as usize] as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(xspc[p as usize] as libc::c_uint) as libc::c_int;
        y = ((*(*a).s).img_y)
            .wrapping_sub(yorig[p as usize] as libc::c_uint)
            .wrapping_add(yspc[p as usize] as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(yspc[p as usize] as libc::c_uint) as libc::c_int;
        if x != 0 && y != 0 {
            let mut img_len: stbi__uint32 = ((((*(*a).s).img_n * x * depth
                + 7 as libc::c_int >> 3 as libc::c_int) + 1 as libc::c_int) * y)
                as stbi__uint32;
            if stbi__create_png_image_raw(
                a,
                image_data,
                image_data_len,
                out_n,
                x as stbi__uint32,
                y as stbi__uint32,
                depth,
                color,
            ) == 0
            {
                stbi_free(final_0 as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            j = 0 as libc::c_int;
            while j < y {
                i = 0 as libc::c_int;
                while i < x {
                    let mut out_y: libc::c_int = j * yspc[p as usize]
                        + yorig[p as usize];
                    let mut out_x: libc::c_int = i * xspc[p as usize]
                        + xorig[p as usize];
                    memcpy(
                        final_0
                            .offset(
                                (out_y as libc::c_uint)
                                    .wrapping_mul((*(*a).s).img_x)
                                    .wrapping_mul(out_bytes as libc::c_uint) as isize,
                            )
                            .offset((out_x * out_bytes) as isize) as *mut libc::c_void,
                        ((*a).out).offset(((j * x + i) * out_bytes) as isize)
                            as *const libc::c_void,
                        out_bytes as libc::c_ulong,
                    );
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
            stbi_free((*a).out as *mut libc::c_void);
            image_data = image_data.offset(img_len as isize);
            image_data_len = (image_data_len as libc::c_uint).wrapping_sub(img_len)
                as stbi__uint32 as stbi__uint32;
        }
        p += 1;
        p;
    }
    (*a).out = final_0;
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__compute_transparency16(
    mut z: *mut stbi__png,
    mut tc: *mut stbi__uint16,
    mut out_n: libc::c_int,
) -> libc::c_int {
    let mut s: *mut stbi__context = (*z).s;
    let mut i: stbi__uint32 = 0;
    let mut pixel_count: stbi__uint32 = ((*s).img_x).wrapping_mul((*s).img_y);
    let mut p: *mut stbi__uint16 = (*z).out as *mut stbi__uint16;
    if out_n == 2 as libc::c_int || out_n == 4 as libc::c_int {} else {
        __assert_fail(
            b"out_n == 2 || out_n == 4\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            4884 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 67],
                &[libc::c_char; 67],
            >(b"int stbi__compute_transparency16(stbi__png *, stbi__uint16 *, int)\0"))
                .as_ptr(),
        );
    };
    if out_n == 2 as libc::c_int {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            *p
                .offset(
                    1 as libc::c_int as isize,
                ) = (if *p.offset(0 as libc::c_int as isize) as libc::c_int
                == *tc.offset(0 as libc::c_int as isize) as libc::c_int
            {
                0 as libc::c_int
            } else {
                65535 as libc::c_int
            }) as stbi__uint16;
            p = p.offset(2 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int
                == *tc.offset(0 as libc::c_int as isize) as libc::c_int
                && *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == *tc.offset(1 as libc::c_int as isize) as libc::c_int
                && *p.offset(2 as libc::c_int as isize) as libc::c_int
                    == *tc.offset(2 as libc::c_int as isize) as libc::c_int
            {
                *p.offset(3 as libc::c_int as isize) = 0 as libc::c_int as stbi__uint16;
            }
            p = p.offset(4 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__compute_transparency(
    mut z: *mut stbi__png,
    mut tc: *mut stbi_uc,
    mut out_n: libc::c_int,
) -> libc::c_int {
    let mut s: *mut stbi__context = (*z).s;
    let mut i: stbi__uint32 = 0;
    let mut pixel_count: stbi__uint32 = ((*s).img_x).wrapping_mul((*s).img_y);
    let mut p: *mut stbi_uc = (*z).out;
    if out_n == 2 as libc::c_int || out_n == 4 as libc::c_int {} else {
        __assert_fail(
            b"out_n == 2 || out_n == 4\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            4859 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[libc::c_char; 60],
            >(b"int stbi__compute_transparency(stbi__png *, stbi_uc *, int)\0"))
                .as_ptr(),
        );
    };
    if out_n == 2 as libc::c_int {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            *p
                .offset(
                    1 as libc::c_int as isize,
                ) = (if *p.offset(0 as libc::c_int as isize) as libc::c_int
                == *tc.offset(0 as libc::c_int as isize) as libc::c_int
            {
                0 as libc::c_int
            } else {
                255 as libc::c_int
            }) as stbi_uc;
            p = p.offset(2 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int
                == *tc.offset(0 as libc::c_int as isize) as libc::c_int
                && *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == *tc.offset(1 as libc::c_int as isize) as libc::c_int
                && *p.offset(2 as libc::c_int as isize) as libc::c_int
                    == *tc.offset(2 as libc::c_int as isize) as libc::c_int
            {
                *p.offset(3 as libc::c_int as isize) = 0 as libc::c_int as stbi_uc;
            }
            p = p.offset(4 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    return 1 as libc::c_int;
}
#[thread_local]
static mut stbi__de_iphone_flag_set: libc::c_int = 0;
#[thread_local]
static mut stbi__de_iphone_flag_local: libc::c_int = 0;
static mut stbi__de_iphone_flag_global: libc::c_int = 0 as libc::c_int;
#[thread_local]
static mut stbi__unpremultiply_on_load_set: libc::c_int = 0;
#[thread_local]
static mut stbi__unpremultiply_on_load_local: libc::c_int = 0;
static mut stbi__unpremultiply_on_load_global: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn stbi__de_iphone(mut z: *mut stbi__png) {
    let mut s: *mut stbi__context = (*z).s;
    let mut i: stbi__uint32 = 0;
    let mut pixel_count: stbi__uint32 = ((*s).img_x).wrapping_mul((*s).img_y);
    let mut p: *mut stbi_uc = (*z).out;
    if (*s).img_out_n == 3 as libc::c_int {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            let mut t: stbi_uc = *p.offset(0 as libc::c_int as isize);
            *p.offset(0 as libc::c_int as isize) = *p.offset(2 as libc::c_int as isize);
            *p.offset(2 as libc::c_int as isize) = t;
            p = p.offset(3 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    } else {
        if (*s).img_out_n == 4 as libc::c_int {} else {
            __assert_fail(
                b"s->img_out_n == 4\0" as *const u8 as *const libc::c_char,
                b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                4992 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void stbi__de_iphone(stbi__png *)\0"))
                    .as_ptr(),
            );
        };
        if if stbi__unpremultiply_on_load_set != 0 {
            stbi__unpremultiply_on_load_local
        } else {
            stbi__unpremultiply_on_load_global
        } != 0
        {
            i = 0 as libc::c_int as stbi__uint32;
            while i < pixel_count {
                let mut a: stbi_uc = *p.offset(3 as libc::c_int as isize);
                let mut t_0: stbi_uc = *p.offset(0 as libc::c_int as isize);
                if a != 0 {
                    let mut half: stbi_uc = (a as libc::c_int / 2 as libc::c_int)
                        as stbi_uc;
                    *p
                        .offset(
                            0 as libc::c_int as isize,
                        ) = ((*p.offset(2 as libc::c_int as isize) as libc::c_int
                        * 255 as libc::c_int + half as libc::c_int) / a as libc::c_int)
                        as stbi_uc;
                    *p
                        .offset(
                            1 as libc::c_int as isize,
                        ) = ((*p.offset(1 as libc::c_int as isize) as libc::c_int
                        * 255 as libc::c_int + half as libc::c_int) / a as libc::c_int)
                        as stbi_uc;
                    *p
                        .offset(
                            2 as libc::c_int as isize,
                        ) = ((t_0 as libc::c_int * 255 as libc::c_int
                        + half as libc::c_int) / a as libc::c_int) as stbi_uc;
                } else {
                    *p
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *p.offset(2 as libc::c_int as isize);
                    *p.offset(2 as libc::c_int as isize) = t_0;
                }
                p = p.offset(4 as libc::c_int as isize);
                i = i.wrapping_add(1);
                i;
            }
        } else {
            i = 0 as libc::c_int as stbi__uint32;
            while i < pixel_count {
                let mut t_1: stbi_uc = *p.offset(0 as libc::c_int as isize);
                *p
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *p.offset(2 as libc::c_int as isize);
                *p.offset(2 as libc::c_int as isize) = t_1;
                p = p.offset(4 as libc::c_int as isize);
                i = i.wrapping_add(1);
                i;
            }
        }
    };
}
unsafe extern "C" fn stbi__expand_png_palette(
    mut a: *mut stbi__png,
    mut palette: *mut stbi_uc,
    mut len: libc::c_int,
    mut pal_img_n: libc::c_int,
) -> libc::c_int {
    let mut i: stbi__uint32 = 0;
    let mut pixel_count: stbi__uint32 = ((*(*a).s).img_x).wrapping_mul((*(*a).s).img_y);
    let mut p: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut temp_out: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut orig: *mut stbi_uc = (*a).out;
    p = stbi__malloc_mad2(pixel_count as libc::c_int, pal_img_n, 0 as libc::c_int)
        as *mut stbi_uc;
    if p.is_null() {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    temp_out = p;
    if pal_img_n == 3 as libc::c_int {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            let mut n: libc::c_int = *orig.offset(i as isize) as libc::c_int
                * 4 as libc::c_int;
            *p.offset(0 as libc::c_int as isize) = *palette.offset(n as isize);
            *p
                .offset(
                    1 as libc::c_int as isize,
                ) = *palette.offset((n + 1 as libc::c_int) as isize);
            *p
                .offset(
                    2 as libc::c_int as isize,
                ) = *palette.offset((n + 2 as libc::c_int) as isize);
            p = p.offset(3 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    } else {
        i = 0 as libc::c_int as stbi__uint32;
        while i < pixel_count {
            let mut n_0: libc::c_int = *orig.offset(i as isize) as libc::c_int
                * 4 as libc::c_int;
            *p.offset(0 as libc::c_int as isize) = *palette.offset(n_0 as isize);
            *p
                .offset(
                    1 as libc::c_int as isize,
                ) = *palette.offset((n_0 + 1 as libc::c_int) as isize);
            *p
                .offset(
                    2 as libc::c_int as isize,
                ) = *palette.offset((n_0 + 2 as libc::c_int) as isize);
            *p
                .offset(
                    3 as libc::c_int as isize,
                ) = *palette.offset((n_0 + 3 as libc::c_int) as isize);
            p = p.offset(4 as libc::c_int as isize);
            i = i.wrapping_add(1);
            i;
        }
    }
    stbi_free((*a).out as *mut libc::c_void);
    (*a).out = temp_out;
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__get_chunk_header(
    mut s: *mut stbi__context,
) -> stbi__pngchunk {
    let mut c: stbi__pngchunk = stbi__pngchunk {
        length: 0,
        type_0: 0,
    };
    c.length = stbi__get32be(s);
    c.type_0 = stbi__get32be(s);
    return c;
}
unsafe extern "C" fn stbi__parse_png_file(
    mut z: *mut stbi__png,
    mut scan: libc::c_int,
    mut req_comp: libc::c_int,
) -> libc::c_int {
    let mut palette: [stbi_uc; 1024] = [0; 1024];
    let mut pal_img_n: stbi_uc = 0 as libc::c_int as stbi_uc;
    let mut has_trans: stbi_uc = 0 as libc::c_int as stbi_uc;
    let mut tc: [stbi_uc; 3] = [0 as libc::c_int as stbi_uc, 0, 0];
    let mut tc16: [stbi__uint16; 3] = [0; 3];
    let mut ioff: stbi__uint32 = 0 as libc::c_int as stbi__uint32;
    let mut idata_limit: stbi__uint32 = 0 as libc::c_int as stbi__uint32;
    let mut i: stbi__uint32 = 0;
    let mut pal_len: stbi__uint32 = 0 as libc::c_int as stbi__uint32;
    let mut first: libc::c_int = 1 as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut interlace: libc::c_int = 0 as libc::c_int;
    let mut color: libc::c_int = 0 as libc::c_int;
    let mut is_iphone: libc::c_int = 0 as libc::c_int;
    let mut s: *mut stbi__context = (*z).s;
    (*z).expanded = 0 as *mut stbi_uc;
    (*z).idata = 0 as *mut stbi_uc;
    (*z).out = 0 as *mut stbi_uc;
    if stbi__check_png_header(s) == 0 {
        return 0 as libc::c_int;
    }
    if scan == STBI__SCAN_type as libc::c_int {
        return 1 as libc::c_int;
    }
    loop {
        let mut c: stbi__pngchunk = stbi__get_chunk_header(s);
        match c.type_0 {
            1130840649 => {
                is_iphone = 1 as libc::c_int;
                stbi__skip(s, c.length as libc::c_int);
            }
            1229472850 => {
                let mut comp: libc::c_int = 0;
                let mut filter: libc::c_int = 0;
                if first == 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                first = 0 as libc::c_int;
                if c.length != 13 as libc::c_int as libc::c_uint {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*s).img_x = stbi__get32be(s);
                (*s).img_y = stbi__get32be(s);
                if (*s).img_y > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
                {
                    return stbi__err(
                        b"Very large image (corrupt?)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if (*s).img_x > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint
                {
                    return stbi__err(
                        b"Very large image (corrupt?)\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*z).depth = stbi__get8(s) as libc::c_int;
                if (*z).depth != 1 as libc::c_int && (*z).depth != 2 as libc::c_int
                    && (*z).depth != 4 as libc::c_int && (*z).depth != 8 as libc::c_int
                    && (*z).depth != 16 as libc::c_int
                {
                    return stbi__err(
                        b"PNG not supported: 1/2/4/8/16-bit only\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                color = stbi__get8(s) as libc::c_int;
                if color > 6 as libc::c_int {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if color == 3 as libc::c_int && (*z).depth == 16 as libc::c_int {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if color == 3 as libc::c_int {
                    pal_img_n = 3 as libc::c_int as stbi_uc;
                } else if color & 1 as libc::c_int != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    )
                }
                comp = stbi__get8(s) as libc::c_int;
                if comp != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                filter = stbi__get8(s) as libc::c_int;
                if filter != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                interlace = stbi__get8(s) as libc::c_int;
                if interlace > 1 as libc::c_int {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if (*s).img_x == 0 || (*s).img_y == 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if pal_img_n == 0 {
                    (*s)
                        .img_n = (if color & 2 as libc::c_int != 0 {
                        3 as libc::c_int
                    } else {
                        1 as libc::c_int
                    })
                        + (if color & 4 as libc::c_int != 0 {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        });
                    if (((1 as libc::c_int) << 30 as libc::c_int) as libc::c_uint)
                        .wrapping_div((*s).img_x)
                        .wrapping_div((*s).img_n as libc::c_uint) < (*s).img_y
                    {
                        return stbi__err(
                            b"Image too large to decode\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    if scan == STBI__SCAN_header as libc::c_int {
                        return 1 as libc::c_int;
                    }
                } else {
                    (*s).img_n = 1 as libc::c_int;
                    if (((1 as libc::c_int) << 30 as libc::c_int) as libc::c_uint)
                        .wrapping_div((*s).img_x)
                        .wrapping_div(4 as libc::c_int as libc::c_uint) < (*s).img_y
                    {
                        return stbi__err(
                            b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            1347179589 => {
                if first != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if c.length > (256 as libc::c_int * 3 as libc::c_int) as libc::c_uint {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                pal_len = (c.length).wrapping_div(3 as libc::c_int as libc::c_uint);
                if pal_len.wrapping_mul(3 as libc::c_int as libc::c_uint) != c.length {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                i = 0 as libc::c_int as stbi__uint32;
                while i < pal_len {
                    palette[i
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(0 as libc::c_int as libc::c_uint)
                        as usize] = stbi__get8(s);
                    palette[i
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as usize] = stbi__get8(s);
                    palette[i
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(2 as libc::c_int as libc::c_uint)
                        as usize] = stbi__get8(s);
                    palette[i
                        .wrapping_mul(4 as libc::c_int as libc::c_uint)
                        .wrapping_add(3 as libc::c_int as libc::c_uint)
                        as usize] = 255 as libc::c_int as stbi_uc;
                    i = i.wrapping_add(1);
                    i;
                }
            }
            1951551059 => {
                if first != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if !((*z).idata).is_null() {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if pal_img_n != 0 {
                    if scan == STBI__SCAN_header as libc::c_int {
                        (*s).img_n = 4 as libc::c_int;
                        return 1 as libc::c_int;
                    }
                    if pal_len == 0 as libc::c_int as libc::c_uint {
                        return stbi__err(
                            b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if c.length > pal_len {
                        return stbi__err(
                            b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    pal_img_n = 4 as libc::c_int as stbi_uc;
                    i = 0 as libc::c_int as stbi__uint32;
                    while i < c.length {
                        palette[i
                            .wrapping_mul(4 as libc::c_int as libc::c_uint)
                            .wrapping_add(3 as libc::c_int as libc::c_uint)
                            as usize] = stbi__get8(s);
                        i = i.wrapping_add(1);
                        i;
                    }
                } else {
                    if (*s).img_n & 1 as libc::c_int == 0 {
                        return stbi__err(
                            b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if c.length
                        != ((*s).img_n as stbi__uint32)
                            .wrapping_mul(2 as libc::c_int as libc::c_uint)
                    {
                        return stbi__err(
                            b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    has_trans = 1 as libc::c_int as stbi_uc;
                    if (*z).depth == 16 as libc::c_int {
                        k = 0 as libc::c_int;
                        while k < (*s).img_n {
                            tc16[k as usize] = stbi__get16be(s) as stbi__uint16;
                            k += 1;
                            k;
                        }
                    } else {
                        k = 0 as libc::c_int;
                        while k < (*s).img_n {
                            tc[k
                                as usize] = ((stbi__get16be(s) & 255 as libc::c_int)
                                as stbi_uc as libc::c_int
                                * stbi__depth_scale_table[(*z).depth as usize]
                                    as libc::c_int) as stbi_uc;
                            k += 1;
                            k;
                        }
                    }
                }
            }
            1229209940 => {
                if first != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if pal_img_n as libc::c_int != 0 && pal_len == 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if scan == STBI__SCAN_header as libc::c_int {
                    (*s).img_n = pal_img_n as libc::c_int;
                    return 1 as libc::c_int;
                }
                if (ioff.wrapping_add(c.length) as libc::c_int) < ioff as libc::c_int {
                    return 0 as libc::c_int;
                }
                if ioff.wrapping_add(c.length) > idata_limit {
                    let mut idata_limit_old: stbi__uint32 = idata_limit;
                    let mut p: *mut stbi_uc = 0 as *mut stbi_uc;
                    if idata_limit == 0 as libc::c_int as libc::c_uint {
                        idata_limit = if c.length > 4096 as libc::c_int as libc::c_uint {
                            c.length
                        } else {
                            4096 as libc::c_int as libc::c_uint
                        };
                    }
                    while ioff.wrapping_add(c.length) > idata_limit {
                        idata_limit = (idata_limit as libc::c_uint)
                            .wrapping_mul(2 as libc::c_int as libc::c_uint)
                            as stbi__uint32 as stbi__uint32;
                    }
                    p = stbi_realloc(
                        (*z).idata as *mut libc::c_void,
                        idata_limit as size_t,
                    ) as *mut stbi_uc;
                    if p.is_null() {
                        return stbi__err(
                            b"Out of memory\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    (*z).idata = p;
                }
                if stbi__getn(
                    s,
                    ((*z).idata).offset(ioff as isize),
                    c.length as libc::c_int,
                ) == 0
                {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                ioff = (ioff as libc::c_uint).wrapping_add(c.length) as stbi__uint32
                    as stbi__uint32;
            }
            1229278788 => {
                let mut raw_len: stbi__uint32 = 0;
                let mut bpl: stbi__uint32 = 0;
                if first != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if scan != STBI__SCAN_load as libc::c_int {
                    return 1 as libc::c_int;
                }
                if ((*z).idata).is_null() {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                bpl = ((*s).img_x)
                    .wrapping_mul((*z).depth as libc::c_uint)
                    .wrapping_add(7 as libc::c_int as libc::c_uint)
                    .wrapping_div(8 as libc::c_int as libc::c_uint);
                raw_len = bpl
                    .wrapping_mul((*s).img_y)
                    .wrapping_mul((*s).img_n as libc::c_uint)
                    .wrapping_add((*s).img_y);
                (*z)
                    .expanded = stbi_zlib_decode_malloc_guesssize_headerflag(
                    (*z).idata as *mut libc::c_char,
                    ioff as libc::c_int,
                    raw_len as libc::c_int,
                    &mut raw_len as *mut stbi__uint32 as *mut libc::c_int,
                    (is_iphone == 0) as libc::c_int,
                ) as *mut stbi_uc;
                if ((*z).expanded).is_null() {
                    return 0 as libc::c_int;
                }
                stbi_free((*z).idata as *mut libc::c_void);
                (*z).idata = 0 as *mut stbi_uc;
                if req_comp == (*s).img_n + 1 as libc::c_int
                    && req_comp != 3 as libc::c_int && pal_img_n == 0
                    || has_trans as libc::c_int != 0
                {
                    (*s).img_out_n = (*s).img_n + 1 as libc::c_int;
                } else {
                    (*s).img_out_n = (*s).img_n;
                }
                if stbi__create_png_image(
                    z,
                    (*z).expanded,
                    raw_len,
                    (*s).img_out_n,
                    (*z).depth,
                    color,
                    interlace,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if has_trans != 0 {
                    if (*z).depth == 16 as libc::c_int {
                        if stbi__compute_transparency16(
                            z,
                            tc16.as_mut_ptr(),
                            (*s).img_out_n,
                        ) == 0
                        {
                            return 0 as libc::c_int;
                        }
                    } else if stbi__compute_transparency(
                        z,
                        tc.as_mut_ptr(),
                        (*s).img_out_n,
                    ) == 0
                    {
                        return 0 as libc::c_int
                    }
                }
                if is_iphone != 0
                    && (if stbi__de_iphone_flag_set != 0 {
                        stbi__de_iphone_flag_local
                    } else {
                        stbi__de_iphone_flag_global
                    }) != 0 && (*s).img_out_n > 2 as libc::c_int
                {
                    stbi__de_iphone(z);
                }
                if pal_img_n != 0 {
                    (*s).img_n = pal_img_n as libc::c_int;
                    (*s).img_out_n = pal_img_n as libc::c_int;
                    if req_comp >= 3 as libc::c_int {
                        (*s).img_out_n = req_comp;
                    }
                    if stbi__expand_png_palette(
                        z,
                        palette.as_mut_ptr(),
                        pal_len as libc::c_int,
                        (*s).img_out_n,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                } else if has_trans != 0 {
                    (*s).img_n += 1;
                    (*s).img_n;
                }
                stbi_free((*z).expanded as *mut libc::c_void);
                (*z).expanded = 0 as *mut stbi_uc;
                stbi__get32be(s);
                return 1 as libc::c_int;
            }
            _ => {
                if first != 0 {
                    return stbi__err(
                        b"Corrupt PNG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if c.type_0 & ((1 as libc::c_int) << 29 as libc::c_int) as libc::c_uint
                    == 0 as libc::c_int as libc::c_uint
                {
                    static mut invalid_chunk: [libc::c_char; 25] = unsafe {
                        *::core::mem::transmute::<
                            &[u8; 25],
                            &mut [libc::c_char; 25],
                        >(b"XXXX PNG chunk not known\0")
                    };
                    invalid_chunk[0 as libc::c_int
                        as usize] = (c.type_0 >> 24 as libc::c_int
                        & 255 as libc::c_int as libc::c_uint) as stbi_uc as libc::c_char;
                    invalid_chunk[1 as libc::c_int
                        as usize] = (c.type_0 >> 16 as libc::c_int
                        & 255 as libc::c_int as libc::c_uint) as stbi_uc as libc::c_char;
                    invalid_chunk[2 as libc::c_int
                        as usize] = (c.type_0 >> 8 as libc::c_int
                        & 255 as libc::c_int as libc::c_uint) as stbi_uc as libc::c_char;
                    invalid_chunk[3 as libc::c_int
                        as usize] = (c.type_0 >> 0 as libc::c_int
                        & 255 as libc::c_int as libc::c_uint) as stbi_uc as libc::c_char;
                    return stbi__err(
                        b"PNG not supported: unknown PNG chunk type\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                stbi__skip(s, c.length as libc::c_int);
            }
        }
        stbi__get32be(s);
    };
}
unsafe extern "C" fn stbi__do_png(
    mut p: *mut stbi__png,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut n: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    if req_comp < 0 as libc::c_int || req_comp > 4 as libc::c_int {
        return (if stbi__err(b"Internal error\0" as *const u8 as *const libc::c_char)
            != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if stbi__parse_png_file(p, STBI__SCAN_load as libc::c_int, req_comp) != 0 {
        if (*p).depth <= 8 as libc::c_int {
            (*ri).bits_per_channel = 8 as libc::c_int;
        } else if (*p).depth == 16 as libc::c_int {
            (*ri).bits_per_channel = 16 as libc::c_int;
        } else {
            return (if stbi__err(
                b"PNG not supported: unsupported color depth\0" as *const u8
                    as *const libc::c_char,
            ) != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void
        }
        result = (*p).out as *mut libc::c_void;
        (*p).out = 0 as *mut stbi_uc;
        if req_comp != 0 && req_comp != (*(*p).s).img_out_n {
            if (*ri).bits_per_channel == 8 as libc::c_int {
                result = stbi__convert_format(
                    result as *mut libc::c_uchar,
                    (*(*p).s).img_out_n,
                    req_comp,
                    (*(*p).s).img_x,
                    (*(*p).s).img_y,
                ) as *mut libc::c_void;
            } else {
                result = stbi__convert_format16(
                    result as *mut stbi__uint16,
                    (*(*p).s).img_out_n,
                    req_comp,
                    (*(*p).s).img_x,
                    (*(*p).s).img_y,
                ) as *mut libc::c_void;
            }
            (*(*p).s).img_out_n = req_comp;
            if result.is_null() {
                return result;
            }
        }
        *x = (*(*p).s).img_x as libc::c_int;
        *y = (*(*p).s).img_y as libc::c_int;
        if !n.is_null() {
            *n = (*(*p).s).img_n;
        }
    }
    stbi_free((*p).out as *mut libc::c_void);
    (*p).out = 0 as *mut stbi_uc;
    stbi_free((*p).expanded as *mut libc::c_void);
    (*p).expanded = 0 as *mut stbi_uc;
    stbi_free((*p).idata as *mut libc::c_void);
    (*p).idata = 0 as *mut stbi_uc;
    return result;
}
unsafe extern "C" fn stbi__png_load(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_void {
    let mut p: stbi__png = stbi__png {
        s: 0 as *mut stbi__context,
        idata: 0 as *mut stbi_uc,
        expanded: 0 as *mut stbi_uc,
        out: 0 as *mut stbi_uc,
        depth: 0,
    };
    p.s = s;
    return stbi__do_png(&mut p, x, y, comp, req_comp, ri);
}
unsafe extern "C" fn stbi__bmp_test_raw(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut sz: libc::c_int = 0;
    if stbi__get8(s) as libc::c_int != 'B' as i32 {
        return 0 as libc::c_int;
    }
    if stbi__get8(s) as libc::c_int != 'M' as i32 {
        return 0 as libc::c_int;
    }
    stbi__get32le(s);
    stbi__get16le(s);
    stbi__get16le(s);
    stbi__get32le(s);
    sz = stbi__get32le(s) as libc::c_int;
    r = (sz == 12 as libc::c_int || sz == 40 as libc::c_int || sz == 56 as libc::c_int
        || sz == 108 as libc::c_int || sz == 124 as libc::c_int) as libc::c_int;
    return r;
}
unsafe extern "C" fn stbi__bmp_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = stbi__bmp_test_raw(s);
    stbi__rewind(s);
    return r;
}
unsafe extern "C" fn stbi__bmp_set_mask_defaults(
    mut info: *mut stbi__bmp_data,
    mut compress: libc::c_int,
) -> libc::c_int {
    if compress == 3 as libc::c_int {
        return 1 as libc::c_int;
    }
    if compress == 0 as libc::c_int {
        if (*info).bpp == 16 as libc::c_int {
            (*info).mr = (31 as libc::c_uint) << 10 as libc::c_int;
            (*info).mg = (31 as libc::c_uint) << 5 as libc::c_int;
            (*info).mb = (31 as libc::c_uint) << 0 as libc::c_int;
        } else if (*info).bpp == 32 as libc::c_int {
            (*info).mr = (0xff as libc::c_uint) << 16 as libc::c_int;
            (*info).mg = (0xff as libc::c_uint) << 8 as libc::c_int;
            (*info).mb = (0xff as libc::c_uint) << 0 as libc::c_int;
            (*info).ma = (0xff as libc::c_uint) << 24 as libc::c_int;
            (*info).all_a = 0 as libc::c_int as libc::c_uint;
        } else {
            (*info).ma = 0 as libc::c_int as libc::c_uint;
            (*info).mb = (*info).ma;
            (*info).mg = (*info).mb;
            (*info).mr = (*info).mg;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stbi__bmp_parse_header(
    mut s: *mut stbi__context,
    mut info: *mut stbi__bmp_data,
) -> *mut libc::c_void {
    let mut hsz: libc::c_int = 0;
    if stbi__get8(s) as libc::c_int != 'B' as i32
        || stbi__get8(s) as libc::c_int != 'M' as i32
    {
        return (if stbi__err(b"Corrupt BMP\0" as *const u8 as *const libc::c_char) != 0 {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    stbi__get32le(s);
    stbi__get16le(s);
    stbi__get16le(s);
    (*info).offset = stbi__get32le(s) as libc::c_int;
    hsz = stbi__get32le(s) as libc::c_int;
    (*info).hsz = hsz;
    (*info).ma = 0 as libc::c_int as libc::c_uint;
    (*info).mb = (*info).ma;
    (*info).mg = (*info).mb;
    (*info).mr = (*info).mg;
    (*info).extra_read = 14 as libc::c_int;
    if (*info).offset < 0 as libc::c_int {
        return (if stbi__err(b"bad BMP\0" as *const u8 as *const libc::c_char) != 0 {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if hsz != 12 as libc::c_int && hsz != 40 as libc::c_int && hsz != 56 as libc::c_int
        && hsz != 108 as libc::c_int && hsz != 124 as libc::c_int
    {
        return (if stbi__err(
            b"BMP type not supported: unknown\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if hsz == 12 as libc::c_int {
        (*s).img_x = stbi__get16le(s) as stbi__uint32;
        (*s).img_y = stbi__get16le(s) as stbi__uint32;
    } else {
        (*s).img_x = stbi__get32le(s);
        (*s).img_y = stbi__get32le(s);
    }
    if stbi__get16le(s) != 1 as libc::c_int {
        return (if stbi__err(b"bad BMP\0" as *const u8 as *const libc::c_char) != 0 {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    (*info).bpp = stbi__get16le(s);
    if hsz != 12 as libc::c_int {
        let mut compress: libc::c_int = stbi__get32le(s) as libc::c_int;
        if compress == 1 as libc::c_int || compress == 2 as libc::c_int {
            return (if stbi__err(
                b"BMP type not supported: RLE\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
        }
        if compress >= 4 as libc::c_int {
            return (if stbi__err(
                b"BMP type not supported: unsupported compression\0" as *const u8
                    as *const libc::c_char,
            ) != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
        }
        if compress == 3 as libc::c_int && (*info).bpp != 16 as libc::c_int
            && (*info).bpp != 32 as libc::c_int
        {
            return (if stbi__err(b"bad BMP\0" as *const u8 as *const libc::c_char) != 0 {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
        }
        stbi__get32le(s);
        stbi__get32le(s);
        stbi__get32le(s);
        stbi__get32le(s);
        stbi__get32le(s);
        if hsz == 40 as libc::c_int || hsz == 56 as libc::c_int {
            if hsz == 56 as libc::c_int {
                stbi__get32le(s);
                stbi__get32le(s);
                stbi__get32le(s);
                stbi__get32le(s);
            }
            if (*info).bpp == 16 as libc::c_int || (*info).bpp == 32 as libc::c_int {
                if compress == 0 as libc::c_int {
                    stbi__bmp_set_mask_defaults(info, compress);
                } else if compress == 3 as libc::c_int {
                    (*info).mr = stbi__get32le(s);
                    (*info).mg = stbi__get32le(s);
                    (*info).mb = stbi__get32le(s);
                    (*info).extra_read += 12 as libc::c_int;
                    if (*info).mr == (*info).mg && (*info).mg == (*info).mb {
                        return (if stbi__err(
                            b"bad BMP\0" as *const u8 as *const libc::c_char,
                        ) != 0
                        {
                            0 as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
                    }
                } else {
                    return (if stbi__err(
                        b"bad BMP\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    {
                        0 as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    }) as size_t as *mut libc::c_uchar as *mut libc::c_void
                }
            }
        } else {
            let mut i: libc::c_int = 0;
            if hsz != 108 as libc::c_int && hsz != 124 as libc::c_int {
                return (if stbi__err(b"bad BMP\0" as *const u8 as *const libc::c_char)
                    != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
            (*info).mr = stbi__get32le(s);
            (*info).mg = stbi__get32le(s);
            (*info).mb = stbi__get32le(s);
            (*info).ma = stbi__get32le(s);
            if compress != 3 as libc::c_int {
                stbi__bmp_set_mask_defaults(info, compress);
            }
            stbi__get32le(s);
            i = 0 as libc::c_int;
            while i < 12 as libc::c_int {
                stbi__get32le(s);
                i += 1;
                i;
            }
            if hsz == 124 as libc::c_int {
                stbi__get32le(s);
                stbi__get32le(s);
                stbi__get32le(s);
                stbi__get32le(s);
            }
        }
    }
    return 1 as libc::c_int as *mut libc::c_void;
}
unsafe extern "C" fn stbi__high_bit(mut z: libc::c_uint) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    if z == 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    if z >= 0x10000 as libc::c_int as libc::c_uint {
        n += 16 as libc::c_int;
        z >>= 16 as libc::c_int;
    }
    if z >= 0x100 as libc::c_int as libc::c_uint {
        n += 8 as libc::c_int;
        z >>= 8 as libc::c_int;
    }
    if z >= 0x10 as libc::c_int as libc::c_uint {
        n += 4 as libc::c_int;
        z >>= 4 as libc::c_int;
    }
    if z >= 0x4 as libc::c_int as libc::c_uint {
        n += 2 as libc::c_int;
        z >>= 2 as libc::c_int;
    }
    if z >= 0x2 as libc::c_int as libc::c_uint {
        n += 1 as libc::c_int;
    }
    return n;
}
unsafe extern "C" fn stbi__bitcount(mut a: libc::c_uint) -> libc::c_int {
    a = (a & 0x55555555 as libc::c_int as libc::c_uint)
        .wrapping_add(a >> 1 as libc::c_int & 0x55555555 as libc::c_int as libc::c_uint);
    a = (a & 0x33333333 as libc::c_int as libc::c_uint)
        .wrapping_add(a >> 2 as libc::c_int & 0x33333333 as libc::c_int as libc::c_uint);
    a = a.wrapping_add(a >> 4 as libc::c_int) & 0xf0f0f0f as libc::c_int as libc::c_uint;
    a = a.wrapping_add(a >> 8 as libc::c_int);
    a = a.wrapping_add(a >> 16 as libc::c_int);
    return (a & 0xff as libc::c_int as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn stbi__shiftsigned(
    mut v: libc::c_uint,
    mut shift: libc::c_int,
    mut bits: libc::c_int,
) -> libc::c_int {
    static mut mul_table: [libc::c_uint; 9] = [
        0 as libc::c_int as libc::c_uint,
        0xff as libc::c_int as libc::c_uint,
        0x55 as libc::c_int as libc::c_uint,
        0x49 as libc::c_int as libc::c_uint,
        0x11 as libc::c_int as libc::c_uint,
        0x21 as libc::c_int as libc::c_uint,
        0x41 as libc::c_int as libc::c_uint,
        0x81 as libc::c_int as libc::c_uint,
        0x1 as libc::c_int as libc::c_uint,
    ];
    static mut shift_table: [libc::c_uint; 9] = [
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        6 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    ];
    if shift < 0 as libc::c_int {
        v <<= -shift;
    } else {
        v >>= shift;
    }
    if v < 256 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"v < 256\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            5345 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int stbi__shiftsigned(unsigned int, int, int)\0"))
                .as_ptr(),
        );
    };
    v >>= 8 as libc::c_int - bits;
    if bits >= 0 as libc::c_int && bits <= 8 as libc::c_int {} else {
        __assert_fail(
            b"bits >= 0 && bits <= 8\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            5347 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 46],
                &[libc::c_char; 46],
            >(b"int stbi__shiftsigned(unsigned int, int, int)\0"))
                .as_ptr(),
        );
    };
    return v.wrapping_mul(mul_table[bits as usize]) as libc::c_int
        >> shift_table[bits as usize];
}
unsafe extern "C" fn stbi__get32le(mut s: *mut stbi__context) -> stbi__uint32 {
    let mut z: stbi__uint32 = stbi__get16le(s) as stbi__uint32;
    z = (z as libc::c_uint)
        .wrapping_add((stbi__get16le(s) as stbi__uint32) << 16 as libc::c_int)
        as stbi__uint32 as stbi__uint32;
    return z;
}
unsafe extern "C" fn stbi__bmp_load(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_void {
    let mut out: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut mr: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mg: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mb: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut ma: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut all_a: libc::c_uint = 0;
    let mut pal: [[stbi_uc; 4]; 256] = [[0; 4]; 256];
    let mut psize: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut flip_vertically: libc::c_int = 0;
    let mut pad: libc::c_int = 0;
    let mut target: libc::c_int = 0;
    let mut info: stbi__bmp_data = stbi__bmp_data {
        bpp: 0,
        offset: 0,
        hsz: 0,
        mr: 0,
        mg: 0,
        mb: 0,
        ma: 0,
        all_a: 0,
        extra_read: 0,
    };
    info.all_a = 255 as libc::c_int as libc::c_uint;
    if (stbi__bmp_parse_header(s, &mut info)).is_null() {
        return 0 as *mut libc::c_void;
    }
    flip_vertically = ((*s).img_y as libc::c_int > 0 as libc::c_int) as libc::c_int;
    (*s).img_y = abs((*s).img_y as libc::c_int) as stbi__uint32;
    if (*s).img_y > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if (*s).img_x > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    mr = info.mr;
    mg = info.mg;
    mb = info.mb;
    ma = info.ma;
    all_a = info.all_a;
    if info.hsz == 12 as libc::c_int {
        if info.bpp < 24 as libc::c_int {
            psize = (info.offset - info.extra_read - 24 as libc::c_int)
                / 3 as libc::c_int;
        }
    } else if info.bpp < 16 as libc::c_int {
        psize = info.offset - info.extra_read - info.hsz >> 2 as libc::c_int;
    }
    if psize == 0 as libc::c_int {
        if info.offset as libc::c_long
            != (*s).callback_already_read as libc::c_long
                + ((*s).img_buffer).offset_from((*s).img_buffer_original) as libc::c_long
        {
            return (if stbi__err(b"Corrupt BMP\0" as *const u8 as *const libc::c_char)
                != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
        }
    }
    if info.bpp == 24 as libc::c_int && ma == 0xff000000 as libc::c_uint {
        (*s).img_n = 3 as libc::c_int;
    } else {
        (*s).img_n = if ma != 0 { 4 as libc::c_int } else { 3 as libc::c_int };
    }
    if req_comp != 0 && req_comp >= 3 as libc::c_int {
        target = req_comp;
    } else {
        target = (*s).img_n;
    }
    if stbi__mad3sizes_valid(
        target,
        (*s).img_x as libc::c_int,
        (*s).img_y as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return (if stbi__err(b"Corrupt BMP\0" as *const u8 as *const libc::c_char) != 0 {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    out = stbi__malloc_mad3(
        target,
        (*s).img_x as libc::c_int,
        (*s).img_y as libc::c_int,
        0 as libc::c_int,
    ) as *mut stbi_uc;
    if out.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if info.bpp < 16 as libc::c_int {
        let mut z: libc::c_int = 0 as libc::c_int;
        if psize == 0 as libc::c_int || psize > 256 as libc::c_int {
            stbi_free(out as *mut libc::c_void);
            return (if stbi__err(b"Corrupt BMP\0" as *const u8 as *const libc::c_char)
                != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
        }
        i = 0 as libc::c_int;
        while i < psize {
            pal[i as usize][2 as libc::c_int as usize] = stbi__get8(s);
            pal[i as usize][1 as libc::c_int as usize] = stbi__get8(s);
            pal[i as usize][0 as libc::c_int as usize] = stbi__get8(s);
            if info.hsz != 12 as libc::c_int {
                stbi__get8(s);
            }
            pal[i as usize][3 as libc::c_int as usize] = 255 as libc::c_int as stbi_uc;
            i += 1;
            i;
        }
        stbi__skip(
            s,
            info.offset - info.extra_read - info.hsz
                - psize
                    * (if info.hsz == 12 as libc::c_int {
                        3 as libc::c_int
                    } else {
                        4 as libc::c_int
                    }),
        );
        if info.bpp == 1 as libc::c_int {
            width = (((*s).img_x).wrapping_add(7 as libc::c_int as libc::c_uint)
                >> 3 as libc::c_int) as libc::c_int;
        } else if info.bpp == 4 as libc::c_int {
            width = (((*s).img_x).wrapping_add(1 as libc::c_int as libc::c_uint)
                >> 1 as libc::c_int) as libc::c_int;
        } else if info.bpp == 8 as libc::c_int {
            width = (*s).img_x as libc::c_int;
        } else {
            stbi_free(out as *mut libc::c_void);
            return (if stbi__err(b"Corrupt BMP\0" as *const u8 as *const libc::c_char)
                != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
        }
        pad = -width & 3 as libc::c_int;
        if info.bpp == 1 as libc::c_int {
            j = 0 as libc::c_int;
            while j < (*s).img_y as libc::c_int {
                let mut bit_offset: libc::c_int = 7 as libc::c_int;
                let mut v: libc::c_int = stbi__get8(s) as libc::c_int;
                i = 0 as libc::c_int;
                while i < (*s).img_x as libc::c_int {
                    let mut color: libc::c_int = v >> bit_offset & 0x1 as libc::c_int;
                    let fresh35 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh35 as isize,
                        ) = pal[color as usize][0 as libc::c_int as usize];
                    let fresh36 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh36 as isize,
                        ) = pal[color as usize][1 as libc::c_int as usize];
                    let fresh37 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh37 as isize,
                        ) = pal[color as usize][2 as libc::c_int as usize];
                    if target == 4 as libc::c_int {
                        let fresh38 = z;
                        z = z + 1;
                        *out.offset(fresh38 as isize) = 255 as libc::c_int as stbi_uc;
                    }
                    if i + 1 as libc::c_int == (*s).img_x as libc::c_int {
                        break;
                    }
                    bit_offset -= 1;
                    if bit_offset < 0 as libc::c_int {
                        bit_offset = 7 as libc::c_int;
                        v = stbi__get8(s) as libc::c_int;
                    }
                    i += 1;
                    i;
                }
                stbi__skip(s, pad);
                j += 1;
                j;
            }
        } else {
            j = 0 as libc::c_int;
            while j < (*s).img_y as libc::c_int {
                i = 0 as libc::c_int;
                while i < (*s).img_x as libc::c_int {
                    let mut v_0: libc::c_int = stbi__get8(s) as libc::c_int;
                    let mut v2: libc::c_int = 0 as libc::c_int;
                    if info.bpp == 4 as libc::c_int {
                        v2 = v_0 & 15 as libc::c_int;
                        v_0 >>= 4 as libc::c_int;
                    }
                    let fresh39 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh39 as isize,
                        ) = pal[v_0 as usize][0 as libc::c_int as usize];
                    let fresh40 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh40 as isize,
                        ) = pal[v_0 as usize][1 as libc::c_int as usize];
                    let fresh41 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh41 as isize,
                        ) = pal[v_0 as usize][2 as libc::c_int as usize];
                    if target == 4 as libc::c_int {
                        let fresh42 = z;
                        z = z + 1;
                        *out.offset(fresh42 as isize) = 255 as libc::c_int as stbi_uc;
                    }
                    if i + 1 as libc::c_int == (*s).img_x as libc::c_int {
                        break;
                    }
                    v_0 = if info.bpp == 8 as libc::c_int {
                        stbi__get8(s) as libc::c_int
                    } else {
                        v2
                    };
                    let fresh43 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh43 as isize,
                        ) = pal[v_0 as usize][0 as libc::c_int as usize];
                    let fresh44 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh44 as isize,
                        ) = pal[v_0 as usize][1 as libc::c_int as usize];
                    let fresh45 = z;
                    z = z + 1;
                    *out
                        .offset(
                            fresh45 as isize,
                        ) = pal[v_0 as usize][2 as libc::c_int as usize];
                    if target == 4 as libc::c_int {
                        let fresh46 = z;
                        z = z + 1;
                        *out.offset(fresh46 as isize) = 255 as libc::c_int as stbi_uc;
                    }
                    i += 2 as libc::c_int;
                }
                stbi__skip(s, pad);
                j += 1;
                j;
            }
        }
    } else {
        let mut rshift: libc::c_int = 0 as libc::c_int;
        let mut gshift: libc::c_int = 0 as libc::c_int;
        let mut bshift: libc::c_int = 0 as libc::c_int;
        let mut ashift: libc::c_int = 0 as libc::c_int;
        let mut rcount: libc::c_int = 0 as libc::c_int;
        let mut gcount: libc::c_int = 0 as libc::c_int;
        let mut bcount: libc::c_int = 0 as libc::c_int;
        let mut acount: libc::c_int = 0 as libc::c_int;
        let mut z_0: libc::c_int = 0 as libc::c_int;
        let mut easy: libc::c_int = 0 as libc::c_int;
        stbi__skip(s, info.offset - info.extra_read - info.hsz);
        if info.bpp == 24 as libc::c_int {
            width = (3 as libc::c_int as libc::c_uint).wrapping_mul((*s).img_x)
                as libc::c_int;
        } else if info.bpp == 16 as libc::c_int {
            width = (2 as libc::c_int as libc::c_uint).wrapping_mul((*s).img_x)
                as libc::c_int;
        } else {
            width = 0 as libc::c_int;
        }
        pad = -width & 3 as libc::c_int;
        if info.bpp == 24 as libc::c_int {
            easy = 1 as libc::c_int;
        } else if info.bpp == 32 as libc::c_int {
            if mb == 0xff as libc::c_int as libc::c_uint
                && mg == 0xff00 as libc::c_int as libc::c_uint
                && mr == 0xff0000 as libc::c_int as libc::c_uint
                && ma == 0xff000000 as libc::c_uint
            {
                easy = 2 as libc::c_int;
            }
        }
        if easy == 0 {
            if mr == 0 || mg == 0 || mb == 0 {
                stbi_free(out as *mut libc::c_void);
                return (if stbi__err(
                    b"Corrupt BMP\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
            rshift = stbi__high_bit(mr) - 7 as libc::c_int;
            rcount = stbi__bitcount(mr);
            gshift = stbi__high_bit(mg) - 7 as libc::c_int;
            gcount = stbi__bitcount(mg);
            bshift = stbi__high_bit(mb) - 7 as libc::c_int;
            bcount = stbi__bitcount(mb);
            ashift = stbi__high_bit(ma) - 7 as libc::c_int;
            acount = stbi__bitcount(ma);
            if rcount > 8 as libc::c_int || gcount > 8 as libc::c_int
                || bcount > 8 as libc::c_int || acount > 8 as libc::c_int
            {
                stbi_free(out as *mut libc::c_void);
                return (if stbi__err(
                    b"Corrupt BMP\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
        }
        j = 0 as libc::c_int;
        while j < (*s).img_y as libc::c_int {
            if easy != 0 {
                i = 0 as libc::c_int;
                while i < (*s).img_x as libc::c_int {
                    let mut a: libc::c_uchar = 0;
                    *out.offset((z_0 + 2 as libc::c_int) as isize) = stbi__get8(s);
                    *out.offset((z_0 + 1 as libc::c_int) as isize) = stbi__get8(s);
                    *out.offset((z_0 + 0 as libc::c_int) as isize) = stbi__get8(s);
                    z_0 += 3 as libc::c_int;
                    a = (if easy == 2 as libc::c_int {
                        stbi__get8(s) as libc::c_int
                    } else {
                        255 as libc::c_int
                    }) as libc::c_uchar;
                    all_a |= a as libc::c_uint;
                    if target == 4 as libc::c_int {
                        let fresh47 = z_0;
                        z_0 = z_0 + 1;
                        *out.offset(fresh47 as isize) = a;
                    }
                    i += 1;
                    i;
                }
            } else {
                let mut bpp: libc::c_int = info.bpp;
                i = 0 as libc::c_int;
                while i < (*s).img_x as libc::c_int {
                    let mut v_1: stbi__uint32 = if bpp == 16 as libc::c_int {
                        stbi__get16le(s) as stbi__uint32
                    } else {
                        stbi__get32le(s)
                    };
                    let mut a_0: libc::c_uint = 0;
                    let fresh48 = z_0;
                    z_0 = z_0 + 1;
                    *out
                        .offset(
                            fresh48 as isize,
                        ) = (stbi__shiftsigned(v_1 & mr, rshift, rcount)
                        & 255 as libc::c_int) as stbi_uc;
                    let fresh49 = z_0;
                    z_0 = z_0 + 1;
                    *out
                        .offset(
                            fresh49 as isize,
                        ) = (stbi__shiftsigned(v_1 & mg, gshift, gcount)
                        & 255 as libc::c_int) as stbi_uc;
                    let fresh50 = z_0;
                    z_0 = z_0 + 1;
                    *out
                        .offset(
                            fresh50 as isize,
                        ) = (stbi__shiftsigned(v_1 & mb, bshift, bcount)
                        & 255 as libc::c_int) as stbi_uc;
                    a_0 = (if ma != 0 {
                        stbi__shiftsigned(v_1 & ma, ashift, acount)
                    } else {
                        255 as libc::c_int
                    }) as libc::c_uint;
                    all_a |= a_0;
                    if target == 4 as libc::c_int {
                        let fresh51 = z_0;
                        z_0 = z_0 + 1;
                        *out
                            .offset(
                                fresh51 as isize,
                            ) = (a_0 & 255 as libc::c_int as libc::c_uint) as stbi_uc;
                    }
                    i += 1;
                    i;
                }
            }
            stbi__skip(s, pad);
            j += 1;
            j;
        }
    }
    if target == 4 as libc::c_int && all_a == 0 as libc::c_int as libc::c_uint {
        i = (4 as libc::c_int as libc::c_uint)
            .wrapping_mul((*s).img_x)
            .wrapping_mul((*s).img_y)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
        while i >= 0 as libc::c_int {
            *out.offset(i as isize) = 255 as libc::c_int as stbi_uc;
            i -= 4 as libc::c_int;
        }
    }
    if flip_vertically != 0 {
        let mut t: stbi_uc = 0;
        j = 0 as libc::c_int;
        while j < (*s).img_y as libc::c_int >> 1 as libc::c_int {
            let mut p1: *mut stbi_uc = out
                .offset(
                    (j as libc::c_uint)
                        .wrapping_mul((*s).img_x)
                        .wrapping_mul(target as libc::c_uint) as isize,
                );
            let mut p2: *mut stbi_uc = out
                .offset(
                    ((*s).img_y)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        .wrapping_sub(j as libc::c_uint)
                        .wrapping_mul((*s).img_x)
                        .wrapping_mul(target as libc::c_uint) as isize,
                );
            i = 0 as libc::c_int;
            while i < (*s).img_x as libc::c_int * target {
                t = *p1.offset(i as isize);
                *p1.offset(i as isize) = *p2.offset(i as isize);
                *p2.offset(i as isize) = t;
                i += 1;
                i;
            }
            j += 1;
            j;
        }
    }
    if req_comp != 0 && req_comp != target {
        out = stbi__convert_format(out, target, req_comp, (*s).img_x, (*s).img_y);
        if out.is_null() {
            return out as *mut libc::c_void;
        }
    }
    *x = (*s).img_x as libc::c_int;
    *y = (*s).img_y as libc::c_int;
    if !comp.is_null() {
        *comp = (*s).img_n;
    }
    return out as *mut libc::c_void;
}
unsafe extern "C" fn stbi__psd_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = (stbi__get32be(s)
        == 0x38425053 as libc::c_int as libc::c_uint) as libc::c_int;
    stbi__rewind(s);
    return r;
}
unsafe extern "C" fn stbi__psd_decode_rle(
    mut s: *mut stbi__context,
    mut p: *mut stbi_uc,
    mut pixelCount: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut nleft: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    count = 0 as libc::c_int;
    loop {
        nleft = pixelCount - count;
        if !(nleft > 0 as libc::c_int) {
            break;
        }
        len = stbi__get8(s) as libc::c_int;
        if !(len == 128 as libc::c_int) {
            if len < 128 as libc::c_int {
                len += 1;
                len;
                if len > nleft {
                    return 0 as libc::c_int;
                }
                count += len;
                while len != 0 {
                    *p = stbi__get8(s);
                    p = p.offset(4 as libc::c_int as isize);
                    len -= 1;
                    len;
                }
            } else if len > 128 as libc::c_int {
                let mut val: stbi_uc = 0;
                len = 257 as libc::c_int - len;
                if len > nleft {
                    return 0 as libc::c_int;
                }
                val = stbi__get8(s);
                count += len;
                while len != 0 {
                    *p = val;
                    p = p.offset(4 as libc::c_int as isize);
                    len -= 1;
                    len;
                }
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__compute_y_16(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> stbi__uint16 {
    return (r * 77 as libc::c_int + g * 150 as libc::c_int + 29 as libc::c_int * b
        >> 8 as libc::c_int) as stbi__uint16;
}
unsafe extern "C" fn stbi__convert_format16(
    mut data: *mut stbi__uint16,
    mut img_n: libc::c_int,
    mut req_comp: libc::c_int,
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> *mut stbi__uint16 {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut good: *mut stbi__uint16 = 0 as *mut stbi__uint16;
    if req_comp == img_n {
        return data;
    }
    if req_comp >= 1 as libc::c_int && req_comp <= 4 as libc::c_int {} else {
        __assert_fail(
            b"req_comp >= 1 && req_comp <= 4\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            1798 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"stbi__uint16 *stbi__convert_format16(stbi__uint16 *, int, int, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    };
    good = stbi__malloc(
        (req_comp as libc::c_uint)
            .wrapping_mul(x)
            .wrapping_mul(y)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
    ) as *mut stbi__uint16;
    if good.is_null() {
        stbi_free(data as *mut libc::c_void);
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut stbi__uint16;
    }
    j = 0 as libc::c_int;
    while j < y as libc::c_int {
        let mut src: *mut stbi__uint16 = data
            .offset(
                (j as libc::c_uint).wrapping_mul(x).wrapping_mul(img_n as libc::c_uint)
                    as isize,
            );
        let mut dest: *mut stbi__uint16 = good
            .offset(
                (j as libc::c_uint)
                    .wrapping_mul(x)
                    .wrapping_mul(req_comp as libc::c_uint) as isize,
            );
        match img_n * 8 as libc::c_int + req_comp {
            10 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = 0xffff as libc::c_int as stbi__uint16;
                    i -= 1;
                    i;
                    src = src.offset(1 as libc::c_int as isize);
                    dest = dest.offset(2 as libc::c_int as isize);
                }
            }
            11 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh52 = *dest.offset(2 as libc::c_int as isize);
                    *fresh52 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh53 = *dest.offset(1 as libc::c_int as isize);
                    *fresh53 = *fresh52;
                    *dest.offset(0 as libc::c_int as isize) = *fresh53;
                    i -= 1;
                    i;
                    src = src.offset(1 as libc::c_int as isize);
                    dest = dest.offset(3 as libc::c_int as isize);
                }
            }
            12 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh54 = *dest.offset(2 as libc::c_int as isize);
                    *fresh54 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh55 = *dest.offset(1 as libc::c_int as isize);
                    *fresh55 = *fresh54;
                    *dest.offset(0 as libc::c_int as isize) = *fresh55;
                    *dest
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 0xffff as libc::c_int as stbi__uint16;
                    i -= 1;
                    i;
                    src = src.offset(1 as libc::c_int as isize);
                    dest = dest.offset(4 as libc::c_int as isize);
                }
            }
            17 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(2 as libc::c_int as isize);
                    dest = dest.offset(1 as libc::c_int as isize);
                }
            }
            19 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh56 = *dest.offset(2 as libc::c_int as isize);
                    *fresh56 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh57 = *dest.offset(1 as libc::c_int as isize);
                    *fresh57 = *fresh56;
                    *dest.offset(0 as libc::c_int as isize) = *fresh57;
                    i -= 1;
                    i;
                    src = src.offset(2 as libc::c_int as isize);
                    dest = dest.offset(3 as libc::c_int as isize);
                }
            }
            20 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh58 = *dest.offset(2 as libc::c_int as isize);
                    *fresh58 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh59 = *dest.offset(1 as libc::c_int as isize);
                    *fresh59 = *fresh58;
                    *dest.offset(0 as libc::c_int as isize) = *fresh59;
                    *dest
                        .offset(
                            3 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(2 as libc::c_int as isize);
                    dest = dest.offset(4 as libc::c_int as isize);
                }
            }
            28 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    *dest
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                    *dest
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 0xffff as libc::c_int as stbi__uint16;
                    i -= 1;
                    i;
                    src = src.offset(3 as libc::c_int as isize);
                    dest = dest.offset(4 as libc::c_int as isize);
                }
            }
            25 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y_16(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    i -= 1;
                    i;
                    src = src.offset(3 as libc::c_int as isize);
                    dest = dest.offset(1 as libc::c_int as isize);
                }
            }
            26 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y_16(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = 0xffff as libc::c_int as stbi__uint16;
                    i -= 1;
                    i;
                    src = src.offset(3 as libc::c_int as isize);
                    dest = dest.offset(2 as libc::c_int as isize);
                }
            }
            33 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y_16(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    i -= 1;
                    i;
                    src = src.offset(4 as libc::c_int as isize);
                    dest = dest.offset(1 as libc::c_int as isize);
                }
            }
            34 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y_16(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(3 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(4 as libc::c_int as isize);
                    dest = dest.offset(2 as libc::c_int as isize);
                }
            }
            35 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    *dest
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(4 as libc::c_int as isize);
                    dest = dest.offset(3 as libc::c_int as isize);
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {} else {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                        1827 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 91],
                            &[libc::c_char; 91],
                        >(
                            b"stbi__uint16 *stbi__convert_format16(stbi__uint16 *, int, int, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                stbi_free(data as *mut libc::c_void);
                stbi_free(good as *mut libc::c_void);
                return (if stbi__err(
                    b"Unsupported format conversion\0" as *const u8
                        as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut stbi__uint16;
            }
        }
        j += 1;
        j;
    }
    stbi_free(data as *mut libc::c_void);
    return good;
}
unsafe extern "C" fn stbi__psd_load(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
    mut bpc: libc::c_int,
) -> *mut libc::c_void {
    let mut pixelCount: libc::c_int = 0;
    let mut channelCount: libc::c_int = 0;
    let mut compression: libc::c_int = 0;
    let mut channel: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut bitdepth: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut out: *mut stbi_uc = 0 as *mut stbi_uc;
    if stbi__get32be(s) != 0x38425053 as libc::c_int as libc::c_uint {
        return (if stbi__err(b"Corrupt PSD image\0" as *const u8 as *const libc::c_char)
            != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if stbi__get16be(s) != 1 as libc::c_int {
        return (if stbi__err(
            b"Unsupported version of PSD image\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    stbi__skip(s, 6 as libc::c_int);
    channelCount = stbi__get16be(s);
    if channelCount < 0 as libc::c_int || channelCount > 16 as libc::c_int {
        return (if stbi__err(
            b"Unsupported number of channels in PSD image\0" as *const u8
                as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    h = stbi__get32be(s) as libc::c_int;
    w = stbi__get32be(s) as libc::c_int;
    if h > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if w > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    bitdepth = stbi__get16be(s);
    if bitdepth != 8 as libc::c_int && bitdepth != 16 as libc::c_int {
        return (if stbi__err(
            b"PSD bit depth is not 8 or 16 bit\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if stbi__get16be(s) != 3 as libc::c_int {
        return (if stbi__err(
            b"PSD is not in RGB color format\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    stbi__skip(s, stbi__get32be(s) as libc::c_int);
    stbi__skip(s, stbi__get32be(s) as libc::c_int);
    stbi__skip(s, stbi__get32be(s) as libc::c_int);
    compression = stbi__get16be(s);
    if compression > 1 as libc::c_int {
        return (if stbi__err(
            b"PSD has an unknown compression format\0" as *const u8
                as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if stbi__mad3sizes_valid(4 as libc::c_int, w, h, 0 as libc::c_int) == 0 {
        return (if stbi__err(b"Corrupt PSD\0" as *const u8 as *const libc::c_char) != 0 {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if compression == 0 && bitdepth == 16 as libc::c_int && bpc == 16 as libc::c_int {
        out = stbi__malloc_mad3(8 as libc::c_int, w, h, 0 as libc::c_int)
            as *mut stbi_uc;
        (*ri).bits_per_channel = 16 as libc::c_int;
    } else {
        out = stbi__malloc((4 as libc::c_int * w * h) as size_t) as *mut stbi_uc;
    }
    if out.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    pixelCount = w * h;
    if compression != 0 {
        stbi__skip(s, h * channelCount * 2 as libc::c_int);
        channel = 0 as libc::c_int;
        while channel < 4 as libc::c_int {
            let mut p: *mut stbi_uc = 0 as *mut stbi_uc;
            p = out.offset(channel as isize);
            if channel >= channelCount {
                i = 0 as libc::c_int;
                while i < pixelCount {
                    *p = (if channel == 3 as libc::c_int {
                        255 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as stbi_uc;
                    i += 1;
                    i;
                    p = p.offset(4 as libc::c_int as isize);
                }
            } else if stbi__psd_decode_rle(s, p, pixelCount) == 0 {
                stbi_free(out as *mut libc::c_void);
                return (if stbi__err(
                    b"bad RLE data\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
            channel += 1;
            channel;
        }
    } else {
        channel = 0 as libc::c_int;
        while channel < 4 as libc::c_int {
            if channel >= channelCount {
                if bitdepth == 16 as libc::c_int && bpc == 16 as libc::c_int {
                    let mut q: *mut stbi__uint16 = (out as *mut stbi__uint16)
                        .offset(channel as isize);
                    let mut val: stbi__uint16 = (if channel == 3 as libc::c_int {
                        65535 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as stbi__uint16;
                    i = 0 as libc::c_int;
                    while i < pixelCount {
                        *q = val;
                        i += 1;
                        i;
                        q = q.offset(4 as libc::c_int as isize);
                    }
                } else {
                    let mut p_0: *mut stbi_uc = out.offset(channel as isize);
                    let mut val_0: stbi_uc = (if channel == 3 as libc::c_int {
                        255 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as stbi_uc;
                    i = 0 as libc::c_int;
                    while i < pixelCount {
                        *p_0 = val_0;
                        i += 1;
                        i;
                        p_0 = p_0.offset(4 as libc::c_int as isize);
                    }
                }
            } else if (*ri).bits_per_channel == 16 as libc::c_int {
                let mut q_0: *mut stbi__uint16 = (out as *mut stbi__uint16)
                    .offset(channel as isize);
                i = 0 as libc::c_int;
                while i < pixelCount {
                    *q_0 = stbi__get16be(s) as stbi__uint16;
                    i += 1;
                    i;
                    q_0 = q_0.offset(4 as libc::c_int as isize);
                }
            } else {
                let mut p_1: *mut stbi_uc = out.offset(channel as isize);
                if bitdepth == 16 as libc::c_int {
                    i = 0 as libc::c_int;
                    while i < pixelCount {
                        *p_1 = (stbi__get16be(s) >> 8 as libc::c_int) as stbi_uc;
                        i += 1;
                        i;
                        p_1 = p_1.offset(4 as libc::c_int as isize);
                    }
                } else {
                    i = 0 as libc::c_int;
                    while i < pixelCount {
                        *p_1 = stbi__get8(s);
                        i += 1;
                        i;
                        p_1 = p_1.offset(4 as libc::c_int as isize);
                    }
                }
            }
            channel += 1;
            channel;
        }
    }
    if channelCount >= 4 as libc::c_int {
        if (*ri).bits_per_channel == 16 as libc::c_int {
            i = 0 as libc::c_int;
            while i < w * h {
                let mut pixel: *mut stbi__uint16 = (out as *mut stbi__uint16)
                    .offset((4 as libc::c_int * i) as isize);
                if *pixel.offset(3 as libc::c_int as isize) as libc::c_int
                    != 0 as libc::c_int
                    && *pixel.offset(3 as libc::c_int as isize) as libc::c_int
                        != 65535 as libc::c_int
                {
                    let mut a: libc::c_float = *pixel.offset(3 as libc::c_int as isize)
                        as libc::c_int as libc::c_float / 65535.0f32;
                    let mut ra: libc::c_float = 1.0f32 / a;
                    let mut inv_a: libc::c_float = 65535.0f32
                        * (1 as libc::c_int as libc::c_float - ra);
                    *pixel
                        .offset(
                            0 as libc::c_int as isize,
                        ) = (*pixel.offset(0 as libc::c_int as isize) as libc::c_int
                        as libc::c_float * ra + inv_a) as stbi__uint16;
                    *pixel
                        .offset(
                            1 as libc::c_int as isize,
                        ) = (*pixel.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float * ra + inv_a) as stbi__uint16;
                    *pixel
                        .offset(
                            2 as libc::c_int as isize,
                        ) = (*pixel.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float * ra + inv_a) as stbi__uint16;
                }
                i += 1;
                i;
            }
        } else {
            i = 0 as libc::c_int;
            while i < w * h {
                let mut pixel_0: *mut libc::c_uchar = out
                    .offset((4 as libc::c_int * i) as isize);
                if *pixel_0.offset(3 as libc::c_int as isize) as libc::c_int
                    != 0 as libc::c_int
                    && *pixel_0.offset(3 as libc::c_int as isize) as libc::c_int
                        != 255 as libc::c_int
                {
                    let mut a_0: libc::c_float = *pixel_0
                        .offset(3 as libc::c_int as isize) as libc::c_int
                        as libc::c_float / 255.0f32;
                    let mut ra_0: libc::c_float = 1.0f32 / a_0;
                    let mut inv_a_0: libc::c_float = 255.0f32
                        * (1 as libc::c_int as libc::c_float - ra_0);
                    *pixel_0
                        .offset(
                            0 as libc::c_int as isize,
                        ) = (*pixel_0.offset(0 as libc::c_int as isize) as libc::c_int
                        as libc::c_float * ra_0 + inv_a_0) as libc::c_uchar;
                    *pixel_0
                        .offset(
                            1 as libc::c_int as isize,
                        ) = (*pixel_0.offset(1 as libc::c_int as isize) as libc::c_int
                        as libc::c_float * ra_0 + inv_a_0) as libc::c_uchar;
                    *pixel_0
                        .offset(
                            2 as libc::c_int as isize,
                        ) = (*pixel_0.offset(2 as libc::c_int as isize) as libc::c_int
                        as libc::c_float * ra_0 + inv_a_0) as libc::c_uchar;
                }
                i += 1;
                i;
            }
        }
    }
    if req_comp != 0 && req_comp != 4 as libc::c_int {
        if (*ri).bits_per_channel == 16 as libc::c_int {
            out = stbi__convert_format16(
                out as *mut stbi__uint16,
                4 as libc::c_int,
                req_comp,
                w as libc::c_uint,
                h as libc::c_uint,
            ) as *mut stbi_uc;
        } else {
            out = stbi__convert_format(
                out,
                4 as libc::c_int,
                req_comp,
                w as libc::c_uint,
                h as libc::c_uint,
            );
        }
        if out.is_null() {
            return out as *mut libc::c_void;
        }
    }
    if !comp.is_null() {
        *comp = 4 as libc::c_int;
    }
    *y = h;
    *x = w;
    return out as *mut libc::c_void;
}
unsafe extern "C" fn stbi__pic_is4(
    mut s: *mut stbi__context,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if stbi__get8(s) as libc::c_int
            != *str.offset(i as isize) as stbi_uc as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__pic_test_core(mut s: *mut stbi__context) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if stbi__pic_is4(s, b"S\x80\xF64\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 84 as libc::c_int {
        stbi__get8(s);
        i += 1;
        i;
    }
    if stbi__pic_is4(s, b"PICT\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__pic_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = stbi__pic_test_core(s);
    stbi__rewind(s);
    return r;
}
unsafe extern "C" fn stbi__get32be(mut s: *mut stbi__context) -> stbi__uint32 {
    let mut z: stbi__uint32 = stbi__get16be(s) as stbi__uint32;
    return (z << 16 as libc::c_int).wrapping_add(stbi__get16be(s) as libc::c_uint);
}
unsafe extern "C" fn stbi__copyval(
    mut channel: libc::c_int,
    mut dest: *mut stbi_uc,
    mut src: *const stbi_uc,
) {
    let mut mask: libc::c_int = 0x80 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if channel & mask != 0 {
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
        i += 1;
        i;
        mask >>= 1 as libc::c_int;
    }
}
unsafe extern "C" fn stbi__readval(
    mut s: *mut stbi__context,
    mut channel: libc::c_int,
    mut dest: *mut stbi_uc,
) -> *mut stbi_uc {
    let mut mask: libc::c_int = 0x80 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if channel & mask != 0 {
            if stbi__at_eof(s) != 0 {
                return (if stbi__err(
                    b"PIC file too short\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar;
            }
            *dest.offset(i as isize) = stbi__get8(s);
        }
        i += 1;
        i;
        mask >>= 1 as libc::c_int;
    }
    return dest;
}
unsafe extern "C" fn stbi__pic_load_core(
    mut s: *mut stbi__context,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut comp: *mut libc::c_int,
    mut result: *mut stbi_uc,
) -> *mut stbi_uc {
    let mut act_comp: libc::c_int = 0 as libc::c_int;
    let mut num_packets: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0;
    let mut chained: libc::c_int = 0;
    let mut packets: [stbi__pic_packet; 10] = [stbi__pic_packet {
        size: 0,
        type_0: 0,
        channel: 0,
    }; 10];
    loop {
        let mut packet: *mut stbi__pic_packet = 0 as *mut stbi__pic_packet;
        if num_packets as libc::c_ulong
            == (::core::mem::size_of::<[stbi__pic_packet; 10]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<stbi__pic_packet>() as libc::c_ulong,
                )
        {
            return (if stbi__err(
                b"too many packets\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar;
        }
        let fresh60 = num_packets;
        num_packets = num_packets + 1;
        packet = &mut *packets.as_mut_ptr().offset(fresh60 as isize)
            as *mut stbi__pic_packet;
        chained = stbi__get8(s) as libc::c_int;
        (*packet).size = stbi__get8(s);
        (*packet).type_0 = stbi__get8(s);
        (*packet).channel = stbi__get8(s);
        act_comp |= (*packet).channel as libc::c_int;
        if stbi__at_eof(s) != 0 {
            return (if stbi__err(
                b"file too short (reading packets)\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar;
        }
        if (*packet).size as libc::c_int != 8 as libc::c_int {
            return (if stbi__err(
                b"packet isn't 8bpp\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar;
        }
        if !(chained != 0) {
            break;
        }
    }
    *comp = if act_comp & 0x10 as libc::c_int != 0 {
        4 as libc::c_int
    } else {
        3 as libc::c_int
    };
    y = 0 as libc::c_int;
    while y < height {
        let mut packet_idx: libc::c_int = 0;
        packet_idx = 0 as libc::c_int;
        while packet_idx < num_packets {
            let mut packet_0: *mut stbi__pic_packet = &mut *packets
                .as_mut_ptr()
                .offset(packet_idx as isize) as *mut stbi__pic_packet;
            let mut dest: *mut stbi_uc = result
                .offset((y * width * 4 as libc::c_int) as isize);
            match (*packet_0).type_0 as libc::c_int {
                0 => {
                    let mut x: libc::c_int = 0;
                    x = 0 as libc::c_int;
                    while x < width {
                        if (stbi__readval(s, (*packet_0).channel as libc::c_int, dest))
                            .is_null()
                        {
                            return 0 as *mut stbi_uc;
                        }
                        x += 1;
                        x;
                        dest = dest.offset(4 as libc::c_int as isize);
                    }
                }
                1 => {
                    let mut left: libc::c_int = width;
                    let mut i: libc::c_int = 0;
                    while left > 0 as libc::c_int {
                        let mut count: stbi_uc = 0;
                        let mut value: [stbi_uc; 4] = [0; 4];
                        count = stbi__get8(s);
                        if stbi__at_eof(s) != 0 {
                            return (if stbi__err(
                                b"file too short (pure read count)\0" as *const u8
                                    as *const libc::c_char,
                            ) != 0
                            {
                                0 as *mut libc::c_void
                            } else {
                                0 as *mut libc::c_void
                            }) as size_t as *mut libc::c_uchar;
                        }
                        if count as libc::c_int > left {
                            count = left as stbi_uc;
                        }
                        if (stbi__readval(
                            s,
                            (*packet_0).channel as libc::c_int,
                            value.as_mut_ptr(),
                        ))
                            .is_null()
                        {
                            return 0 as *mut stbi_uc;
                        }
                        i = 0 as libc::c_int;
                        while i < count as libc::c_int {
                            stbi__copyval(
                                (*packet_0).channel as libc::c_int,
                                dest,
                                value.as_mut_ptr(),
                            );
                            i += 1;
                            i;
                            dest = dest.offset(4 as libc::c_int as isize);
                        }
                        left -= count as libc::c_int;
                    }
                }
                2 => {
                    let mut left_0: libc::c_int = width;
                    while left_0 > 0 as libc::c_int {
                        let mut count_0: libc::c_int = stbi__get8(s) as libc::c_int;
                        let mut i_0: libc::c_int = 0;
                        if stbi__at_eof(s) != 0 {
                            return (if stbi__err(
                                b"file too short (mixed read count)\0" as *const u8
                                    as *const libc::c_char,
                            ) != 0
                            {
                                0 as *mut libc::c_void
                            } else {
                                0 as *mut libc::c_void
                            }) as size_t as *mut libc::c_uchar;
                        }
                        if count_0 >= 128 as libc::c_int {
                            let mut value_0: [stbi_uc; 4] = [0; 4];
                            if count_0 == 128 as libc::c_int {
                                count_0 = stbi__get16be(s);
                            } else {
                                count_0 -= 127 as libc::c_int;
                            }
                            if count_0 > left_0 {
                                return (if stbi__err(
                                    b"scanline overrun\0" as *const u8 as *const libc::c_char,
                                ) != 0
                                {
                                    0 as *mut libc::c_void
                                } else {
                                    0 as *mut libc::c_void
                                }) as size_t as *mut libc::c_uchar;
                            }
                            if (stbi__readval(
                                s,
                                (*packet_0).channel as libc::c_int,
                                value_0.as_mut_ptr(),
                            ))
                                .is_null()
                            {
                                return 0 as *mut stbi_uc;
                            }
                            i_0 = 0 as libc::c_int;
                            while i_0 < count_0 {
                                stbi__copyval(
                                    (*packet_0).channel as libc::c_int,
                                    dest,
                                    value_0.as_mut_ptr(),
                                );
                                i_0 += 1;
                                i_0;
                                dest = dest.offset(4 as libc::c_int as isize);
                            }
                        } else {
                            count_0 += 1;
                            count_0;
                            if count_0 > left_0 {
                                return (if stbi__err(
                                    b"scanline overrun\0" as *const u8 as *const libc::c_char,
                                ) != 0
                                {
                                    0 as *mut libc::c_void
                                } else {
                                    0 as *mut libc::c_void
                                }) as size_t as *mut libc::c_uchar;
                            }
                            i_0 = 0 as libc::c_int;
                            while i_0 < count_0 {
                                if (stbi__readval(
                                    s,
                                    (*packet_0).channel as libc::c_int,
                                    dest,
                                ))
                                    .is_null()
                                {
                                    return 0 as *mut stbi_uc;
                                }
                                i_0 += 1;
                                i_0;
                                dest = dest.offset(4 as libc::c_int as isize);
                            }
                        }
                        left_0 -= count_0;
                    }
                }
                _ => {
                    return (if stbi__err(
                        b"packet has bad compression type\0" as *const u8
                            as *const libc::c_char,
                    ) != 0
                    {
                        0 as *mut libc::c_void
                    } else {
                        0 as *mut libc::c_void
                    }) as size_t as *mut libc::c_uchar;
                }
            }
            packet_idx += 1;
            packet_idx;
        }
        y += 1;
        y;
    }
    return result;
}
unsafe extern "C" fn stbi__pic_load(
    mut s: *mut stbi__context,
    mut px: *mut libc::c_int,
    mut py: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_void {
    let mut result: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut internal_comp: libc::c_int = 0;
    if comp.is_null() {
        comp = &mut internal_comp;
    }
    i = 0 as libc::c_int;
    while i < 92 as libc::c_int {
        stbi__get8(s);
        i += 1;
        i;
    }
    x = stbi__get16be(s);
    y = stbi__get16be(s);
    if y > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if x > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if stbi__at_eof(s) != 0 {
        return (if stbi__err(
            b"file too short (pic header)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if stbi__mad3sizes_valid(x, y, 4 as libc::c_int, 0 as libc::c_int) == 0 {
        return (if stbi__err(
            b"PIC image too large to decode\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    stbi__get32be(s);
    stbi__get16be(s);
    stbi__get16be(s);
    result = stbi__malloc_mad3(x, y, 4 as libc::c_int, 0 as libc::c_int) as *mut stbi_uc;
    if result.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    memset(
        result as *mut libc::c_void,
        0xff as libc::c_int,
        (x * y * 4 as libc::c_int) as libc::c_ulong,
    );
    if (stbi__pic_load_core(s, x, y, comp, result)).is_null() {
        stbi_free(result as *mut libc::c_void);
        result = 0 as *mut stbi_uc;
    }
    *px = x;
    *py = y;
    if req_comp == 0 as libc::c_int {
        req_comp = *comp;
    }
    result = stbi__convert_format(
        result,
        4 as libc::c_int,
        req_comp,
        x as libc::c_uint,
        y as libc::c_uint,
    );
    return result as *mut libc::c_void;
}
unsafe extern "C" fn stbi__jpeg_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut j: *mut stbi__jpeg = stbi__malloc(
        ::core::mem::size_of::<stbi__jpeg>() as libc::c_ulong,
    ) as *mut stbi__jpeg;
    if j.is_null() {
        return stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char);
    }
    (*j).s = s;
    stbi__setup_jpeg(j);
    r = stbi__decode_jpeg_header(j, STBI__SCAN_type as libc::c_int);
    stbi__rewind(s);
    stbi_free(j as *mut libc::c_void);
    return r;
}
unsafe extern "C" fn stbi__clamp(mut x: libc::c_int) -> stbi_uc {
    if x as libc::c_uint > 255 as libc::c_int as libc::c_uint {
        if x < 0 as libc::c_int {
            return 0 as libc::c_int as stbi_uc;
        }
        if x > 255 as libc::c_int {
            return 255 as libc::c_int as stbi_uc;
        }
    }
    return x as stbi_uc;
}
unsafe extern "C" fn stbi__idct_block(
    mut out: *mut stbi_uc,
    mut out_stride: libc::c_int,
    mut data: *mut libc::c_short,
) {
    let mut i: libc::c_int = 0;
    let mut val: [libc::c_int; 64] = [0; 64];
    let mut v: *mut libc::c_int = val.as_mut_ptr();
    let mut o: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut d: *mut libc::c_short = data;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if *d.offset(8 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *d.offset(16 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *d.offset(24 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *d.offset(32 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *d.offset(40 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *d.offset(48 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            && *d.offset(56 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
        {
            let mut dcterm: libc::c_int = *d.offset(0 as libc::c_int as isize)
                as libc::c_int * 4 as libc::c_int;
            let ref mut fresh61 = *v.offset(56 as libc::c_int as isize);
            *fresh61 = dcterm;
            let ref mut fresh62 = *v.offset(48 as libc::c_int as isize);
            *fresh62 = *fresh61;
            let ref mut fresh63 = *v.offset(40 as libc::c_int as isize);
            *fresh63 = *fresh62;
            let ref mut fresh64 = *v.offset(32 as libc::c_int as isize);
            *fresh64 = *fresh63;
            let ref mut fresh65 = *v.offset(24 as libc::c_int as isize);
            *fresh65 = *fresh64;
            let ref mut fresh66 = *v.offset(16 as libc::c_int as isize);
            *fresh66 = *fresh65;
            let ref mut fresh67 = *v.offset(8 as libc::c_int as isize);
            *fresh67 = *fresh66;
            *v.offset(0 as libc::c_int as isize) = *fresh67;
        } else {
            let mut t0: libc::c_int = 0;
            let mut t1: libc::c_int = 0;
            let mut t2: libc::c_int = 0;
            let mut t3: libc::c_int = 0;
            let mut p1: libc::c_int = 0;
            let mut p2: libc::c_int = 0;
            let mut p3: libc::c_int = 0;
            let mut p4: libc::c_int = 0;
            let mut p5: libc::c_int = 0;
            let mut x0: libc::c_int = 0;
            let mut x1: libc::c_int = 0;
            let mut x2: libc::c_int = 0;
            let mut x3: libc::c_int = 0;
            p2 = *d.offset(16 as libc::c_int as isize) as libc::c_int;
            p3 = *d.offset(48 as libc::c_int as isize) as libc::c_int;
            p1 = (p2 + p3)
                * ((0.5411961f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            t2 = p1
                + p3
                    * ((-1.847759065f32 * 4096 as libc::c_int as libc::c_float)
                        as libc::c_double + 0.5f64) as libc::c_int;
            t3 = p1
                + p2
                    * ((0.765366865f32 * 4096 as libc::c_int as libc::c_float)
                        as libc::c_double + 0.5f64) as libc::c_int;
            p2 = *d.offset(0 as libc::c_int as isize) as libc::c_int;
            p3 = *d.offset(32 as libc::c_int as isize) as libc::c_int;
            t0 = (p2 + p3) * 4096 as libc::c_int;
            t1 = (p2 - p3) * 4096 as libc::c_int;
            x0 = t0 + t3;
            x3 = t0 - t3;
            x1 = t1 + t2;
            x2 = t1 - t2;
            t0 = *d.offset(56 as libc::c_int as isize) as libc::c_int;
            t1 = *d.offset(40 as libc::c_int as isize) as libc::c_int;
            t2 = *d.offset(24 as libc::c_int as isize) as libc::c_int;
            t3 = *d.offset(8 as libc::c_int as isize) as libc::c_int;
            p3 = t0 + t2;
            p4 = t1 + t3;
            p1 = t0 + t3;
            p2 = t1 + t2;
            p5 = (p3 + p4)
                * ((1.175875602f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            t0 = t0
                * ((0.298631336f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            t1 = t1
                * ((2.053119869f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            t2 = t2
                * ((3.072711026f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            t3 = t3
                * ((1.501321110f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            p1 = p5
                + p1
                    * ((-0.899976223f32 * 4096 as libc::c_int as libc::c_float)
                        as libc::c_double + 0.5f64) as libc::c_int;
            p2 = p5
                + p2
                    * ((-2.562915447f32 * 4096 as libc::c_int as libc::c_float)
                        as libc::c_double + 0.5f64) as libc::c_int;
            p3 = p3
                * ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            p4 = p4
                * ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
            t3 += p1 + p4;
            t2 += p2 + p3;
            t1 += p2 + p4;
            t0 += p1 + p3;
            x0 += 512 as libc::c_int;
            x1 += 512 as libc::c_int;
            x2 += 512 as libc::c_int;
            x3 += 512 as libc::c_int;
            *v.offset(0 as libc::c_int as isize) = x0 + t3 >> 10 as libc::c_int;
            *v.offset(56 as libc::c_int as isize) = x0 - t3 >> 10 as libc::c_int;
            *v.offset(8 as libc::c_int as isize) = x1 + t2 >> 10 as libc::c_int;
            *v.offset(48 as libc::c_int as isize) = x1 - t2 >> 10 as libc::c_int;
            *v.offset(16 as libc::c_int as isize) = x2 + t1 >> 10 as libc::c_int;
            *v.offset(40 as libc::c_int as isize) = x2 - t1 >> 10 as libc::c_int;
            *v.offset(24 as libc::c_int as isize) = x3 + t0 >> 10 as libc::c_int;
            *v.offset(32 as libc::c_int as isize) = x3 - t0 >> 10 as libc::c_int;
        }
        i += 1;
        i;
        d = d.offset(1);
        d;
        v = v.offset(1);
        v;
    }
    i = 0 as libc::c_int;
    v = val.as_mut_ptr();
    o = out;
    while i < 8 as libc::c_int {
        let mut t0_0: libc::c_int = 0;
        let mut t1_0: libc::c_int = 0;
        let mut t2_0: libc::c_int = 0;
        let mut t3_0: libc::c_int = 0;
        let mut p1_0: libc::c_int = 0;
        let mut p2_0: libc::c_int = 0;
        let mut p3_0: libc::c_int = 0;
        let mut p4_0: libc::c_int = 0;
        let mut p5_0: libc::c_int = 0;
        let mut x0_0: libc::c_int = 0;
        let mut x1_0: libc::c_int = 0;
        let mut x2_0: libc::c_int = 0;
        let mut x3_0: libc::c_int = 0;
        p2_0 = *v.offset(2 as libc::c_int as isize);
        p3_0 = *v.offset(6 as libc::c_int as isize);
        p1_0 = (p2_0 + p3_0)
            * ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        t2_0 = p1_0
            + p3_0
                * ((-1.847759065f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
        t3_0 = p1_0
            + p2_0
                * ((0.765366865f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
        p2_0 = *v.offset(0 as libc::c_int as isize);
        p3_0 = *v.offset(4 as libc::c_int as isize);
        t0_0 = (p2_0 + p3_0) * 4096 as libc::c_int;
        t1_0 = (p2_0 - p3_0) * 4096 as libc::c_int;
        x0_0 = t0_0 + t3_0;
        x3_0 = t0_0 - t3_0;
        x1_0 = t1_0 + t2_0;
        x2_0 = t1_0 - t2_0;
        t0_0 = *v.offset(7 as libc::c_int as isize);
        t1_0 = *v.offset(5 as libc::c_int as isize);
        t2_0 = *v.offset(3 as libc::c_int as isize);
        t3_0 = *v.offset(1 as libc::c_int as isize);
        p3_0 = t0_0 + t2_0;
        p4_0 = t1_0 + t3_0;
        p1_0 = t0_0 + t3_0;
        p2_0 = t1_0 + t2_0;
        p5_0 = (p3_0 + p4_0)
            * ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        t0_0 = t0_0
            * ((0.298631336f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        t1_0 = t1_0
            * ((2.053119869f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        t2_0 = t2_0
            * ((3.072711026f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        t3_0 = t3_0
            * ((1.501321110f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        p1_0 = p5_0
            + p1_0
                * ((-0.899976223f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
        p2_0 = p5_0
            + p2_0
                * ((-2.562915447f32 * 4096 as libc::c_int as libc::c_float)
                    as libc::c_double + 0.5f64) as libc::c_int;
        p3_0 = p3_0
            * ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        p4_0 = p4_0
            * ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int;
        t3_0 += p1_0 + p4_0;
        t2_0 += p2_0 + p3_0;
        t1_0 += p2_0 + p4_0;
        t0_0 += p1_0 + p3_0;
        x0_0 += 65536 as libc::c_int + ((128 as libc::c_int) << 17 as libc::c_int);
        x1_0 += 65536 as libc::c_int + ((128 as libc::c_int) << 17 as libc::c_int);
        x2_0 += 65536 as libc::c_int + ((128 as libc::c_int) << 17 as libc::c_int);
        x3_0 += 65536 as libc::c_int + ((128 as libc::c_int) << 17 as libc::c_int);
        *o
            .offset(
                0 as libc::c_int as isize,
            ) = stbi__clamp(x0_0 + t3_0 >> 17 as libc::c_int);
        *o
            .offset(
                7 as libc::c_int as isize,
            ) = stbi__clamp(x0_0 - t3_0 >> 17 as libc::c_int);
        *o
            .offset(
                1 as libc::c_int as isize,
            ) = stbi__clamp(x1_0 + t2_0 >> 17 as libc::c_int);
        *o
            .offset(
                6 as libc::c_int as isize,
            ) = stbi__clamp(x1_0 - t2_0 >> 17 as libc::c_int);
        *o
            .offset(
                2 as libc::c_int as isize,
            ) = stbi__clamp(x2_0 + t1_0 >> 17 as libc::c_int);
        *o
            .offset(
                5 as libc::c_int as isize,
            ) = stbi__clamp(x2_0 - t1_0 >> 17 as libc::c_int);
        *o
            .offset(
                3 as libc::c_int as isize,
            ) = stbi__clamp(x3_0 + t0_0 >> 17 as libc::c_int);
        *o
            .offset(
                4 as libc::c_int as isize,
            ) = stbi__clamp(x3_0 - t0_0 >> 17 as libc::c_int);
        i += 1;
        i;
        v = v.offset(8 as libc::c_int as isize);
        o = o.offset(out_stride as isize);
    }
}
unsafe extern "C" fn stbi__YCbCr_to_RGB_row(
    mut out: *mut stbi_uc,
    mut y: *const stbi_uc,
    mut pcb: *const stbi_uc,
    mut pcr: *const stbi_uc,
    mut count: libc::c_int,
    mut step: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        let mut y_fixed: libc::c_int = ((*y.offset(i as isize) as libc::c_int)
            << 20 as libc::c_int) + ((1 as libc::c_int) << 19 as libc::c_int);
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut cr: libc::c_int = *pcr.offset(i as isize) as libc::c_int
            - 128 as libc::c_int;
        let mut cb: libc::c_int = *pcb.offset(i as isize) as libc::c_int
            - 128 as libc::c_int;
        r = y_fixed
            + cr
                * (((1.40200f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                    << 8 as libc::c_int);
        g = ((y_fixed
            + cr
                * -(((0.71414f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                    << 8 as libc::c_int)) as libc::c_uint)
            .wrapping_add(
                (cb
                    * -(((0.34414f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                        << 8 as libc::c_int)) as libc::c_uint
                    & 0xffff0000 as libc::c_uint,
            ) as libc::c_int;
        b = y_fixed
            + cb
                * (((1.77200f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                    << 8 as libc::c_int);
        r >>= 20 as libc::c_int;
        g >>= 20 as libc::c_int;
        b >>= 20 as libc::c_int;
        if r as libc::c_uint > 255 as libc::c_int as libc::c_uint {
            if r < 0 as libc::c_int {
                r = 0 as libc::c_int;
            } else {
                r = 255 as libc::c_int;
            }
        }
        if g as libc::c_uint > 255 as libc::c_int as libc::c_uint {
            if g < 0 as libc::c_int {
                g = 0 as libc::c_int;
            } else {
                g = 255 as libc::c_int;
            }
        }
        if b as libc::c_uint > 255 as libc::c_int as libc::c_uint {
            if b < 0 as libc::c_int {
                b = 0 as libc::c_int;
            } else {
                b = 255 as libc::c_int;
            }
        }
        *out.offset(0 as libc::c_int as isize) = r as stbi_uc;
        *out.offset(1 as libc::c_int as isize) = g as stbi_uc;
        *out.offset(2 as libc::c_int as isize) = b as stbi_uc;
        *out.offset(3 as libc::c_int as isize) = 255 as libc::c_int as stbi_uc;
        out = out.offset(step as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn stbi__resample_row_hv_2(
    mut out: *mut stbi_uc,
    mut in_near: *mut stbi_uc,
    mut in_far: *mut stbi_uc,
    mut w: libc::c_int,
    mut hs: libc::c_int,
) -> *mut stbi_uc {
    let mut i: libc::c_int = 0;
    let mut t0: libc::c_int = 0;
    let mut t1: libc::c_int = 0;
    if w == 1 as libc::c_int {
        let ref mut fresh68 = *out.offset(1 as libc::c_int as isize);
        *fresh68 = (3 as libc::c_int
            * *in_near.offset(0 as libc::c_int as isize) as libc::c_int
            + *in_far.offset(0 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int
            >> 2 as libc::c_int) as stbi_uc;
        *out.offset(0 as libc::c_int as isize) = *fresh68;
        return out;
    }
    t1 = 3 as libc::c_int * *in_near.offset(0 as libc::c_int as isize) as libc::c_int
        + *in_far.offset(0 as libc::c_int as isize) as libc::c_int;
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = (t1 + 2 as libc::c_int >> 2 as libc::c_int) as stbi_uc;
    i = 1 as libc::c_int;
    while i < w {
        t0 = t1;
        t1 = 3 as libc::c_int * *in_near.offset(i as isize) as libc::c_int
            + *in_far.offset(i as isize) as libc::c_int;
        *out
            .offset(
                (i * 2 as libc::c_int - 1 as libc::c_int) as isize,
            ) = (3 as libc::c_int * t0 + t1 + 8 as libc::c_int >> 4 as libc::c_int)
            as stbi_uc;
        *out
            .offset(
                (i * 2 as libc::c_int) as isize,
            ) = (3 as libc::c_int * t1 + t0 + 8 as libc::c_int >> 4 as libc::c_int)
            as stbi_uc;
        i += 1;
        i;
    }
    *out
        .offset(
            (w * 2 as libc::c_int - 1 as libc::c_int) as isize,
        ) = (t1 + 2 as libc::c_int >> 2 as libc::c_int) as stbi_uc;
    return out;
}
unsafe extern "C" fn stbi__sse2_available() -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__idct_simd(
    mut out: *mut stbi_uc,
    mut out_stride: libc::c_int,
    mut data: *mut libc::c_short,
) {
    let mut row0: __m128i = _mm_setzero_si128();
    let mut row1: __m128i = _mm_setzero_si128();
    let mut row2: __m128i = _mm_setzero_si128();
    let mut row3: __m128i = _mm_setzero_si128();
    let mut row4: __m128i = _mm_setzero_si128();
    let mut row5: __m128i = _mm_setzero_si128();
    let mut row6: __m128i = _mm_setzero_si128();
    let mut row7: __m128i = _mm_setzero_si128();
    let mut tmp: __m128i = _mm_setzero_si128();
    let mut rot0_0: __m128i = _mm_setr_epi16(
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-1.847759065f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-1.847759065f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-1.847759065f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-1.847759065f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
    );
    let mut rot0_1: __m128i = _mm_setr_epi16(
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.765366865f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.765366865f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.765366865f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.765366865f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((0.5411961f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
    );
    let mut rot1_0: __m128i = _mm_setr_epi16(
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-0.899976223f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-0.899976223f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-0.899976223f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-0.899976223f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
    );
    let mut rot1_1: __m128i = _mm_setr_epi16(
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-2.562915447f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-2.562915447f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-2.562915447f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((1.175875602f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((-2.562915447f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
    );
    let mut rot2_0: __m128i = _mm_setr_epi16(
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.298631336f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.298631336f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.298631336f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((0.298631336f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
    );
    let mut rot2_1: __m128i = _mm_setr_epi16(
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((3.072711026f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((3.072711026f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((3.072711026f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-1.961570560f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((3.072711026f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
    );
    let mut rot3_0: __m128i = _mm_setr_epi16(
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((2.053119869f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((2.053119869f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((2.053119869f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((2.053119869f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
    );
    let mut rot3_1: __m128i = _mm_setr_epi16(
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((1.501321110f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((1.501321110f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((1.501321110f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
        ((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int as libc::c_short,
        (((-0.390180644f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
            + 0.5f64) as libc::c_int
            + ((1.501321110f32 * 4096 as libc::c_int as libc::c_float) as libc::c_double
                + 0.5f64) as libc::c_int) as libc::c_short,
    );
    let mut bias_0: __m128i = _mm_set1_epi32(512 as libc::c_int);
    let mut bias_1: __m128i = _mm_set1_epi32(
        65536 as libc::c_int + ((128 as libc::c_int) << 17 as libc::c_int),
    );
    row0 = _mm_load_si128(
        data.offset((0 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row1 = _mm_load_si128(
        data.offset((1 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row2 = _mm_load_si128(
        data.offset((2 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row3 = _mm_load_si128(
        data.offset((3 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row4 = _mm_load_si128(
        data.offset((4 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row5 = _mm_load_si128(
        data.offset((5 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row6 = _mm_load_si128(
        data.offset((6 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    row7 = _mm_load_si128(
        data.offset((7 as libc::c_int * 8 as libc::c_int) as isize) as *const __m128i,
    );
    let mut rot0_0lo: __m128i = _mm_unpacklo_epi16(row2, row6);
    let mut rot0_0hi: __m128i = _mm_unpackhi_epi16(row2, row6);
    let mut t2e_l: __m128i = _mm_madd_epi16(rot0_0lo, rot0_0);
    let mut t2e_h: __m128i = _mm_madd_epi16(rot0_0hi, rot0_0);
    let mut t3e_l: __m128i = _mm_madd_epi16(rot0_0lo, rot0_1);
    let mut t3e_h: __m128i = _mm_madd_epi16(rot0_0hi, rot0_1);
    let mut sum04: __m128i = _mm_add_epi16(row0, row4);
    let mut dif04: __m128i = _mm_sub_epi16(row0, row4);
    let mut t0e_l: __m128i = _mm_srai_epi32(
        _mm_unpacklo_epi16(_mm_setzero_si128(), sum04),
        4 as libc::c_int,
    );
    let mut t0e_h: __m128i = _mm_srai_epi32(
        _mm_unpackhi_epi16(_mm_setzero_si128(), sum04),
        4 as libc::c_int,
    );
    let mut t1e_l: __m128i = _mm_srai_epi32(
        _mm_unpacklo_epi16(_mm_setzero_si128(), dif04),
        4 as libc::c_int,
    );
    let mut t1e_h: __m128i = _mm_srai_epi32(
        _mm_unpackhi_epi16(_mm_setzero_si128(), dif04),
        4 as libc::c_int,
    );
    let mut x0_l: __m128i = _mm_add_epi32(t0e_l, t3e_l);
    let mut x0_h: __m128i = _mm_add_epi32(t0e_h, t3e_h);
    let mut x3_l: __m128i = _mm_sub_epi32(t0e_l, t3e_l);
    let mut x3_h: __m128i = _mm_sub_epi32(t0e_h, t3e_h);
    let mut x1_l: __m128i = _mm_add_epi32(t1e_l, t2e_l);
    let mut x1_h: __m128i = _mm_add_epi32(t1e_h, t2e_h);
    let mut x2_l: __m128i = _mm_sub_epi32(t1e_l, t2e_l);
    let mut x2_h: __m128i = _mm_sub_epi32(t1e_h, t2e_h);
    let mut rot2_0lo: __m128i = _mm_unpacklo_epi16(row7, row3);
    let mut rot2_0hi: __m128i = _mm_unpackhi_epi16(row7, row3);
    let mut y0o_l: __m128i = _mm_madd_epi16(rot2_0lo, rot2_0);
    let mut y0o_h: __m128i = _mm_madd_epi16(rot2_0hi, rot2_0);
    let mut y2o_l: __m128i = _mm_madd_epi16(rot2_0lo, rot2_1);
    let mut y2o_h: __m128i = _mm_madd_epi16(rot2_0hi, rot2_1);
    let mut rot3_0lo: __m128i = _mm_unpacklo_epi16(row5, row1);
    let mut rot3_0hi: __m128i = _mm_unpackhi_epi16(row5, row1);
    let mut y1o_l: __m128i = _mm_madd_epi16(rot3_0lo, rot3_0);
    let mut y1o_h: __m128i = _mm_madd_epi16(rot3_0hi, rot3_0);
    let mut y3o_l: __m128i = _mm_madd_epi16(rot3_0lo, rot3_1);
    let mut y3o_h: __m128i = _mm_madd_epi16(rot3_0hi, rot3_1);
    let mut sum17: __m128i = _mm_add_epi16(row1, row7);
    let mut sum35: __m128i = _mm_add_epi16(row3, row5);
    let mut rot1_0lo: __m128i = _mm_unpacklo_epi16(sum17, sum35);
    let mut rot1_0hi: __m128i = _mm_unpackhi_epi16(sum17, sum35);
    let mut y4o_l: __m128i = _mm_madd_epi16(rot1_0lo, rot1_0);
    let mut y4o_h: __m128i = _mm_madd_epi16(rot1_0hi, rot1_0);
    let mut y5o_l: __m128i = _mm_madd_epi16(rot1_0lo, rot1_1);
    let mut y5o_h: __m128i = _mm_madd_epi16(rot1_0hi, rot1_1);
    let mut x4_l: __m128i = _mm_add_epi32(y0o_l, y4o_l);
    let mut x4_h: __m128i = _mm_add_epi32(y0o_h, y4o_h);
    let mut x5_l: __m128i = _mm_add_epi32(y1o_l, y5o_l);
    let mut x5_h: __m128i = _mm_add_epi32(y1o_h, y5o_h);
    let mut x6_l: __m128i = _mm_add_epi32(y2o_l, y5o_l);
    let mut x6_h: __m128i = _mm_add_epi32(y2o_h, y5o_h);
    let mut x7_l: __m128i = _mm_add_epi32(y3o_l, y4o_l);
    let mut x7_h: __m128i = _mm_add_epi32(y3o_h, y4o_h);
    let mut abiased_l: __m128i = _mm_add_epi32(x0_l, bias_0);
    let mut abiased_h: __m128i = _mm_add_epi32(x0_h, bias_0);
    let mut sum_l: __m128i = _mm_add_epi32(abiased_l, x7_l);
    let mut sum_h: __m128i = _mm_add_epi32(abiased_h, x7_h);
    let mut dif_l: __m128i = _mm_sub_epi32(abiased_l, x7_l);
    let mut dif_h: __m128i = _mm_sub_epi32(abiased_h, x7_h);
    row0 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l, 10 as libc::c_int),
        _mm_srai_epi32(sum_h, 10 as libc::c_int),
    );
    row7 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l, 10 as libc::c_int),
        _mm_srai_epi32(dif_h, 10 as libc::c_int),
    );
    let mut abiased_l_0: __m128i = _mm_add_epi32(x1_l, bias_0);
    let mut abiased_h_0: __m128i = _mm_add_epi32(x1_h, bias_0);
    let mut sum_l_0: __m128i = _mm_add_epi32(abiased_l_0, x6_l);
    let mut sum_h_0: __m128i = _mm_add_epi32(abiased_h_0, x6_h);
    let mut dif_l_0: __m128i = _mm_sub_epi32(abiased_l_0, x6_l);
    let mut dif_h_0: __m128i = _mm_sub_epi32(abiased_h_0, x6_h);
    row1 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_0, 10 as libc::c_int),
        _mm_srai_epi32(sum_h_0, 10 as libc::c_int),
    );
    row6 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_0, 10 as libc::c_int),
        _mm_srai_epi32(dif_h_0, 10 as libc::c_int),
    );
    let mut abiased_l_1: __m128i = _mm_add_epi32(x2_l, bias_0);
    let mut abiased_h_1: __m128i = _mm_add_epi32(x2_h, bias_0);
    let mut sum_l_1: __m128i = _mm_add_epi32(abiased_l_1, x5_l);
    let mut sum_h_1: __m128i = _mm_add_epi32(abiased_h_1, x5_h);
    let mut dif_l_1: __m128i = _mm_sub_epi32(abiased_l_1, x5_l);
    let mut dif_h_1: __m128i = _mm_sub_epi32(abiased_h_1, x5_h);
    row2 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_1, 10 as libc::c_int),
        _mm_srai_epi32(sum_h_1, 10 as libc::c_int),
    );
    row5 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_1, 10 as libc::c_int),
        _mm_srai_epi32(dif_h_1, 10 as libc::c_int),
    );
    let mut abiased_l_2: __m128i = _mm_add_epi32(x3_l, bias_0);
    let mut abiased_h_2: __m128i = _mm_add_epi32(x3_h, bias_0);
    let mut sum_l_2: __m128i = _mm_add_epi32(abiased_l_2, x4_l);
    let mut sum_h_2: __m128i = _mm_add_epi32(abiased_h_2, x4_h);
    let mut dif_l_2: __m128i = _mm_sub_epi32(abiased_l_2, x4_l);
    let mut dif_h_2: __m128i = _mm_sub_epi32(abiased_h_2, x4_h);
    row3 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_2, 10 as libc::c_int),
        _mm_srai_epi32(sum_h_2, 10 as libc::c_int),
    );
    row4 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_2, 10 as libc::c_int),
        _mm_srai_epi32(dif_h_2, 10 as libc::c_int),
    );
    tmp = row0;
    row0 = _mm_unpacklo_epi16(row0, row4);
    row4 = _mm_unpackhi_epi16(tmp, row4);
    tmp = row1;
    row1 = _mm_unpacklo_epi16(row1, row5);
    row5 = _mm_unpackhi_epi16(tmp, row5);
    tmp = row2;
    row2 = _mm_unpacklo_epi16(row2, row6);
    row6 = _mm_unpackhi_epi16(tmp, row6);
    tmp = row3;
    row3 = _mm_unpacklo_epi16(row3, row7);
    row7 = _mm_unpackhi_epi16(tmp, row7);
    tmp = row0;
    row0 = _mm_unpacklo_epi16(row0, row2);
    row2 = _mm_unpackhi_epi16(tmp, row2);
    tmp = row1;
    row1 = _mm_unpacklo_epi16(row1, row3);
    row3 = _mm_unpackhi_epi16(tmp, row3);
    tmp = row4;
    row4 = _mm_unpacklo_epi16(row4, row6);
    row6 = _mm_unpackhi_epi16(tmp, row6);
    tmp = row5;
    row5 = _mm_unpacklo_epi16(row5, row7);
    row7 = _mm_unpackhi_epi16(tmp, row7);
    tmp = row0;
    row0 = _mm_unpacklo_epi16(row0, row1);
    row1 = _mm_unpackhi_epi16(tmp, row1);
    tmp = row2;
    row2 = _mm_unpacklo_epi16(row2, row3);
    row3 = _mm_unpackhi_epi16(tmp, row3);
    tmp = row4;
    row4 = _mm_unpacklo_epi16(row4, row5);
    row5 = _mm_unpackhi_epi16(tmp, row5);
    tmp = row6;
    row6 = _mm_unpacklo_epi16(row6, row7);
    row7 = _mm_unpackhi_epi16(tmp, row7);
    let mut rot0_0lo_0: __m128i = _mm_unpacklo_epi16(row2, row6);
    let mut rot0_0hi_0: __m128i = _mm_unpackhi_epi16(row2, row6);
    let mut t2e_l_0: __m128i = _mm_madd_epi16(rot0_0lo_0, rot0_0);
    let mut t2e_h_0: __m128i = _mm_madd_epi16(rot0_0hi_0, rot0_0);
    let mut t3e_l_0: __m128i = _mm_madd_epi16(rot0_0lo_0, rot0_1);
    let mut t3e_h_0: __m128i = _mm_madd_epi16(rot0_0hi_0, rot0_1);
    let mut sum04_0: __m128i = _mm_add_epi16(row0, row4);
    let mut dif04_0: __m128i = _mm_sub_epi16(row0, row4);
    let mut t0e_l_0: __m128i = _mm_srai_epi32(
        _mm_unpacklo_epi16(_mm_setzero_si128(), sum04_0),
        4 as libc::c_int,
    );
    let mut t0e_h_0: __m128i = _mm_srai_epi32(
        _mm_unpackhi_epi16(_mm_setzero_si128(), sum04_0),
        4 as libc::c_int,
    );
    let mut t1e_l_0: __m128i = _mm_srai_epi32(
        _mm_unpacklo_epi16(_mm_setzero_si128(), dif04_0),
        4 as libc::c_int,
    );
    let mut t1e_h_0: __m128i = _mm_srai_epi32(
        _mm_unpackhi_epi16(_mm_setzero_si128(), dif04_0),
        4 as libc::c_int,
    );
    let mut x0_l_0: __m128i = _mm_add_epi32(t0e_l_0, t3e_l_0);
    let mut x0_h_0: __m128i = _mm_add_epi32(t0e_h_0, t3e_h_0);
    let mut x3_l_0: __m128i = _mm_sub_epi32(t0e_l_0, t3e_l_0);
    let mut x3_h_0: __m128i = _mm_sub_epi32(t0e_h_0, t3e_h_0);
    let mut x1_l_0: __m128i = _mm_add_epi32(t1e_l_0, t2e_l_0);
    let mut x1_h_0: __m128i = _mm_add_epi32(t1e_h_0, t2e_h_0);
    let mut x2_l_0: __m128i = _mm_sub_epi32(t1e_l_0, t2e_l_0);
    let mut x2_h_0: __m128i = _mm_sub_epi32(t1e_h_0, t2e_h_0);
    let mut rot2_0lo_0: __m128i = _mm_unpacklo_epi16(row7, row3);
    let mut rot2_0hi_0: __m128i = _mm_unpackhi_epi16(row7, row3);
    let mut y0o_l_0: __m128i = _mm_madd_epi16(rot2_0lo_0, rot2_0);
    let mut y0o_h_0: __m128i = _mm_madd_epi16(rot2_0hi_0, rot2_0);
    let mut y2o_l_0: __m128i = _mm_madd_epi16(rot2_0lo_0, rot2_1);
    let mut y2o_h_0: __m128i = _mm_madd_epi16(rot2_0hi_0, rot2_1);
    let mut rot3_0lo_0: __m128i = _mm_unpacklo_epi16(row5, row1);
    let mut rot3_0hi_0: __m128i = _mm_unpackhi_epi16(row5, row1);
    let mut y1o_l_0: __m128i = _mm_madd_epi16(rot3_0lo_0, rot3_0);
    let mut y1o_h_0: __m128i = _mm_madd_epi16(rot3_0hi_0, rot3_0);
    let mut y3o_l_0: __m128i = _mm_madd_epi16(rot3_0lo_0, rot3_1);
    let mut y3o_h_0: __m128i = _mm_madd_epi16(rot3_0hi_0, rot3_1);
    let mut sum17_0: __m128i = _mm_add_epi16(row1, row7);
    let mut sum35_0: __m128i = _mm_add_epi16(row3, row5);
    let mut rot1_0lo_0: __m128i = _mm_unpacklo_epi16(sum17_0, sum35_0);
    let mut rot1_0hi_0: __m128i = _mm_unpackhi_epi16(sum17_0, sum35_0);
    let mut y4o_l_0: __m128i = _mm_madd_epi16(rot1_0lo_0, rot1_0);
    let mut y4o_h_0: __m128i = _mm_madd_epi16(rot1_0hi_0, rot1_0);
    let mut y5o_l_0: __m128i = _mm_madd_epi16(rot1_0lo_0, rot1_1);
    let mut y5o_h_0: __m128i = _mm_madd_epi16(rot1_0hi_0, rot1_1);
    let mut x4_l_0: __m128i = _mm_add_epi32(y0o_l_0, y4o_l_0);
    let mut x4_h_0: __m128i = _mm_add_epi32(y0o_h_0, y4o_h_0);
    let mut x5_l_0: __m128i = _mm_add_epi32(y1o_l_0, y5o_l_0);
    let mut x5_h_0: __m128i = _mm_add_epi32(y1o_h_0, y5o_h_0);
    let mut x6_l_0: __m128i = _mm_add_epi32(y2o_l_0, y5o_l_0);
    let mut x6_h_0: __m128i = _mm_add_epi32(y2o_h_0, y5o_h_0);
    let mut x7_l_0: __m128i = _mm_add_epi32(y3o_l_0, y4o_l_0);
    let mut x7_h_0: __m128i = _mm_add_epi32(y3o_h_0, y4o_h_0);
    let mut abiased_l_3: __m128i = _mm_add_epi32(x0_l_0, bias_1);
    let mut abiased_h_3: __m128i = _mm_add_epi32(x0_h_0, bias_1);
    let mut sum_l_3: __m128i = _mm_add_epi32(abiased_l_3, x7_l_0);
    let mut sum_h_3: __m128i = _mm_add_epi32(abiased_h_3, x7_h_0);
    let mut dif_l_3: __m128i = _mm_sub_epi32(abiased_l_3, x7_l_0);
    let mut dif_h_3: __m128i = _mm_sub_epi32(abiased_h_3, x7_h_0);
    row0 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_3, 17 as libc::c_int),
        _mm_srai_epi32(sum_h_3, 17 as libc::c_int),
    );
    row7 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_3, 17 as libc::c_int),
        _mm_srai_epi32(dif_h_3, 17 as libc::c_int),
    );
    let mut abiased_l_4: __m128i = _mm_add_epi32(x1_l_0, bias_1);
    let mut abiased_h_4: __m128i = _mm_add_epi32(x1_h_0, bias_1);
    let mut sum_l_4: __m128i = _mm_add_epi32(abiased_l_4, x6_l_0);
    let mut sum_h_4: __m128i = _mm_add_epi32(abiased_h_4, x6_h_0);
    let mut dif_l_4: __m128i = _mm_sub_epi32(abiased_l_4, x6_l_0);
    let mut dif_h_4: __m128i = _mm_sub_epi32(abiased_h_4, x6_h_0);
    row1 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_4, 17 as libc::c_int),
        _mm_srai_epi32(sum_h_4, 17 as libc::c_int),
    );
    row6 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_4, 17 as libc::c_int),
        _mm_srai_epi32(dif_h_4, 17 as libc::c_int),
    );
    let mut abiased_l_5: __m128i = _mm_add_epi32(x2_l_0, bias_1);
    let mut abiased_h_5: __m128i = _mm_add_epi32(x2_h_0, bias_1);
    let mut sum_l_5: __m128i = _mm_add_epi32(abiased_l_5, x5_l_0);
    let mut sum_h_5: __m128i = _mm_add_epi32(abiased_h_5, x5_h_0);
    let mut dif_l_5: __m128i = _mm_sub_epi32(abiased_l_5, x5_l_0);
    let mut dif_h_5: __m128i = _mm_sub_epi32(abiased_h_5, x5_h_0);
    row2 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_5, 17 as libc::c_int),
        _mm_srai_epi32(sum_h_5, 17 as libc::c_int),
    );
    row5 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_5, 17 as libc::c_int),
        _mm_srai_epi32(dif_h_5, 17 as libc::c_int),
    );
    let mut abiased_l_6: __m128i = _mm_add_epi32(x3_l_0, bias_1);
    let mut abiased_h_6: __m128i = _mm_add_epi32(x3_h_0, bias_1);
    let mut sum_l_6: __m128i = _mm_add_epi32(abiased_l_6, x4_l_0);
    let mut sum_h_6: __m128i = _mm_add_epi32(abiased_h_6, x4_h_0);
    let mut dif_l_6: __m128i = _mm_sub_epi32(abiased_l_6, x4_l_0);
    let mut dif_h_6: __m128i = _mm_sub_epi32(abiased_h_6, x4_h_0);
    row3 = _mm_packs_epi32(
        _mm_srai_epi32(sum_l_6, 17 as libc::c_int),
        _mm_srai_epi32(sum_h_6, 17 as libc::c_int),
    );
    row4 = _mm_packs_epi32(
        _mm_srai_epi32(dif_l_6, 17 as libc::c_int),
        _mm_srai_epi32(dif_h_6, 17 as libc::c_int),
    );
    let mut p0: __m128i = _mm_packus_epi16(row0, row1);
    let mut p1: __m128i = _mm_packus_epi16(row2, row3);
    let mut p2: __m128i = _mm_packus_epi16(row4, row5);
    let mut p3: __m128i = _mm_packus_epi16(row6, row7);
    tmp = p0;
    p0 = _mm_unpacklo_epi8(p0, p2);
    p2 = _mm_unpackhi_epi8(tmp, p2);
    tmp = p1;
    p1 = _mm_unpacklo_epi8(p1, p3);
    p3 = _mm_unpackhi_epi8(tmp, p3);
    tmp = p0;
    p0 = _mm_unpacklo_epi8(p0, p1);
    p1 = _mm_unpackhi_epi8(tmp, p1);
    tmp = p2;
    p2 = _mm_unpacklo_epi8(p2, p3);
    p3 = _mm_unpackhi_epi8(tmp, p3);
    tmp = p0;
    p0 = _mm_unpacklo_epi8(p0, p2);
    p2 = _mm_unpackhi_epi8(tmp, p2);
    tmp = p1;
    p1 = _mm_unpacklo_epi8(p1, p3);
    p3 = _mm_unpackhi_epi8(tmp, p3);
    _mm_storel_epi64(out as *mut __m128i, p0);
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, _mm_shuffle_epi32(p0, 0x4e as libc::c_int));
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, p2);
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, _mm_shuffle_epi32(p2, 0x4e as libc::c_int));
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, p1);
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, _mm_shuffle_epi32(p1, 0x4e as libc::c_int));
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, p3);
    out = out.offset(out_stride as isize);
    _mm_storel_epi64(out as *mut __m128i, _mm_shuffle_epi32(p3, 0x4e as libc::c_int));
}
unsafe extern "C" fn stbi__YCbCr_to_RGB_simd(
    mut out: *mut stbi_uc,
    mut y: *const stbi_uc,
    mut pcb: *const stbi_uc,
    mut pcr: *const stbi_uc,
    mut count: libc::c_int,
    mut step: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    if step == 4 as libc::c_int {
        let mut signflip: __m128i = _mm_set1_epi8(
            -(0x80 as libc::c_int) as libc::c_char,
        );
        let mut cr_const0: __m128i = _mm_set1_epi16(
            (1.40200f32 * 4096.0f32 + 0.5f32) as libc::c_short,
        );
        let mut cr_const1: __m128i = _mm_set1_epi16(
            -((0.71414f32 * 4096.0f32 + 0.5f32) as libc::c_short as libc::c_int)
                as libc::c_short,
        );
        let mut cb_const0: __m128i = _mm_set1_epi16(
            -((0.34414f32 * 4096.0f32 + 0.5f32) as libc::c_short as libc::c_int)
                as libc::c_short,
        );
        let mut cb_const1: __m128i = _mm_set1_epi16(
            (1.77200f32 * 4096.0f32 + 0.5f32) as libc::c_short,
        );
        let mut y_bias: __m128i = _mm_set1_epi8(
            128 as libc::c_int as libc::c_uchar as libc::c_char,
        );
        let mut xw: __m128i = _mm_set1_epi16(255 as libc::c_int as libc::c_short);
        while (i + 7 as libc::c_int) < count {
            let mut y_bytes: __m128i = _mm_loadl_epi64(
                y.offset(i as isize) as *mut __m128i,
            );
            let mut cr_bytes: __m128i = _mm_loadl_epi64(
                pcr.offset(i as isize) as *mut __m128i,
            );
            let mut cb_bytes: __m128i = _mm_loadl_epi64(
                pcb.offset(i as isize) as *mut __m128i,
            );
            let mut cr_biased: __m128i = _mm_xor_si128(cr_bytes, signflip);
            let mut cb_biased: __m128i = _mm_xor_si128(cb_bytes, signflip);
            let mut yw: __m128i = _mm_unpacklo_epi8(y_bias, y_bytes);
            let mut crw: __m128i = _mm_unpacklo_epi8(_mm_setzero_si128(), cr_biased);
            let mut cbw: __m128i = _mm_unpacklo_epi8(_mm_setzero_si128(), cb_biased);
            let mut yws: __m128i = _mm_srli_epi16(yw, 4 as libc::c_int);
            let mut cr0: __m128i = _mm_mulhi_epi16(cr_const0, crw);
            let mut cb0: __m128i = _mm_mulhi_epi16(cb_const0, cbw);
            let mut cb1: __m128i = _mm_mulhi_epi16(cbw, cb_const1);
            let mut cr1: __m128i = _mm_mulhi_epi16(crw, cr_const1);
            let mut rws: __m128i = _mm_add_epi16(cr0, yws);
            let mut gwt: __m128i = _mm_add_epi16(cb0, yws);
            let mut bws: __m128i = _mm_add_epi16(yws, cb1);
            let mut gws: __m128i = _mm_add_epi16(gwt, cr1);
            let mut rw: __m128i = _mm_srai_epi16(rws, 4 as libc::c_int);
            let mut bw: __m128i = _mm_srai_epi16(bws, 4 as libc::c_int);
            let mut gw: __m128i = _mm_srai_epi16(gws, 4 as libc::c_int);
            let mut brb: __m128i = _mm_packus_epi16(rw, bw);
            let mut gxb: __m128i = _mm_packus_epi16(gw, xw);
            let mut t0: __m128i = _mm_unpacklo_epi8(brb, gxb);
            let mut t1: __m128i = _mm_unpackhi_epi8(brb, gxb);
            let mut o0: __m128i = _mm_unpacklo_epi16(t0, t1);
            let mut o1: __m128i = _mm_unpackhi_epi16(t0, t1);
            _mm_storeu_si128(out.offset(0 as libc::c_int as isize) as *mut __m128i, o0);
            _mm_storeu_si128(out.offset(16 as libc::c_int as isize) as *mut __m128i, o1);
            out = out.offset(32 as libc::c_int as isize);
            i += 8 as libc::c_int;
        }
    }
    while i < count {
        let mut y_fixed: libc::c_int = ((*y.offset(i as isize) as libc::c_int)
            << 20 as libc::c_int) + ((1 as libc::c_int) << 19 as libc::c_int);
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut cr: libc::c_int = *pcr.offset(i as isize) as libc::c_int
            - 128 as libc::c_int;
        let mut cb: libc::c_int = *pcb.offset(i as isize) as libc::c_int
            - 128 as libc::c_int;
        r = y_fixed
            + cr
                * (((1.40200f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                    << 8 as libc::c_int);
        g = ((y_fixed
            + cr
                * -(((0.71414f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                    << 8 as libc::c_int)) as libc::c_uint)
            .wrapping_add(
                (cb
                    * -(((0.34414f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                        << 8 as libc::c_int)) as libc::c_uint
                    & 0xffff0000 as libc::c_uint,
            ) as libc::c_int;
        b = y_fixed
            + cb
                * (((1.77200f32 * 4096.0f32 + 0.5f32) as libc::c_int)
                    << 8 as libc::c_int);
        r >>= 20 as libc::c_int;
        g >>= 20 as libc::c_int;
        b >>= 20 as libc::c_int;
        if r as libc::c_uint > 255 as libc::c_int as libc::c_uint {
            if r < 0 as libc::c_int {
                r = 0 as libc::c_int;
            } else {
                r = 255 as libc::c_int;
            }
        }
        if g as libc::c_uint > 255 as libc::c_int as libc::c_uint {
            if g < 0 as libc::c_int {
                g = 0 as libc::c_int;
            } else {
                g = 255 as libc::c_int;
            }
        }
        if b as libc::c_uint > 255 as libc::c_int as libc::c_uint {
            if b < 0 as libc::c_int {
                b = 0 as libc::c_int;
            } else {
                b = 255 as libc::c_int;
            }
        }
        *out.offset(0 as libc::c_int as isize) = r as stbi_uc;
        *out.offset(1 as libc::c_int as isize) = g as stbi_uc;
        *out.offset(2 as libc::c_int as isize) = b as stbi_uc;
        *out.offset(3 as libc::c_int as isize) = 255 as libc::c_int as stbi_uc;
        out = out.offset(step as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn stbi__setup_jpeg(mut j: *mut stbi__jpeg) {
    (*j)
        .idct_block_kernel = Some(
        stbi__idct_block
            as unsafe extern "C" fn(*mut stbi_uc, libc::c_int, *mut libc::c_short) -> (),
    );
    (*j)
        .YCbCr_to_RGB_kernel = Some(
        stbi__YCbCr_to_RGB_row
            as unsafe extern "C" fn(
                *mut stbi_uc,
                *const stbi_uc,
                *const stbi_uc,
                *const stbi_uc,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    (*j)
        .resample_row_hv_2_kernel = Some(
        stbi__resample_row_hv_2
            as unsafe extern "C" fn(
                *mut stbi_uc,
                *mut stbi_uc,
                *mut stbi_uc,
                libc::c_int,
                libc::c_int,
            ) -> *mut stbi_uc,
    );
    if stbi__sse2_available() != 0 {
        (*j)
            .idct_block_kernel = Some(
            stbi__idct_simd
                as unsafe extern "C" fn(
                    *mut stbi_uc,
                    libc::c_int,
                    *mut libc::c_short,
                ) -> (),
        );
        (*j)
            .YCbCr_to_RGB_kernel = Some(
            stbi__YCbCr_to_RGB_simd
                as unsafe extern "C" fn(
                    *mut stbi_uc,
                    *const stbi_uc,
                    *const stbi_uc,
                    *const stbi_uc,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
        );
        (*j)
            .resample_row_hv_2_kernel = Some(
            stbi__resample_row_hv_2_simd
                as unsafe extern "C" fn(
                    *mut stbi_uc,
                    *mut stbi_uc,
                    *mut stbi_uc,
                    libc::c_int,
                    libc::c_int,
                ) -> *mut stbi_uc,
        );
    }
}
unsafe extern "C" fn stbi__addsizes_valid(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    if b < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (a <= 2147483647 as libc::c_int - b) as libc::c_int;
}
unsafe extern "C" fn stbi__process_frame_header(
    mut z: *mut stbi__jpeg,
    mut scan: libc::c_int,
) -> libc::c_int {
    let mut s: *mut stbi__context = (*z).s;
    let mut Lf: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut q: libc::c_int = 0;
    let mut h_max: libc::c_int = 1 as libc::c_int;
    let mut v_max: libc::c_int = 1 as libc::c_int;
    let mut c: libc::c_int = 0;
    Lf = stbi__get16be(s);
    if Lf < 11 as libc::c_int {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    p = stbi__get8(s) as libc::c_int;
    if p != 8 as libc::c_int {
        return stbi__err(
            b"JPEG format not supported: 8-bit only\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*s).img_y = stbi__get16be(s) as stbi__uint32;
    if (*s).img_y == 0 as libc::c_int as libc::c_uint {
        return stbi__err(
            b"JPEG format not supported: delayed height\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*s).img_x = stbi__get16be(s) as stbi__uint32;
    if (*s).img_x == 0 as libc::c_int as libc::c_uint {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    if (*s).img_y > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint {
        return stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*s).img_x > ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_uint {
        return stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        );
    }
    c = stbi__get8(s) as libc::c_int;
    if c != 3 as libc::c_int && c != 1 as libc::c_int && c != 4 as libc::c_int {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    (*s).img_n = c;
    i = 0 as libc::c_int;
    while i < c {
        (*z).img_comp[i as usize].data = 0 as *mut stbi_uc;
        (*z).img_comp[i as usize].linebuf = 0 as *mut stbi_uc;
        i += 1;
        i;
    }
    if Lf != 8 as libc::c_int + 3 as libc::c_int * (*s).img_n {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    (*z).rgb = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*s).img_n {
        static mut rgb: [libc::c_uchar; 3] = [
            'R' as i32 as libc::c_uchar,
            'G' as i32 as libc::c_uchar,
            'B' as i32 as libc::c_uchar,
        ];
        (*z).img_comp[i as usize].id = stbi__get8(s) as libc::c_int;
        if (*s).img_n == 3 as libc::c_int
            && (*z).img_comp[i as usize].id == rgb[i as usize] as libc::c_int
        {
            (*z).rgb += 1;
            (*z).rgb;
        }
        q = stbi__get8(s) as libc::c_int;
        (*z).img_comp[i as usize].h = q >> 4 as libc::c_int;
        if (*z).img_comp[i as usize].h == 0
            || (*z).img_comp[i as usize].h > 4 as libc::c_int
        {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        (*z).img_comp[i as usize].v = q & 15 as libc::c_int;
        if (*z).img_comp[i as usize].v == 0
            || (*z).img_comp[i as usize].v > 4 as libc::c_int
        {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        (*z).img_comp[i as usize].tq = stbi__get8(s) as libc::c_int;
        if (*z).img_comp[i as usize].tq > 3 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    if scan != STBI__SCAN_load as libc::c_int {
        return 1 as libc::c_int;
    }
    if stbi__mad3sizes_valid(
        (*s).img_x as libc::c_int,
        (*s).img_y as libc::c_int,
        (*s).img_n,
        0 as libc::c_int,
    ) == 0
    {
        return stbi__err(
            b"Image too large to decode\0" as *const u8 as *const libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < (*s).img_n {
        if (*z).img_comp[i as usize].h > h_max {
            h_max = (*z).img_comp[i as usize].h;
        }
        if (*z).img_comp[i as usize].v > v_max {
            v_max = (*z).img_comp[i as usize].v;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < (*s).img_n {
        if h_max % (*z).img_comp[i as usize].h != 0 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        if v_max % (*z).img_comp[i as usize].v != 0 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    (*z).img_h_max = h_max;
    (*z).img_v_max = v_max;
    (*z).img_mcu_w = h_max * 8 as libc::c_int;
    (*z).img_mcu_h = v_max * 8 as libc::c_int;
    (*z)
        .img_mcu_x = ((*s).img_x)
        .wrapping_add((*z).img_mcu_w as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div((*z).img_mcu_w as libc::c_uint) as libc::c_int;
    (*z)
        .img_mcu_y = ((*s).img_y)
        .wrapping_add((*z).img_mcu_h as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div((*z).img_mcu_h as libc::c_uint) as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*s).img_n {
        (*z)
            .img_comp[i as usize]
            .x = ((*s).img_x)
            .wrapping_mul((*z).img_comp[i as usize].h as libc::c_uint)
            .wrapping_add(h_max as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(h_max as libc::c_uint) as libc::c_int;
        (*z)
            .img_comp[i as usize]
            .y = ((*s).img_y)
            .wrapping_mul((*z).img_comp[i as usize].v as libc::c_uint)
            .wrapping_add(v_max as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div(v_max as libc::c_uint) as libc::c_int;
        (*z)
            .img_comp[i as usize]
            .w2 = (*z).img_mcu_x * (*z).img_comp[i as usize].h * 8 as libc::c_int;
        (*z)
            .img_comp[i as usize]
            .h2 = (*z).img_mcu_y * (*z).img_comp[i as usize].v * 8 as libc::c_int;
        (*z).img_comp[i as usize].coeff = 0 as *mut libc::c_short;
        (*z).img_comp[i as usize].raw_coeff = 0 as *mut libc::c_void;
        (*z).img_comp[i as usize].linebuf = 0 as *mut stbi_uc;
        (*z)
            .img_comp[i as usize]
            .raw_data = stbi__malloc_mad2(
            (*z).img_comp[i as usize].w2,
            (*z).img_comp[i as usize].h2,
            15 as libc::c_int,
        );
        if ((*z).img_comp[i as usize].raw_data).is_null() {
            return stbi__free_jpeg_components(
                z,
                i + 1 as libc::c_int,
                stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char),
            );
        }
        (*z)
            .img_comp[i as usize]
            .data = (((*z).img_comp[i as usize].raw_data as size_t)
            .wrapping_add(15 as libc::c_int as libc::c_ulong)
            & !(15 as libc::c_int) as libc::c_ulong) as *mut stbi_uc;
        if (*z).progressive != 0 {
            (*z)
                .img_comp[i as usize]
                .coeff_w = (*z).img_comp[i as usize].w2 / 8 as libc::c_int;
            (*z)
                .img_comp[i as usize]
                .coeff_h = (*z).img_comp[i as usize].h2 / 8 as libc::c_int;
            (*z)
                .img_comp[i as usize]
                .raw_coeff = stbi__malloc_mad3(
                (*z).img_comp[i as usize].w2,
                (*z).img_comp[i as usize].h2,
                ::core::mem::size_of::<libc::c_short>() as libc::c_ulong as libc::c_int,
                15 as libc::c_int,
            );
            if ((*z).img_comp[i as usize].raw_coeff).is_null() {
                return stbi__free_jpeg_components(
                    z,
                    i + 1 as libc::c_int,
                    stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char),
                );
            }
            (*z)
                .img_comp[i as usize]
                .coeff = (((*z).img_comp[i as usize].raw_coeff as size_t)
                .wrapping_add(15 as libc::c_int as libc::c_ulong)
                & !(15 as libc::c_int) as libc::c_ulong) as *mut libc::c_short;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__decode_jpeg_header(
    mut z: *mut stbi__jpeg,
    mut scan: libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = 0;
    (*z).jfif = 0 as libc::c_int;
    (*z).app14_color_transform = -(1 as libc::c_int);
    (*z).marker = 0xff as libc::c_int as libc::c_uchar;
    m = stbi__get_marker(z) as libc::c_int;
    if !(m == 0xd8 as libc::c_int) {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    if scan == STBI__SCAN_type as libc::c_int {
        return 1 as libc::c_int;
    }
    m = stbi__get_marker(z) as libc::c_int;
    while !(m == 0xc0 as libc::c_int || m == 0xc1 as libc::c_int
        || m == 0xc2 as libc::c_int)
    {
        if stbi__process_marker(z, m) == 0 {
            return 0 as libc::c_int;
        }
        m = stbi__get_marker(z) as libc::c_int;
        while m == 0xff as libc::c_int {
            if stbi__at_eof((*z).s) != 0 {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
            }
            m = stbi__get_marker(z) as libc::c_int;
        }
    }
    (*z).progressive = (m == 0xc2 as libc::c_int) as libc::c_int;
    if stbi__process_frame_header(z, scan) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__process_scan_header(mut z: *mut stbi__jpeg) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut Ls: libc::c_int = stbi__get16be((*z).s);
    (*z).scan_n = stbi__get8((*z).s) as libc::c_int;
    if (*z).scan_n < 1 as libc::c_int || (*z).scan_n > 4 as libc::c_int
        || (*z).scan_n > (*(*z).s).img_n
    {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    if Ls != 6 as libc::c_int + 2 as libc::c_int * (*z).scan_n {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < (*z).scan_n {
        let mut id: libc::c_int = stbi__get8((*z).s) as libc::c_int;
        let mut which: libc::c_int = 0;
        let mut q: libc::c_int = stbi__get8((*z).s) as libc::c_int;
        which = 0 as libc::c_int;
        while which < (*(*z).s).img_n {
            if (*z).img_comp[which as usize].id == id {
                break;
            }
            which += 1;
            which;
        }
        if which == (*(*z).s).img_n {
            return 0 as libc::c_int;
        }
        (*z).img_comp[which as usize].hd = q >> 4 as libc::c_int;
        if (*z).img_comp[which as usize].hd > 3 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        (*z).img_comp[which as usize].ha = q & 15 as libc::c_int;
        if (*z).img_comp[which as usize].ha > 3 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        (*z).order[i as usize] = which;
        i += 1;
        i;
    }
    let mut aa: libc::c_int = 0;
    (*z).spec_start = stbi__get8((*z).s) as libc::c_int;
    (*z).spec_end = stbi__get8((*z).s) as libc::c_int;
    aa = stbi__get8((*z).s) as libc::c_int;
    (*z).succ_high = aa >> 4 as libc::c_int;
    (*z).succ_low = aa & 15 as libc::c_int;
    if (*z).progressive != 0 {
        if (*z).spec_start > 63 as libc::c_int || (*z).spec_end > 63 as libc::c_int
            || (*z).spec_start > (*z).spec_end || (*z).succ_high > 13 as libc::c_int
            || (*z).succ_low > 13 as libc::c_int
        {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
    } else {
        if (*z).spec_start != 0 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        if (*z).succ_high != 0 as libc::c_int || (*z).succ_low != 0 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        (*z).spec_end = 63 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_decode_block(
    mut j: *mut stbi__jpeg,
    mut data: *mut libc::c_short,
    mut hdc: *mut stbi__huffman,
    mut hac: *mut stbi__huffman,
    mut fac: *mut stbi__int16,
    mut b: libc::c_int,
    mut dequant: *mut stbi__uint16,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut dc: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    if (*j).code_bits < 16 as libc::c_int {
        stbi__grow_buffer_unsafe(j);
    }
    t = stbi__jpeg_huff_decode(j, hdc);
    if t < 0 as libc::c_int || t > 15 as libc::c_int {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    memset(
        data as *mut libc::c_void,
        0 as libc::c_int,
        (64 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
    );
    diff = if t != 0 { stbi__extend_receive(j, t) } else { 0 as libc::c_int };
    dc = (*j).img_comp[b as usize].dc_pred + diff;
    (*j).img_comp[b as usize].dc_pred = dc;
    *data
        .offset(
            0 as libc::c_int as isize,
        ) = (dc * *dequant.offset(0 as libc::c_int as isize) as libc::c_int)
        as libc::c_short;
    k = 1 as libc::c_int;
    loop {
        let mut zig: libc::c_uint = 0;
        let mut c: libc::c_int = 0;
        let mut r: libc::c_int = 0;
        let mut s: libc::c_int = 0;
        if (*j).code_bits < 16 as libc::c_int {
            stbi__grow_buffer_unsafe(j);
        }
        c = ((*j).code_buffer >> 32 as libc::c_int - 9 as libc::c_int
            & (((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int)
                as libc::c_uint) as libc::c_int;
        r = *fac.offset(c as isize) as libc::c_int;
        if r != 0 {
            k += r >> 4 as libc::c_int & 15 as libc::c_int;
            s = r & 15 as libc::c_int;
            (*j).code_buffer <<= s;
            (*j).code_bits -= s;
            let fresh70 = k;
            k = k + 1;
            zig = stbi__jpeg_dezigzag[fresh70 as usize] as libc::c_uint;
            *data
                .offset(
                    zig as isize,
                ) = ((r >> 8 as libc::c_int)
                * *dequant.offset(zig as isize) as libc::c_int) as libc::c_short;
        } else {
            let mut rs: libc::c_int = stbi__jpeg_huff_decode(j, hac);
            if rs < 0 as libc::c_int {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
            }
            s = rs & 15 as libc::c_int;
            r = rs >> 4 as libc::c_int;
            if s == 0 as libc::c_int {
                if rs != 0xf0 as libc::c_int {
                    break;
                }
                k += 16 as libc::c_int;
            } else {
                k += r;
                let fresh71 = k;
                k = k + 1;
                zig = stbi__jpeg_dezigzag[fresh71 as usize] as libc::c_uint;
                *data
                    .offset(
                        zig as isize,
                    ) = (stbi__extend_receive(j, s)
                    * *dequant.offset(zig as isize) as libc::c_int) as libc::c_short;
            }
        }
        if !(k < 64 as libc::c_int) {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_get_bits(
    mut j: *mut stbi__jpeg,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_uint = 0;
    if (*j).code_bits < n {
        stbi__grow_buffer_unsafe(j);
    }
    k = (*j).code_buffer << n | (*j).code_buffer >> (-n & 31 as libc::c_int);
    (*j).code_buffer = k & !stbi__bmask[n as usize];
    k &= stbi__bmask[n as usize];
    (*j).code_bits -= n;
    return k as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_decode_block_prog_ac(
    mut j: *mut stbi__jpeg,
    mut data: *mut libc::c_short,
    mut hac: *mut stbi__huffman,
    mut fac: *mut stbi__int16,
) -> libc::c_int {
    let mut k: libc::c_int = 0;
    if (*j).spec_start == 0 as libc::c_int {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    if (*j).succ_high == 0 as libc::c_int {
        let mut shift: libc::c_int = (*j).succ_low;
        if (*j).eob_run != 0 {
            (*j).eob_run -= 1;
            (*j).eob_run;
            return 1 as libc::c_int;
        }
        k = (*j).spec_start;
        loop {
            let mut zig: libc::c_uint = 0;
            let mut c: libc::c_int = 0;
            let mut r: libc::c_int = 0;
            let mut s: libc::c_int = 0;
            if (*j).code_bits < 16 as libc::c_int {
                stbi__grow_buffer_unsafe(j);
            }
            c = ((*j).code_buffer >> 32 as libc::c_int - 9 as libc::c_int
                & (((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint) as libc::c_int;
            r = *fac.offset(c as isize) as libc::c_int;
            if r != 0 {
                k += r >> 4 as libc::c_int & 15 as libc::c_int;
                s = r & 15 as libc::c_int;
                (*j).code_buffer <<= s;
                (*j).code_bits -= s;
                let fresh72 = k;
                k = k + 1;
                zig = stbi__jpeg_dezigzag[fresh72 as usize] as libc::c_uint;
                *data
                    .offset(
                        zig as isize,
                    ) = ((r >> 8 as libc::c_int) * ((1 as libc::c_int) << shift))
                    as libc::c_short;
            } else {
                let mut rs: libc::c_int = stbi__jpeg_huff_decode(j, hac);
                if rs < 0 as libc::c_int {
                    return stbi__err(
                        b"Corrupt JPEG\0" as *const u8 as *const libc::c_char,
                    );
                }
                s = rs & 15 as libc::c_int;
                r = rs >> 4 as libc::c_int;
                if s == 0 as libc::c_int {
                    if r < 15 as libc::c_int {
                        (*j).eob_run = (1 as libc::c_int) << r;
                        if r != 0 {
                            (*j).eob_run += stbi__jpeg_get_bits(j, r);
                        }
                        (*j).eob_run -= 1;
                        (*j).eob_run;
                        break;
                    } else {
                        k += 16 as libc::c_int;
                    }
                } else {
                    k += r;
                    let fresh73 = k;
                    k = k + 1;
                    zig = stbi__jpeg_dezigzag[fresh73 as usize] as libc::c_uint;
                    *data
                        .offset(
                            zig as isize,
                        ) = (stbi__extend_receive(j, s) * ((1 as libc::c_int) << shift))
                        as libc::c_short;
                }
            }
            if !(k <= (*j).spec_end) {
                break;
            }
        }
    } else {
        let mut bit: libc::c_short = ((1 as libc::c_int) << (*j).succ_low)
            as libc::c_short;
        if (*j).eob_run != 0 {
            (*j).eob_run -= 1;
            (*j).eob_run;
            k = (*j).spec_start;
            while k <= (*j).spec_end {
                let mut p: *mut libc::c_short = &mut *data
                    .offset(*stbi__jpeg_dezigzag.as_ptr().offset(k as isize) as isize)
                    as *mut libc::c_short;
                if *p as libc::c_int != 0 as libc::c_int {
                    if stbi__jpeg_get_bit(j) != 0 {
                        if *p as libc::c_int & bit as libc::c_int == 0 as libc::c_int {
                            if *p as libc::c_int > 0 as libc::c_int {
                                *p = (*p as libc::c_int + bit as libc::c_int)
                                    as libc::c_short;
                            } else {
                                *p = (*p as libc::c_int - bit as libc::c_int)
                                    as libc::c_short;
                            }
                        }
                    }
                }
                k += 1;
                k;
            }
        } else {
            k = (*j).spec_start;
            loop {
                let mut r_0: libc::c_int = 0;
                let mut s_0: libc::c_int = 0;
                let mut rs_0: libc::c_int = stbi__jpeg_huff_decode(j, hac);
                if rs_0 < 0 as libc::c_int {
                    return stbi__err(
                        b"Corrupt JPEG\0" as *const u8 as *const libc::c_char,
                    );
                }
                s_0 = rs_0 & 15 as libc::c_int;
                r_0 = rs_0 >> 4 as libc::c_int;
                if s_0 == 0 as libc::c_int {
                    if r_0 < 15 as libc::c_int {
                        (*j).eob_run = ((1 as libc::c_int) << r_0) - 1 as libc::c_int;
                        if r_0 != 0 {
                            (*j).eob_run += stbi__jpeg_get_bits(j, r_0);
                        }
                        r_0 = 64 as libc::c_int;
                    }
                } else {
                    if s_0 != 1 as libc::c_int {
                        return stbi__err(
                            b"Corrupt JPEG\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if stbi__jpeg_get_bit(j) != 0 {
                        s_0 = bit as libc::c_int;
                    } else {
                        s_0 = -(bit as libc::c_int);
                    }
                }
                while k <= (*j).spec_end {
                    let fresh74 = k;
                    k = k + 1;
                    let mut p_0: *mut libc::c_short = &mut *data
                        .offset(
                            *stbi__jpeg_dezigzag.as_ptr().offset(fresh74 as isize)
                                as isize,
                        ) as *mut libc::c_short;
                    if *p_0 as libc::c_int != 0 as libc::c_int {
                        if stbi__jpeg_get_bit(j) != 0 {
                            if *p_0 as libc::c_int & bit as libc::c_int
                                == 0 as libc::c_int
                            {
                                if *p_0 as libc::c_int > 0 as libc::c_int {
                                    *p_0 = (*p_0 as libc::c_int + bit as libc::c_int)
                                        as libc::c_short;
                                } else {
                                    *p_0 = (*p_0 as libc::c_int - bit as libc::c_int)
                                        as libc::c_short;
                                }
                            }
                        }
                    } else if r_0 == 0 as libc::c_int {
                        *p_0 = s_0 as libc::c_short;
                        break;
                    } else {
                        r_0 -= 1;
                        r_0;
                    }
                }
                if !(k <= (*j).spec_end) {
                    break;
                }
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_huff_decode(
    mut j: *mut stbi__jpeg,
    mut h: *mut stbi__huffman,
) -> libc::c_int {
    let mut temp: libc::c_uint = 0;
    let mut c: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if (*j).code_bits < 16 as libc::c_int {
        stbi__grow_buffer_unsafe(j);
    }
    c = ((*j).code_buffer >> 32 as libc::c_int - 9 as libc::c_int
        & (((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    k = (*h).fast[c as usize] as libc::c_int;
    if k < 255 as libc::c_int {
        let mut s: libc::c_int = (*h).size[k as usize] as libc::c_int;
        if s > (*j).code_bits {
            return -(1 as libc::c_int);
        }
        (*j).code_buffer <<= s;
        (*j).code_bits -= s;
        return (*h).values[k as usize] as libc::c_int;
    }
    temp = (*j).code_buffer >> 16 as libc::c_int;
    k = 9 as libc::c_int + 1 as libc::c_int;
    while !(temp < (*h).maxcode[k as usize]) {
        k += 1;
        k;
    }
    if k == 17 as libc::c_int {
        (*j).code_bits -= 16 as libc::c_int;
        return -(1 as libc::c_int);
    }
    if k > (*j).code_bits {
        return -(1 as libc::c_int);
    }
    c = ((*j).code_buffer >> 32 as libc::c_int - k & stbi__bmask[k as usize])
        .wrapping_add((*h).delta[k as usize] as libc::c_uint) as libc::c_int;
    if (*j).code_buffer >> 32 as libc::c_int - (*h).size[c as usize] as libc::c_int
        & stbi__bmask[(*h).size[c as usize] as usize]
        == (*h).code[c as usize] as libc::c_uint
    {} else {
        __assert_fail(
            b"(((j->code_buffer) >> (32 - h->size[c])) & stbi__bmask[h->size[c]]) == h->code[c]\0"
                as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            2115 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[libc::c_char; 58],
            >(b"int stbi__jpeg_huff_decode(stbi__jpeg *, stbi__huffman *)\0"))
                .as_ptr(),
        );
    };
    (*j).code_bits -= k;
    (*j).code_buffer <<= k;
    return (*h).values[c as usize] as libc::c_int;
}
static mut stbi__bmask: [stbi__uint32; 17] = [
    0 as libc::c_int as stbi__uint32,
    1 as libc::c_int as stbi__uint32,
    3 as libc::c_int as stbi__uint32,
    7 as libc::c_int as stbi__uint32,
    15 as libc::c_int as stbi__uint32,
    31 as libc::c_int as stbi__uint32,
    63 as libc::c_int as stbi__uint32,
    127 as libc::c_int as stbi__uint32,
    255 as libc::c_int as stbi__uint32,
    511 as libc::c_int as stbi__uint32,
    1023 as libc::c_int as stbi__uint32,
    2047 as libc::c_int as stbi__uint32,
    4095 as libc::c_int as stbi__uint32,
    8191 as libc::c_int as stbi__uint32,
    16383 as libc::c_int as stbi__uint32,
    32767 as libc::c_int as stbi__uint32,
    65535 as libc::c_int as stbi__uint32,
];
static mut stbi__jbias: [libc::c_int; 16] = [
    0 as libc::c_int,
    -(1 as libc::c_int),
    -(3 as libc::c_int),
    -(7 as libc::c_int),
    -(15 as libc::c_int),
    -(31 as libc::c_int),
    -(63 as libc::c_int),
    -(127 as libc::c_int),
    -(255 as libc::c_int),
    -(511 as libc::c_int),
    -(1023 as libc::c_int),
    -(2047 as libc::c_int),
    -(4095 as libc::c_int),
    -(8191 as libc::c_int),
    -(16383 as libc::c_int),
    -(32767 as libc::c_int),
];
unsafe extern "C" fn stbi__extend_receive(
    mut j: *mut stbi__jpeg,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut k: libc::c_uint = 0;
    let mut sgn: libc::c_int = 0;
    if (*j).code_bits < n {
        stbi__grow_buffer_unsafe(j);
    }
    sgn = ((*j).code_buffer >> 31 as libc::c_int) as libc::c_int;
    k = (*j).code_buffer << n | (*j).code_buffer >> (-n & 31 as libc::c_int);
    (*j).code_buffer = k & !stbi__bmask[n as usize];
    k &= stbi__bmask[n as usize];
    (*j).code_bits -= n;
    return k
        .wrapping_add((stbi__jbias[n as usize] & sgn - 1 as libc::c_int) as libc::c_uint)
        as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_get_bit(mut j: *mut stbi__jpeg) -> libc::c_int {
    let mut k: libc::c_uint = 0;
    if (*j).code_bits < 1 as libc::c_int {
        stbi__grow_buffer_unsafe(j);
    }
    k = (*j).code_buffer;
    (*j).code_buffer <<= 1 as libc::c_int;
    (*j).code_bits -= 1;
    (*j).code_bits;
    return (k & 0x80000000 as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn stbi__jpeg_decode_block_prog_dc(
    mut j: *mut stbi__jpeg,
    mut data: *mut libc::c_short,
    mut hdc: *mut stbi__huffman,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut dc: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    if (*j).spec_end != 0 as libc::c_int {
        return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
    }
    if (*j).code_bits < 16 as libc::c_int {
        stbi__grow_buffer_unsafe(j);
    }
    if (*j).succ_high == 0 as libc::c_int {
        memset(
            data as *mut libc::c_void,
            0 as libc::c_int,
            (64 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        t = stbi__jpeg_huff_decode(j, hdc);
        if t < 0 as libc::c_int || t > 15 as libc::c_int {
            return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
        }
        diff = if t != 0 { stbi__extend_receive(j, t) } else { 0 as libc::c_int };
        dc = (*j).img_comp[b as usize].dc_pred + diff;
        (*j).img_comp[b as usize].dc_pred = dc;
        *data
            .offset(
                0 as libc::c_int as isize,
            ) = (dc * ((1 as libc::c_int) << (*j).succ_low)) as libc::c_short;
    } else if stbi__jpeg_get_bit(j) != 0 {
        let ref mut fresh75 = *data.offset(0 as libc::c_int as isize);
        *fresh75 = (*fresh75 as libc::c_int
            + ((1 as libc::c_int) << (*j).succ_low) as libc::c_short as libc::c_int)
            as libc::c_short;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__grow_buffer_unsafe(mut j: *mut stbi__jpeg) {
    loop {
        let mut b: libc::c_uint = (if (*j).nomore != 0 {
            0 as libc::c_int
        } else {
            stbi__get8((*j).s) as libc::c_int
        }) as libc::c_uint;
        if b == 0xff as libc::c_int as libc::c_uint {
            let mut c: libc::c_int = stbi__get8((*j).s) as libc::c_int;
            while c == 0xff as libc::c_int {
                c = stbi__get8((*j).s) as libc::c_int;
            }
            if c != 0 as libc::c_int {
                (*j).marker = c as libc::c_uchar;
                (*j).nomore = 1 as libc::c_int;
                return;
            }
        }
        (*j).code_buffer |= b << 24 as libc::c_int - (*j).code_bits;
        (*j).code_bits += 8 as libc::c_int;
        if !((*j).code_bits <= 24 as libc::c_int) {
            break;
        }
    };
}
unsafe extern "C" fn stbi__jpeg_reset(mut j: *mut stbi__jpeg) {
    (*j).code_bits = 0 as libc::c_int;
    (*j).code_buffer = 0 as libc::c_int as stbi__uint32;
    (*j).nomore = 0 as libc::c_int;
    (*j).img_comp[3 as libc::c_int as usize].dc_pred = 0 as libc::c_int;
    (*j)
        .img_comp[2 as libc::c_int as usize]
        .dc_pred = (*j).img_comp[3 as libc::c_int as usize].dc_pred;
    (*j)
        .img_comp[1 as libc::c_int as usize]
        .dc_pred = (*j).img_comp[2 as libc::c_int as usize].dc_pred;
    (*j)
        .img_comp[0 as libc::c_int as usize]
        .dc_pred = (*j).img_comp[1 as libc::c_int as usize].dc_pred;
    (*j).marker = 0xff as libc::c_int as libc::c_uchar;
    (*j)
        .todo = if (*j).restart_interval != 0 {
        (*j).restart_interval
    } else {
        0x7fffffff as libc::c_int
    };
    (*j).eob_run = 0 as libc::c_int;
}
unsafe extern "C" fn stbi__parse_entropy_coded_data(
    mut z: *mut stbi__jpeg,
) -> libc::c_int {
    stbi__jpeg_reset(z);
    if (*z).progressive == 0 {
        if (*z).scan_n == 1 as libc::c_int {
            let mut i: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut data: [libc::c_short; 64] = [0; 64];
            let mut n: libc::c_int = (*z).order[0 as libc::c_int as usize];
            let mut w: libc::c_int = (*z).img_comp[n as usize].x + 7 as libc::c_int
                >> 3 as libc::c_int;
            let mut h: libc::c_int = (*z).img_comp[n as usize].y + 7 as libc::c_int
                >> 3 as libc::c_int;
            j = 0 as libc::c_int;
            while j < h {
                i = 0 as libc::c_int;
                while i < w {
                    let mut ha: libc::c_int = (*z).img_comp[n as usize].ha;
                    if stbi__jpeg_decode_block(
                        z,
                        data.as_mut_ptr(),
                        ((*z).huff_dc)
                            .as_mut_ptr()
                            .offset((*z).img_comp[n as usize].hd as isize),
                        ((*z).huff_ac).as_mut_ptr().offset(ha as isize),
                        ((*z).fast_ac[ha as usize]).as_mut_ptr(),
                        n,
                        ((*z).dequant[(*z).img_comp[n as usize].tq as usize])
                            .as_mut_ptr(),
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    ((*z).idct_block_kernel)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*z).img_comp[n as usize].data)
                            .offset(
                                ((*z).img_comp[n as usize].w2 * j * 8 as libc::c_int)
                                    as isize,
                            )
                            .offset((i * 8 as libc::c_int) as isize),
                        (*z).img_comp[n as usize].w2,
                        data.as_mut_ptr(),
                    );
                    (*z).todo -= 1;
                    if (*z).todo <= 0 as libc::c_int {
                        if (*z).code_bits < 24 as libc::c_int {
                            stbi__grow_buffer_unsafe(z);
                        }
                        if !((*z).marker as libc::c_int >= 0xd0 as libc::c_int
                            && (*z).marker as libc::c_int <= 0xd7 as libc::c_int)
                        {
                            return 1 as libc::c_int;
                        }
                        stbi__jpeg_reset(z);
                    }
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
            return 1 as libc::c_int;
        } else {
            let mut i_0: libc::c_int = 0;
            let mut j_0: libc::c_int = 0;
            let mut k: libc::c_int = 0;
            let mut x: libc::c_int = 0;
            let mut y: libc::c_int = 0;
            let mut data_0: [libc::c_short; 64] = [0; 64];
            j_0 = 0 as libc::c_int;
            while j_0 < (*z).img_mcu_y {
                i_0 = 0 as libc::c_int;
                while i_0 < (*z).img_mcu_x {
                    k = 0 as libc::c_int;
                    while k < (*z).scan_n {
                        let mut n_0: libc::c_int = (*z).order[k as usize];
                        y = 0 as libc::c_int;
                        while y < (*z).img_comp[n_0 as usize].v {
                            x = 0 as libc::c_int;
                            while x < (*z).img_comp[n_0 as usize].h {
                                let mut x2: libc::c_int = (i_0
                                    * (*z).img_comp[n_0 as usize].h + x) * 8 as libc::c_int;
                                let mut y2: libc::c_int = (j_0
                                    * (*z).img_comp[n_0 as usize].v + y) * 8 as libc::c_int;
                                let mut ha_0: libc::c_int = (*z).img_comp[n_0 as usize].ha;
                                if stbi__jpeg_decode_block(
                                    z,
                                    data_0.as_mut_ptr(),
                                    ((*z).huff_dc)
                                        .as_mut_ptr()
                                        .offset((*z).img_comp[n_0 as usize].hd as isize),
                                    ((*z).huff_ac).as_mut_ptr().offset(ha_0 as isize),
                                    ((*z).fast_ac[ha_0 as usize]).as_mut_ptr(),
                                    n_0,
                                    ((*z).dequant[(*z).img_comp[n_0 as usize].tq as usize])
                                        .as_mut_ptr(),
                                ) == 0
                                {
                                    return 0 as libc::c_int;
                                }
                                ((*z).idct_block_kernel)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ((*z).img_comp[n_0 as usize].data)
                                        .offset(((*z).img_comp[n_0 as usize].w2 * y2) as isize)
                                        .offset(x2 as isize),
                                    (*z).img_comp[n_0 as usize].w2,
                                    data_0.as_mut_ptr(),
                                );
                                x += 1;
                                x;
                            }
                            y += 1;
                            y;
                        }
                        k += 1;
                        k;
                    }
                    (*z).todo -= 1;
                    if (*z).todo <= 0 as libc::c_int {
                        if (*z).code_bits < 24 as libc::c_int {
                            stbi__grow_buffer_unsafe(z);
                        }
                        if !((*z).marker as libc::c_int >= 0xd0 as libc::c_int
                            && (*z).marker as libc::c_int <= 0xd7 as libc::c_int)
                        {
                            return 1 as libc::c_int;
                        }
                        stbi__jpeg_reset(z);
                    }
                    i_0 += 1;
                    i_0;
                }
                j_0 += 1;
                j_0;
            }
            return 1 as libc::c_int;
        }
    } else if (*z).scan_n == 1 as libc::c_int {
        let mut i_1: libc::c_int = 0;
        let mut j_1: libc::c_int = 0;
        let mut n_1: libc::c_int = (*z).order[0 as libc::c_int as usize];
        let mut w_0: libc::c_int = (*z).img_comp[n_1 as usize].x + 7 as libc::c_int
            >> 3 as libc::c_int;
        let mut h_0: libc::c_int = (*z).img_comp[n_1 as usize].y + 7 as libc::c_int
            >> 3 as libc::c_int;
        j_1 = 0 as libc::c_int;
        while j_1 < h_0 {
            i_1 = 0 as libc::c_int;
            while i_1 < w_0 {
                let mut data_1: *mut libc::c_short = ((*z).img_comp[n_1 as usize].coeff)
                    .offset(
                        (64 as libc::c_int
                            * (i_1 + j_1 * (*z).img_comp[n_1 as usize].coeff_w)) as isize,
                    );
                if (*z).spec_start == 0 as libc::c_int {
                    if stbi__jpeg_decode_block_prog_dc(
                        z,
                        data_1,
                        &mut *((*z).huff_dc)
                            .as_mut_ptr()
                            .offset(
                                (*((*z).img_comp).as_mut_ptr().offset(n_1 as isize)).hd
                                    as isize,
                            ),
                        n_1,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                } else {
                    let mut ha_1: libc::c_int = (*z).img_comp[n_1 as usize].ha;
                    if stbi__jpeg_decode_block_prog_ac(
                        z,
                        data_1,
                        &mut *((*z).huff_ac).as_mut_ptr().offset(ha_1 as isize),
                        ((*z).fast_ac[ha_1 as usize]).as_mut_ptr(),
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                }
                (*z).todo -= 1;
                if (*z).todo <= 0 as libc::c_int {
                    if (*z).code_bits < 24 as libc::c_int {
                        stbi__grow_buffer_unsafe(z);
                    }
                    if !((*z).marker as libc::c_int >= 0xd0 as libc::c_int
                        && (*z).marker as libc::c_int <= 0xd7 as libc::c_int)
                    {
                        return 1 as libc::c_int;
                    }
                    stbi__jpeg_reset(z);
                }
                i_1 += 1;
                i_1;
            }
            j_1 += 1;
            j_1;
        }
        return 1 as libc::c_int;
    } else {
        let mut i_2: libc::c_int = 0;
        let mut j_2: libc::c_int = 0;
        let mut k_0: libc::c_int = 0;
        let mut x_0: libc::c_int = 0;
        let mut y_0: libc::c_int = 0;
        j_2 = 0 as libc::c_int;
        while j_2 < (*z).img_mcu_y {
            i_2 = 0 as libc::c_int;
            while i_2 < (*z).img_mcu_x {
                k_0 = 0 as libc::c_int;
                while k_0 < (*z).scan_n {
                    let mut n_2: libc::c_int = (*z).order[k_0 as usize];
                    y_0 = 0 as libc::c_int;
                    while y_0 < (*z).img_comp[n_2 as usize].v {
                        x_0 = 0 as libc::c_int;
                        while x_0 < (*z).img_comp[n_2 as usize].h {
                            let mut x2_0: libc::c_int = i_2
                                * (*z).img_comp[n_2 as usize].h + x_0;
                            let mut y2_0: libc::c_int = j_2
                                * (*z).img_comp[n_2 as usize].v + y_0;
                            let mut data_2: *mut libc::c_short = ((*z)
                                .img_comp[n_2 as usize]
                                .coeff)
                                .offset(
                                    (64 as libc::c_int
                                        * (x2_0 + y2_0 * (*z).img_comp[n_2 as usize].coeff_w))
                                        as isize,
                                );
                            if stbi__jpeg_decode_block_prog_dc(
                                z,
                                data_2,
                                &mut *((*z).huff_dc)
                                    .as_mut_ptr()
                                    .offset(
                                        (*((*z).img_comp).as_mut_ptr().offset(n_2 as isize)).hd
                                            as isize,
                                    ),
                                n_2,
                            ) == 0
                            {
                                return 0 as libc::c_int;
                            }
                            x_0 += 1;
                            x_0;
                        }
                        y_0 += 1;
                        y_0;
                    }
                    k_0 += 1;
                    k_0;
                }
                (*z).todo -= 1;
                if (*z).todo <= 0 as libc::c_int {
                    if (*z).code_bits < 24 as libc::c_int {
                        stbi__grow_buffer_unsafe(z);
                    }
                    if !((*z).marker as libc::c_int >= 0xd0 as libc::c_int
                        && (*z).marker as libc::c_int <= 0xd7 as libc::c_int)
                    {
                        return 1 as libc::c_int;
                    }
                    stbi__jpeg_reset(z);
                }
                i_2 += 1;
                i_2;
            }
            j_2 += 1;
            j_2;
        }
        return 1 as libc::c_int;
    };
}
static mut stbi__jpeg_dezigzag: [stbi_uc; 79] = [
    0 as libc::c_int as stbi_uc,
    1 as libc::c_int as stbi_uc,
    8 as libc::c_int as stbi_uc,
    16 as libc::c_int as stbi_uc,
    9 as libc::c_int as stbi_uc,
    2 as libc::c_int as stbi_uc,
    3 as libc::c_int as stbi_uc,
    10 as libc::c_int as stbi_uc,
    17 as libc::c_int as stbi_uc,
    24 as libc::c_int as stbi_uc,
    32 as libc::c_int as stbi_uc,
    25 as libc::c_int as stbi_uc,
    18 as libc::c_int as stbi_uc,
    11 as libc::c_int as stbi_uc,
    4 as libc::c_int as stbi_uc,
    5 as libc::c_int as stbi_uc,
    12 as libc::c_int as stbi_uc,
    19 as libc::c_int as stbi_uc,
    26 as libc::c_int as stbi_uc,
    33 as libc::c_int as stbi_uc,
    40 as libc::c_int as stbi_uc,
    48 as libc::c_int as stbi_uc,
    41 as libc::c_int as stbi_uc,
    34 as libc::c_int as stbi_uc,
    27 as libc::c_int as stbi_uc,
    20 as libc::c_int as stbi_uc,
    13 as libc::c_int as stbi_uc,
    6 as libc::c_int as stbi_uc,
    7 as libc::c_int as stbi_uc,
    14 as libc::c_int as stbi_uc,
    21 as libc::c_int as stbi_uc,
    28 as libc::c_int as stbi_uc,
    35 as libc::c_int as stbi_uc,
    42 as libc::c_int as stbi_uc,
    49 as libc::c_int as stbi_uc,
    56 as libc::c_int as stbi_uc,
    57 as libc::c_int as stbi_uc,
    50 as libc::c_int as stbi_uc,
    43 as libc::c_int as stbi_uc,
    36 as libc::c_int as stbi_uc,
    29 as libc::c_int as stbi_uc,
    22 as libc::c_int as stbi_uc,
    15 as libc::c_int as stbi_uc,
    23 as libc::c_int as stbi_uc,
    30 as libc::c_int as stbi_uc,
    37 as libc::c_int as stbi_uc,
    44 as libc::c_int as stbi_uc,
    51 as libc::c_int as stbi_uc,
    58 as libc::c_int as stbi_uc,
    59 as libc::c_int as stbi_uc,
    52 as libc::c_int as stbi_uc,
    45 as libc::c_int as stbi_uc,
    38 as libc::c_int as stbi_uc,
    31 as libc::c_int as stbi_uc,
    39 as libc::c_int as stbi_uc,
    46 as libc::c_int as stbi_uc,
    53 as libc::c_int as stbi_uc,
    60 as libc::c_int as stbi_uc,
    61 as libc::c_int as stbi_uc,
    54 as libc::c_int as stbi_uc,
    47 as libc::c_int as stbi_uc,
    55 as libc::c_int as stbi_uc,
    62 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
    63 as libc::c_int as stbi_uc,
];
unsafe extern "C" fn stbi__build_huffman(
    mut h: *mut stbi__huffman,
    mut count: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut code: libc::c_uint = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        j = 0 as libc::c_int;
        while j < *count.offset(i as isize) {
            let fresh76 = k;
            k = k + 1;
            (*h).size[fresh76 as usize] = (i + 1 as libc::c_int) as stbi_uc;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*h).size[k as usize] = 0 as libc::c_int as stbi_uc;
    code = 0 as libc::c_int as libc::c_uint;
    k = 0 as libc::c_int;
    j = 1 as libc::c_int;
    while j <= 16 as libc::c_int {
        (*h).delta[j as usize] = (k as libc::c_uint).wrapping_sub(code) as libc::c_int;
        if (*h).size[k as usize] as libc::c_int == j {
            while (*h).size[k as usize] as libc::c_int == j {
                let fresh77 = code;
                code = code.wrapping_add(1);
                let fresh78 = k;
                k = k + 1;
                (*h).code[fresh78 as usize] = fresh77 as stbi__uint16;
            }
            if code.wrapping_sub(1 as libc::c_int as libc::c_uint)
                >= (1 as libc::c_uint) << j
            {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
            }
        }
        (*h).maxcode[j as usize] = code << 16 as libc::c_int - j;
        code <<= 1 as libc::c_int;
        j += 1;
        j;
    }
    (*h).maxcode[j as usize] = 0xffffffff as libc::c_uint;
    memset(
        ((*h).fast).as_mut_ptr() as *mut libc::c_void,
        255 as libc::c_int,
        ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < k {
        let mut s: libc::c_int = (*h).size[i as usize] as libc::c_int;
        if s <= 9 as libc::c_int {
            let mut c: libc::c_int = ((*h).code[i as usize] as libc::c_int)
                << 9 as libc::c_int - s;
            let mut m: libc::c_int = (1 as libc::c_int) << 9 as libc::c_int - s;
            j = 0 as libc::c_int;
            while j < m {
                (*h).fast[(c + j) as usize] = i as stbi_uc;
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__build_fast_ac(
    mut fast_ac: *mut stbi__int16,
    mut h: *mut stbi__huffman,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 9 as libc::c_int {
        let mut fast: stbi_uc = (*h).fast[i as usize];
        *fast_ac.offset(i as isize) = 0 as libc::c_int as stbi__int16;
        if (fast as libc::c_int) < 255 as libc::c_int {
            let mut rs: libc::c_int = (*h).values[fast as usize] as libc::c_int;
            let mut run: libc::c_int = rs >> 4 as libc::c_int & 15 as libc::c_int;
            let mut magbits: libc::c_int = rs & 15 as libc::c_int;
            let mut len: libc::c_int = (*h).size[fast as usize] as libc::c_int;
            if magbits != 0 && len + magbits <= 9 as libc::c_int {
                let mut k: libc::c_int = (i << len
                    & ((1 as libc::c_int) << 9 as libc::c_int) - 1 as libc::c_int)
                    >> 9 as libc::c_int - magbits;
                let mut m: libc::c_int = (1 as libc::c_int)
                    << magbits - 1 as libc::c_int;
                if k < m {
                    k = (k as libc::c_uint)
                        .wrapping_add(
                            (!(0 as libc::c_uint) << magbits)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ) as libc::c_int as libc::c_int;
                }
                if k >= -(128 as libc::c_int) && k <= 127 as libc::c_int {
                    *fast_ac
                        .offset(
                            i as isize,
                        ) = (k * 256 as libc::c_int + run * 16 as libc::c_int
                        + (len + magbits)) as stbi__int16;
                }
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn stbi__get16be(mut s: *mut stbi__context) -> libc::c_int {
    let mut z: libc::c_int = stbi__get8(s) as libc::c_int;
    return (z << 8 as libc::c_int) + stbi__get8(s) as libc::c_int;
}
unsafe extern "C" fn stbi__process_marker(
    mut z: *mut stbi__jpeg,
    mut m: libc::c_int,
) -> libc::c_int {
    let mut L: libc::c_int = 0;
    match m {
        255 => return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char),
        221 => {
            if stbi__get16be((*z).s) != 4 as libc::c_int {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
            }
            (*z).restart_interval = stbi__get16be((*z).s);
            return 1 as libc::c_int;
        }
        219 => {
            L = stbi__get16be((*z).s) - 2 as libc::c_int;
            while L > 0 as libc::c_int {
                let mut q: libc::c_int = stbi__get8((*z).s) as libc::c_int;
                let mut p: libc::c_int = q >> 4 as libc::c_int;
                let mut sixteen: libc::c_int = (p != 0 as libc::c_int) as libc::c_int;
                let mut t: libc::c_int = q & 15 as libc::c_int;
                let mut i: libc::c_int = 0;
                if p != 0 as libc::c_int && p != 1 as libc::c_int {
                    return stbi__err(
                        b"Corrupt JPEG\0" as *const u8 as *const libc::c_char,
                    );
                }
                if t > 3 as libc::c_int {
                    return stbi__err(
                        b"Corrupt JPEG\0" as *const u8 as *const libc::c_char,
                    );
                }
                i = 0 as libc::c_int;
                while i < 64 as libc::c_int {
                    (*z)
                        .dequant[t
                        as usize][stbi__jpeg_dezigzag[i as usize]
                        as usize] = (if sixteen != 0 {
                        stbi__get16be((*z).s)
                    } else {
                        stbi__get8((*z).s) as libc::c_int
                    }) as stbi__uint16;
                    i += 1;
                    i;
                }
                L -= if sixteen != 0 { 129 as libc::c_int } else { 65 as libc::c_int };
            }
            return (L == 0 as libc::c_int) as libc::c_int;
        }
        196 => {
            L = stbi__get16be((*z).s) - 2 as libc::c_int;
            while L > 0 as libc::c_int {
                let mut v: *mut stbi_uc = 0 as *mut stbi_uc;
                let mut sizes: [libc::c_int; 16] = [0; 16];
                let mut i_0: libc::c_int = 0;
                let mut n: libc::c_int = 0 as libc::c_int;
                let mut q_0: libc::c_int = stbi__get8((*z).s) as libc::c_int;
                let mut tc: libc::c_int = q_0 >> 4 as libc::c_int;
                let mut th: libc::c_int = q_0 & 15 as libc::c_int;
                if tc > 1 as libc::c_int || th > 3 as libc::c_int {
                    return stbi__err(
                        b"Corrupt JPEG\0" as *const u8 as *const libc::c_char,
                    );
                }
                i_0 = 0 as libc::c_int;
                while i_0 < 16 as libc::c_int {
                    sizes[i_0 as usize] = stbi__get8((*z).s) as libc::c_int;
                    n += sizes[i_0 as usize];
                    i_0 += 1;
                    i_0;
                }
                L -= 17 as libc::c_int;
                if tc == 0 as libc::c_int {
                    if stbi__build_huffman(
                        ((*z).huff_dc).as_mut_ptr().offset(th as isize),
                        sizes.as_mut_ptr(),
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    v = ((*z).huff_dc[th as usize].values).as_mut_ptr();
                } else {
                    if stbi__build_huffman(
                        ((*z).huff_ac).as_mut_ptr().offset(th as isize),
                        sizes.as_mut_ptr(),
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    v = ((*z).huff_ac[th as usize].values).as_mut_ptr();
                }
                i_0 = 0 as libc::c_int;
                while i_0 < n {
                    *v.offset(i_0 as isize) = stbi__get8((*z).s);
                    i_0 += 1;
                    i_0;
                }
                if tc != 0 as libc::c_int {
                    stbi__build_fast_ac(
                        ((*z).fast_ac[th as usize]).as_mut_ptr(),
                        ((*z).huff_ac).as_mut_ptr().offset(th as isize),
                    );
                }
                L -= n;
            }
            return (L == 0 as libc::c_int) as libc::c_int;
        }
        _ => {}
    }
    if m >= 0xe0 as libc::c_int && m <= 0xef as libc::c_int || m == 0xfe as libc::c_int {
        L = stbi__get16be((*z).s);
        if L < 2 as libc::c_int {
            if m == 0xfe as libc::c_int {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char)
            } else {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char)
            }
        }
        L -= 2 as libc::c_int;
        if m == 0xe0 as libc::c_int && L >= 5 as libc::c_int {
            static mut tag: [libc::c_uchar; 5] = [
                'J' as i32 as libc::c_uchar,
                'F' as i32 as libc::c_uchar,
                'I' as i32 as libc::c_uchar,
                'F' as i32 as libc::c_uchar,
                '\0' as i32 as libc::c_uchar,
            ];
            let mut ok: libc::c_int = 1 as libc::c_int;
            let mut i_1: libc::c_int = 0;
            i_1 = 0 as libc::c_int;
            while i_1 < 5 as libc::c_int {
                if stbi__get8((*z).s) as libc::c_int != tag[i_1 as usize] as libc::c_int
                {
                    ok = 0 as libc::c_int;
                }
                i_1 += 1;
                i_1;
            }
            L -= 5 as libc::c_int;
            if ok != 0 {
                (*z).jfif = 1 as libc::c_int;
            }
        } else if m == 0xee as libc::c_int && L >= 12 as libc::c_int {
            static mut tag_0: [libc::c_uchar; 6] = [
                'A' as i32 as libc::c_uchar,
                'd' as i32 as libc::c_uchar,
                'o' as i32 as libc::c_uchar,
                'b' as i32 as libc::c_uchar,
                'e' as i32 as libc::c_uchar,
                '\0' as i32 as libc::c_uchar,
            ];
            let mut ok_0: libc::c_int = 1 as libc::c_int;
            let mut i_2: libc::c_int = 0;
            i_2 = 0 as libc::c_int;
            while i_2 < 6 as libc::c_int {
                if stbi__get8((*z).s) as libc::c_int
                    != tag_0[i_2 as usize] as libc::c_int
                {
                    ok_0 = 0 as libc::c_int;
                }
                i_2 += 1;
                i_2;
            }
            L -= 6 as libc::c_int;
            if ok_0 != 0 {
                stbi__get8((*z).s);
                stbi__get16be((*z).s);
                stbi__get16be((*z).s);
                (*z).app14_color_transform = stbi__get8((*z).s) as libc::c_int;
                L -= 6 as libc::c_int;
            }
        }
        stbi__skip((*z).s, L);
        return 1 as libc::c_int;
    }
    return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn stbi__get_marker(mut j: *mut stbi__jpeg) -> stbi_uc {
    let mut x: stbi_uc = 0;
    if (*j).marker as libc::c_int != 0xff as libc::c_int {
        x = (*j).marker;
        (*j).marker = 0xff as libc::c_int as libc::c_uchar;
        return x;
    }
    x = stbi__get8((*j).s);
    if x as libc::c_int != 0xff as libc::c_int {
        return 0xff as libc::c_int as stbi_uc;
    }
    while x as libc::c_int == 0xff as libc::c_int {
        x = stbi__get8((*j).s);
    }
    return x;
}
unsafe extern "C" fn stbi__jpeg_dequantize(
    mut data: *mut libc::c_short,
    mut dequant: *mut stbi__uint16,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        let ref mut fresh79 = *data.offset(i as isize);
        *fresh79 = (*fresh79 as libc::c_int * *dequant.offset(i as isize) as libc::c_int)
            as libc::c_short;
        i += 1;
        i;
    }
}
unsafe extern "C" fn stbi__jpeg_finish(mut z: *mut stbi__jpeg) {
    if (*z).progressive != 0 {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut n: libc::c_int = 0;
        n = 0 as libc::c_int;
        while n < (*(*z).s).img_n {
            let mut w: libc::c_int = (*z).img_comp[n as usize].x + 7 as libc::c_int
                >> 3 as libc::c_int;
            let mut h: libc::c_int = (*z).img_comp[n as usize].y + 7 as libc::c_int
                >> 3 as libc::c_int;
            j = 0 as libc::c_int;
            while j < h {
                i = 0 as libc::c_int;
                while i < w {
                    let mut data: *mut libc::c_short = ((*z).img_comp[n as usize].coeff)
                        .offset(
                            (64 as libc::c_int
                                * (i + j * (*z).img_comp[n as usize].coeff_w)) as isize,
                        );
                    stbi__jpeg_dequantize(
                        data,
                        ((*z).dequant[(*z).img_comp[n as usize].tq as usize])
                            .as_mut_ptr(),
                    );
                    ((*z).idct_block_kernel)
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*z).img_comp[n as usize].data)
                            .offset(
                                ((*z).img_comp[n as usize].w2 * j * 8 as libc::c_int)
                                    as isize,
                            )
                            .offset((i * 8 as libc::c_int) as isize),
                        (*z).img_comp[n as usize].w2,
                        data,
                    );
                    i += 1;
                    i;
                }
                j += 1;
                j;
            }
            n += 1;
            n;
        }
    }
}
unsafe extern "C" fn stbi__decode_jpeg_image(mut j: *mut stbi__jpeg) -> libc::c_int {
    let mut m: libc::c_int = 0;
    m = 0 as libc::c_int;
    while m < 4 as libc::c_int {
        (*j).img_comp[m as usize].raw_data = 0 as *mut libc::c_void;
        (*j).img_comp[m as usize].raw_coeff = 0 as *mut libc::c_void;
        m += 1;
        m;
    }
    (*j).restart_interval = 0 as libc::c_int;
    if stbi__decode_jpeg_header(j, STBI__SCAN_load as libc::c_int) == 0 {
        return 0 as libc::c_int;
    }
    m = stbi__get_marker(j) as libc::c_int;
    while !(m == 0xd9 as libc::c_int) {
        if m == 0xda as libc::c_int {
            if stbi__process_scan_header(j) == 0 {
                return 0 as libc::c_int;
            }
            if stbi__parse_entropy_coded_data(j) == 0 {
                return 0 as libc::c_int;
            }
            if (*j).marker as libc::c_int == 0xff as libc::c_int {
                while stbi__at_eof((*j).s) == 0 {
                    let mut x: libc::c_int = stbi__get8((*j).s) as libc::c_int;
                    if !(x == 255 as libc::c_int) {
                        continue;
                    }
                    (*j).marker = stbi__get8((*j).s);
                    break;
                }
            }
        } else if m == 0xdc as libc::c_int {
            let mut Ld: libc::c_int = stbi__get16be((*j).s);
            let mut NL: stbi__uint32 = stbi__get16be((*j).s) as stbi__uint32;
            if Ld != 4 as libc::c_int {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
            }
            if NL != (*(*j).s).img_y {
                return stbi__err(b"Corrupt JPEG\0" as *const u8 as *const libc::c_char);
            }
        } else if stbi__process_marker(j, m) == 0 {
            return 0 as libc::c_int
        }
        m = stbi__get_marker(j) as libc::c_int;
    }
    if (*j).progressive != 0 {
        stbi__jpeg_finish(j);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn resample_row_1(
    mut out: *mut stbi_uc,
    mut in_near: *mut stbi_uc,
    mut in_far: *mut stbi_uc,
    mut w: libc::c_int,
    mut hs: libc::c_int,
) -> *mut stbi_uc {
    return in_near;
}
unsafe extern "C" fn stbi__resample_row_v_2(
    mut out: *mut stbi_uc,
    mut in_near: *mut stbi_uc,
    mut in_far: *mut stbi_uc,
    mut w: libc::c_int,
    mut hs: libc::c_int,
) -> *mut stbi_uc {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < w {
        *out
            .offset(
                i as isize,
            ) = (3 as libc::c_int * *in_near.offset(i as isize) as libc::c_int
            + *in_far.offset(i as isize) as libc::c_int + 2 as libc::c_int
            >> 2 as libc::c_int) as stbi_uc;
        i += 1;
        i;
    }
    return out;
}
unsafe extern "C" fn stbi__resample_row_h_2(
    mut out: *mut stbi_uc,
    mut in_near: *mut stbi_uc,
    mut in_far: *mut stbi_uc,
    mut w: libc::c_int,
    mut hs: libc::c_int,
) -> *mut stbi_uc {
    let mut i: libc::c_int = 0;
    let mut input: *mut stbi_uc = in_near;
    if w == 1 as libc::c_int {
        let ref mut fresh80 = *out.offset(1 as libc::c_int as isize);
        *fresh80 = *input.offset(0 as libc::c_int as isize);
        *out.offset(0 as libc::c_int as isize) = *fresh80;
        return out;
    }
    *out.offset(0 as libc::c_int as isize) = *input.offset(0 as libc::c_int as isize);
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = (*input.offset(0 as libc::c_int as isize) as libc::c_int * 3 as libc::c_int
        + *input.offset(1 as libc::c_int as isize) as libc::c_int + 2 as libc::c_int
        >> 2 as libc::c_int) as stbi_uc;
    i = 1 as libc::c_int;
    while i < w - 1 as libc::c_int {
        let mut n: libc::c_int = 3 as libc::c_int
            * *input.offset(i as isize) as libc::c_int + 2 as libc::c_int;
        *out
            .offset(
                (i * 2 as libc::c_int + 0 as libc::c_int) as isize,
            ) = (n + *input.offset((i - 1 as libc::c_int) as isize) as libc::c_int
            >> 2 as libc::c_int) as stbi_uc;
        *out
            .offset(
                (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
            ) = (n + *input.offset((i + 1 as libc::c_int) as isize) as libc::c_int
            >> 2 as libc::c_int) as stbi_uc;
        i += 1;
        i;
    }
    *out
        .offset(
            (i * 2 as libc::c_int + 0 as libc::c_int) as isize,
        ) = (*input.offset((w - 2 as libc::c_int) as isize) as libc::c_int
        * 3 as libc::c_int
        + *input.offset((w - 1 as libc::c_int) as isize) as libc::c_int
        + 2 as libc::c_int >> 2 as libc::c_int) as stbi_uc;
    *out
        .offset(
            (i * 2 as libc::c_int + 1 as libc::c_int) as isize,
        ) = *input.offset((w - 1 as libc::c_int) as isize);
    return out;
}
unsafe extern "C" fn stbi__resample_row_generic(
    mut out: *mut stbi_uc,
    mut in_near: *mut stbi_uc,
    mut in_far: *mut stbi_uc,
    mut w: libc::c_int,
    mut hs: libc::c_int,
) -> *mut stbi_uc {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < w {
        j = 0 as libc::c_int;
        while j < hs {
            *out.offset((i * hs + j) as isize) = *in_near.offset(i as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return out;
}
unsafe extern "C" fn stbi__blinn_8x8(mut x: stbi_uc, mut y: stbi_uc) -> stbi_uc {
    let mut t: libc::c_uint = (x as libc::c_int * y as libc::c_int + 128 as libc::c_int)
        as libc::c_uint;
    return (t.wrapping_add(t >> 8 as libc::c_int) >> 8 as libc::c_int) as stbi_uc;
}
unsafe extern "C" fn stbi__free_jpeg_components(
    mut z: *mut stbi__jpeg,
    mut ncomp: libc::c_int,
    mut why: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < ncomp {
        if !((*z).img_comp[i as usize].raw_data).is_null() {
            stbi_free((*z).img_comp[i as usize].raw_data);
            (*z).img_comp[i as usize].raw_data = 0 as *mut libc::c_void;
            (*z).img_comp[i as usize].data = 0 as *mut stbi_uc;
        }
        if !((*z).img_comp[i as usize].raw_coeff).is_null() {
            stbi_free((*z).img_comp[i as usize].raw_coeff);
            (*z).img_comp[i as usize].raw_coeff = 0 as *mut libc::c_void;
            (*z).img_comp[i as usize].coeff = 0 as *mut libc::c_short;
        }
        if !((*z).img_comp[i as usize].linebuf).is_null() {
            stbi_free((*z).img_comp[i as usize].linebuf as *mut libc::c_void);
            (*z).img_comp[i as usize].linebuf = 0 as *mut stbi_uc;
        }
        i += 1;
        i;
    }
    return why;
}
unsafe extern "C" fn stbi__cleanup_jpeg(mut j: *mut stbi__jpeg) {
    stbi__free_jpeg_components(j, (*(*j).s).img_n, 0 as libc::c_int);
}
unsafe extern "C" fn load_jpeg_image(
    mut z: *mut stbi__jpeg,
    mut out_x: *mut libc::c_int,
    mut out_y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut stbi_uc {
    let mut n: libc::c_int = 0;
    let mut decode_n: libc::c_int = 0;
    let mut is_rgb: libc::c_int = 0;
    (*(*z).s).img_n = 0 as libc::c_int;
    if req_comp < 0 as libc::c_int || req_comp > 4 as libc::c_int {
        return (if stbi__err(b"Internal error\0" as *const u8 as *const libc::c_char)
            != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar;
    }
    if stbi__decode_jpeg_image(z) == 0 {
        stbi__cleanup_jpeg(z);
        return 0 as *mut stbi_uc;
    }
    n = if req_comp != 0 {
        req_comp
    } else if (*(*z).s).img_n >= 3 as libc::c_int {
        3 as libc::c_int
    } else {
        1 as libc::c_int
    };
    is_rgb = ((*(*z).s).img_n == 3 as libc::c_int
        && ((*z).rgb == 3 as libc::c_int
            || (*z).app14_color_transform == 0 as libc::c_int && (*z).jfif == 0))
        as libc::c_int;
    if (*(*z).s).img_n == 3 as libc::c_int && n < 3 as libc::c_int && is_rgb == 0 {
        decode_n = 1 as libc::c_int;
    } else {
        decode_n = (*(*z).s).img_n;
    }
    if decode_n <= 0 as libc::c_int {
        stbi__cleanup_jpeg(z);
        return 0 as *mut stbi_uc;
    }
    let mut k: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut output: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut coutput: [*mut stbi_uc; 4] = [
        0 as *mut stbi_uc,
        0 as *mut stbi_uc,
        0 as *mut stbi_uc,
        0 as *mut stbi_uc,
    ];
    let mut res_comp: [stbi__resample; 4] = [stbi__resample {
        resample: None,
        line0: 0 as *mut stbi_uc,
        line1: 0 as *mut stbi_uc,
        hs: 0,
        vs: 0,
        w_lores: 0,
        ystep: 0,
        ypos: 0,
    }; 4];
    k = 0 as libc::c_int;
    while k < decode_n {
        let mut r: *mut stbi__resample = &mut *res_comp.as_mut_ptr().offset(k as isize)
            as *mut stbi__resample;
        (*z)
            .img_comp[k as usize]
            .linebuf = stbi__malloc(
            ((*(*z).s).img_x).wrapping_add(3 as libc::c_int as libc::c_uint) as size_t,
        ) as *mut stbi_uc;
        if ((*z).img_comp[k as usize].linebuf).is_null() {
            stbi__cleanup_jpeg(z);
            return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char)
                != 0
            {
                0 as *mut libc::c_void
            } else {
                0 as *mut libc::c_void
            }) as size_t as *mut libc::c_uchar;
        }
        (*r).hs = (*z).img_h_max / (*z).img_comp[k as usize].h;
        (*r).vs = (*z).img_v_max / (*z).img_comp[k as usize].v;
        (*r).ystep = (*r).vs >> 1 as libc::c_int;
        (*r)
            .w_lores = ((*(*z).s).img_x)
            .wrapping_add((*r).hs as libc::c_uint)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_div((*r).hs as libc::c_uint) as libc::c_int;
        (*r).ypos = 0 as libc::c_int;
        (*r).line1 = (*z).img_comp[k as usize].data;
        (*r).line0 = (*r).line1;
        if (*r).hs == 1 as libc::c_int && (*r).vs == 1 as libc::c_int {
            (*r)
                .resample = Some(
                resample_row_1
                    as unsafe extern "C" fn(
                        *mut stbi_uc,
                        *mut stbi_uc,
                        *mut stbi_uc,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut stbi_uc,
            );
        } else if (*r).hs == 1 as libc::c_int && (*r).vs == 2 as libc::c_int {
            (*r)
                .resample = Some(
                stbi__resample_row_v_2
                    as unsafe extern "C" fn(
                        *mut stbi_uc,
                        *mut stbi_uc,
                        *mut stbi_uc,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut stbi_uc,
            );
        } else if (*r).hs == 2 as libc::c_int && (*r).vs == 1 as libc::c_int {
            (*r)
                .resample = Some(
                stbi__resample_row_h_2
                    as unsafe extern "C" fn(
                        *mut stbi_uc,
                        *mut stbi_uc,
                        *mut stbi_uc,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut stbi_uc,
            );
        } else if (*r).hs == 2 as libc::c_int && (*r).vs == 2 as libc::c_int {
            (*r).resample = (*z).resample_row_hv_2_kernel;
        } else {
            (*r)
                .resample = Some(
                stbi__resample_row_generic
                    as unsafe extern "C" fn(
                        *mut stbi_uc,
                        *mut stbi_uc,
                        *mut stbi_uc,
                        libc::c_int,
                        libc::c_int,
                    ) -> *mut stbi_uc,
            );
        }
        k += 1;
        k;
    }
    output = stbi__malloc_mad3(
        n,
        (*(*z).s).img_x as libc::c_int,
        (*(*z).s).img_y as libc::c_int,
        1 as libc::c_int,
    ) as *mut stbi_uc;
    if output.is_null() {
        stbi__cleanup_jpeg(z);
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar;
    }
    j = 0 as libc::c_int as libc::c_uint;
    while j < (*(*z).s).img_y {
        let mut out: *mut stbi_uc = output
            .offset(
                (n as libc::c_uint).wrapping_mul((*(*z).s).img_x).wrapping_mul(j)
                    as isize,
            );
        k = 0 as libc::c_int;
        while k < decode_n {
            let mut r_0: *mut stbi__resample = &mut *res_comp
                .as_mut_ptr()
                .offset(k as isize) as *mut stbi__resample;
            let mut y_bot: libc::c_int = ((*r_0).ystep >= (*r_0).vs >> 1 as libc::c_int)
                as libc::c_int;
            coutput[k
                as usize] = ((*r_0).resample)
                .expect(
                    "non-null function pointer",
                )(
                (*z).img_comp[k as usize].linebuf,
                if y_bot != 0 { (*r_0).line1 } else { (*r_0).line0 },
                if y_bot != 0 { (*r_0).line0 } else { (*r_0).line1 },
                (*r_0).w_lores,
                (*r_0).hs,
            );
            (*r_0).ystep += 1;
            if (*r_0).ystep >= (*r_0).vs {
                (*r_0).ystep = 0 as libc::c_int;
                (*r_0).line0 = (*r_0).line1;
                (*r_0).ypos += 1;
                if (*r_0).ypos < (*z).img_comp[k as usize].y {
                    (*r_0)
                        .line1 = ((*r_0).line1)
                        .offset((*z).img_comp[k as usize].w2 as isize);
                }
            }
            k += 1;
            k;
        }
        if n >= 3 as libc::c_int {
            let mut y: *mut stbi_uc = coutput[0 as libc::c_int as usize];
            if (*(*z).s).img_n == 3 as libc::c_int {
                if is_rgb != 0 {
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < (*(*z).s).img_x {
                        *out.offset(0 as libc::c_int as isize) = *y.offset(i as isize);
                        *out
                            .offset(
                                1 as libc::c_int as isize,
                            ) = *(coutput[1 as libc::c_int as usize]).offset(i as isize);
                        *out
                            .offset(
                                2 as libc::c_int as isize,
                            ) = *(coutput[2 as libc::c_int as usize]).offset(i as isize);
                        *out
                            .offset(
                                3 as libc::c_int as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        out = out.offset(n as isize);
                        i = i.wrapping_add(1);
                        i;
                    }
                } else {
                    ((*z).YCbCr_to_RGB_kernel)
                        .expect(
                            "non-null function pointer",
                        )(
                        out,
                        y,
                        coutput[1 as libc::c_int as usize],
                        coutput[2 as libc::c_int as usize],
                        (*(*z).s).img_x as libc::c_int,
                        n,
                    );
                }
            } else if (*(*z).s).img_n == 4 as libc::c_int {
                if (*z).app14_color_transform == 0 as libc::c_int {
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < (*(*z).s).img_x {
                        let mut m: stbi_uc = *(coutput[3 as libc::c_int as usize])
                            .offset(i as isize);
                        *out
                            .offset(
                                0 as libc::c_int as isize,
                            ) = stbi__blinn_8x8(
                            *(coutput[0 as libc::c_int as usize]).offset(i as isize),
                            m,
                        );
                        *out
                            .offset(
                                1 as libc::c_int as isize,
                            ) = stbi__blinn_8x8(
                            *(coutput[1 as libc::c_int as usize]).offset(i as isize),
                            m,
                        );
                        *out
                            .offset(
                                2 as libc::c_int as isize,
                            ) = stbi__blinn_8x8(
                            *(coutput[2 as libc::c_int as usize]).offset(i as isize),
                            m,
                        );
                        *out
                            .offset(
                                3 as libc::c_int as isize,
                            ) = 255 as libc::c_int as stbi_uc;
                        out = out.offset(n as isize);
                        i = i.wrapping_add(1);
                        i;
                    }
                } else if (*z).app14_color_transform == 2 as libc::c_int {
                    ((*z).YCbCr_to_RGB_kernel)
                        .expect(
                            "non-null function pointer",
                        )(
                        out,
                        y,
                        coutput[1 as libc::c_int as usize],
                        coutput[2 as libc::c_int as usize],
                        (*(*z).s).img_x as libc::c_int,
                        n,
                    );
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < (*(*z).s).img_x {
                        let mut m_0: stbi_uc = *(coutput[3 as libc::c_int as usize])
                            .offset(i as isize);
                        *out
                            .offset(
                                0 as libc::c_int as isize,
                            ) = stbi__blinn_8x8(
                            (255 as libc::c_int
                                - *out.offset(0 as libc::c_int as isize) as libc::c_int)
                                as stbi_uc,
                            m_0,
                        );
                        *out
                            .offset(
                                1 as libc::c_int as isize,
                            ) = stbi__blinn_8x8(
                            (255 as libc::c_int
                                - *out.offset(1 as libc::c_int as isize) as libc::c_int)
                                as stbi_uc,
                            m_0,
                        );
                        *out
                            .offset(
                                2 as libc::c_int as isize,
                            ) = stbi__blinn_8x8(
                            (255 as libc::c_int
                                - *out.offset(2 as libc::c_int as isize) as libc::c_int)
                                as stbi_uc,
                            m_0,
                        );
                        out = out.offset(n as isize);
                        i = i.wrapping_add(1);
                        i;
                    }
                } else {
                    ((*z).YCbCr_to_RGB_kernel)
                        .expect(
                            "non-null function pointer",
                        )(
                        out,
                        y,
                        coutput[1 as libc::c_int as usize],
                        coutput[2 as libc::c_int as usize],
                        (*(*z).s).img_x as libc::c_int,
                        n,
                    );
                }
            } else {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*z).s).img_x {
                    let ref mut fresh81 = *out.offset(2 as libc::c_int as isize);
                    *fresh81 = *y.offset(i as isize);
                    let ref mut fresh82 = *out.offset(1 as libc::c_int as isize);
                    *fresh82 = *fresh81;
                    *out.offset(0 as libc::c_int as isize) = *fresh82;
                    *out
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 255 as libc::c_int as stbi_uc;
                    out = out.offset(n as isize);
                    i = i.wrapping_add(1);
                    i;
                }
            }
        } else if is_rgb != 0 {
            if n == 1 as libc::c_int {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*z).s).img_x {
                    let fresh83 = out;
                    out = out.offset(1);
                    *fresh83 = stbi__compute_y(
                        *(coutput[0 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int,
                        *(coutput[1 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int,
                        *(coutput[2 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int,
                    );
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*z).s).img_x {
                    *out
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y(
                        *(coutput[0 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int,
                        *(coutput[1 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int,
                        *(coutput[2 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int,
                    );
                    *out
                        .offset(
                            1 as libc::c_int as isize,
                        ) = 255 as libc::c_int as stbi_uc;
                    i = i.wrapping_add(1);
                    i;
                    out = out.offset(2 as libc::c_int as isize);
                }
            }
        } else if (*(*z).s).img_n == 4 as libc::c_int
            && (*z).app14_color_transform == 0 as libc::c_int
        {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*(*z).s).img_x {
                let mut m_1: stbi_uc = *(coutput[3 as libc::c_int as usize])
                    .offset(i as isize);
                let mut r_1: stbi_uc = stbi__blinn_8x8(
                    *(coutput[0 as libc::c_int as usize]).offset(i as isize),
                    m_1,
                );
                let mut g: stbi_uc = stbi__blinn_8x8(
                    *(coutput[1 as libc::c_int as usize]).offset(i as isize),
                    m_1,
                );
                let mut b: stbi_uc = stbi__blinn_8x8(
                    *(coutput[2 as libc::c_int as usize]).offset(i as isize),
                    m_1,
                );
                *out
                    .offset(
                        0 as libc::c_int as isize,
                    ) = stbi__compute_y(
                    r_1 as libc::c_int,
                    g as libc::c_int,
                    b as libc::c_int,
                );
                *out.offset(1 as libc::c_int as isize) = 255 as libc::c_int as stbi_uc;
                out = out.offset(n as isize);
                i = i.wrapping_add(1);
                i;
            }
        } else if (*(*z).s).img_n == 4 as libc::c_int
            && (*z).app14_color_transform == 2 as libc::c_int
        {
            i = 0 as libc::c_int as libc::c_uint;
            while i < (*(*z).s).img_x {
                *out
                    .offset(
                        0 as libc::c_int as isize,
                    ) = stbi__blinn_8x8(
                    (255 as libc::c_int
                        - *(coutput[0 as libc::c_int as usize]).offset(i as isize)
                            as libc::c_int) as stbi_uc,
                    *(coutput[3 as libc::c_int as usize]).offset(i as isize),
                );
                *out.offset(1 as libc::c_int as isize) = 255 as libc::c_int as stbi_uc;
                out = out.offset(n as isize);
                i = i.wrapping_add(1);
                i;
            }
        } else {
            let mut y_0: *mut stbi_uc = coutput[0 as libc::c_int as usize];
            if n == 1 as libc::c_int {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*z).s).img_x {
                    *out.offset(i as isize) = *y_0.offset(i as isize);
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                i = 0 as libc::c_int as libc::c_uint;
                while i < (*(*z).s).img_x {
                    let fresh84 = out;
                    out = out.offset(1);
                    *fresh84 = *y_0.offset(i as isize);
                    let fresh85 = out;
                    out = out.offset(1);
                    *fresh85 = 255 as libc::c_int as stbi_uc;
                    i = i.wrapping_add(1);
                    i;
                }
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    stbi__cleanup_jpeg(z);
    *out_x = (*(*z).s).img_x as libc::c_int;
    *out_y = (*(*z).s).img_y as libc::c_int;
    if !comp.is_null() {
        *comp = if (*(*z).s).img_n >= 3 as libc::c_int {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    return output;
}
unsafe extern "C" fn stbi__jpeg_load(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut j: *mut stbi__jpeg = stbi__malloc(
        ::core::mem::size_of::<stbi__jpeg>() as libc::c_ulong,
    ) as *mut stbi__jpeg;
    if j.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    (*j).s = s;
    stbi__setup_jpeg(j);
    result = load_jpeg_image(j, x, y, comp, req_comp);
    stbi_free(j as *mut libc::c_void);
    return result as *mut libc::c_void;
}
unsafe extern "C" fn stbi__hdr_test_core(
    mut s: *mut stbi__context,
    mut signature: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while *signature.offset(i as isize) != 0 {
        if stbi__get8(s) as libc::c_int != *signature.offset(i as isize) as libc::c_int {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    stbi__rewind(s);
    return 1 as libc::c_int;
}
unsafe extern "C" fn stbi__hdr_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut r: libc::c_int = stbi__hdr_test_core(
        s,
        b"#?RADIANCE\n\0" as *const u8 as *const libc::c_char,
    );
    stbi__rewind(s);
    if r == 0 {
        r = stbi__hdr_test_core(s, b"#?RGBE\n\0" as *const u8 as *const libc::c_char);
        stbi__rewind(s);
    }
    return r;
}
static mut stbi__h2l_scale_i: libc::c_float = 1.0f32;
static mut stbi__h2l_gamma_i: libc::c_float = 1.0f32 / 2.2f32;
unsafe extern "C" fn stbi__hdr_to_ldr(
    mut data: *mut libc::c_float,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut comp: libc::c_int,
) -> *mut stbi_uc {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut output: *mut stbi_uc = 0 as *mut stbi_uc;
    if data.is_null() {
        return 0 as *mut stbi_uc;
    }
    output = stbi__malloc_mad3(x, y, comp, 0 as libc::c_int) as *mut stbi_uc;
    if output.is_null() {
        stbi_free(data as *mut libc::c_void);
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar;
    }
    if comp & 1 as libc::c_int != 0 {
        n = comp;
    } else {
        n = comp - 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < x * y {
        k = 0 as libc::c_int;
        while k < n {
            let mut z: libc::c_float = pow(
                (*data.offset((i * comp + k) as isize) * stbi__h2l_scale_i)
                    as libc::c_double,
                stbi__h2l_gamma_i as libc::c_double,
            ) as libc::c_float * 255 as libc::c_int as libc::c_float + 0.5f32;
            if z < 0 as libc::c_int as libc::c_float {
                z = 0 as libc::c_int as libc::c_float;
            }
            if z > 255 as libc::c_int as libc::c_float {
                z = 255 as libc::c_int as libc::c_float;
            }
            *output.offset((i * comp + k) as isize) = z as libc::c_int as stbi_uc;
            k += 1;
            k;
        }
        if k < comp {
            let mut z_0: libc::c_float = *data.offset((i * comp + k) as isize)
                * 255 as libc::c_int as libc::c_float + 0.5f32;
            if z_0 < 0 as libc::c_int as libc::c_float {
                z_0 = 0 as libc::c_int as libc::c_float;
            }
            if z_0 > 255 as libc::c_int as libc::c_float {
                z_0 = 255 as libc::c_int as libc::c_float;
            }
            *output.offset((i * comp + k) as isize) = z_0 as libc::c_int as stbi_uc;
        }
        i += 1;
        i;
    }
    stbi_free(data as *mut libc::c_void);
    return output;
}
unsafe extern "C" fn stbi__at_eof(mut s: *mut stbi__context) -> libc::c_int {
    if ((*s).io.read).is_some() {
        if ((*s).io.eof).expect("non-null function pointer")((*s).io_user_data) == 0 {
            return 0 as libc::c_int;
        }
        if (*s).read_from_callbacks == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return ((*s).img_buffer >= (*s).img_buffer_end) as libc::c_int;
}
unsafe extern "C" fn stbi__hdr_gettoken(
    mut z: *mut stbi__context,
    mut buffer: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = '\0' as i32 as libc::c_char;
    c = stbi__get8(z) as libc::c_char;
    while stbi__at_eof(z) == 0 && c as libc::c_int != '\n' as i32 {
        let fresh86 = len;
        len = len + 1;
        *buffer.offset(fresh86 as isize) = c;
        if len == 1024 as libc::c_int - 1 as libc::c_int {
            while stbi__at_eof(z) == 0 && stbi__get8(z) as libc::c_int != '\n' as i32 {}
            break;
        } else {
            c = stbi__get8(z) as libc::c_char;
        }
    }
    *buffer.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    return buffer;
}
unsafe extern "C" fn stbi__mad4sizes_valid(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut d: libc::c_int,
    mut add: libc::c_int,
) -> libc::c_int {
    return (stbi__mul2sizes_valid(a, b) != 0 && stbi__mul2sizes_valid(a * b, c) != 0
        && stbi__mul2sizes_valid(a * b * c, d) != 0
        && stbi__addsizes_valid(a * b * c * d, add) != 0) as libc::c_int;
}
unsafe extern "C" fn stbi__malloc_mad4(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut d: libc::c_int,
    mut add: libc::c_int,
) -> *mut libc::c_void {
    if stbi__mad4sizes_valid(a, b, c, d, add) == 0 {
        return 0 as *mut libc::c_void;
    }
    return stbi__malloc((a * b * c * d + add) as size_t);
}
unsafe extern "C" fn stbi__hdr_convert(
    mut output: *mut libc::c_float,
    mut input: *mut stbi_uc,
    mut req_comp: libc::c_int,
) {
    if *input.offset(3 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int {
        let mut f1: libc::c_float = 0.;
        f1 = ldexp(
            1.0f32 as libc::c_double,
            *input.offset(3 as libc::c_int as isize) as libc::c_int
                - (128 as libc::c_int + 8 as libc::c_int),
        ) as libc::c_float;
        if req_comp <= 2 as libc::c_int {
            *output
                .offset(
                    0 as libc::c_int as isize,
                ) = (*input.offset(0 as libc::c_int as isize) as libc::c_int
                + *input.offset(1 as libc::c_int as isize) as libc::c_int
                + *input.offset(2 as libc::c_int as isize) as libc::c_int)
                as libc::c_float * f1 / 3 as libc::c_int as libc::c_float;
        } else {
            *output
                .offset(
                    0 as libc::c_int as isize,
                ) = *input.offset(0 as libc::c_int as isize) as libc::c_int
                as libc::c_float * f1;
            *output
                .offset(
                    1 as libc::c_int as isize,
                ) = *input.offset(1 as libc::c_int as isize) as libc::c_int
                as libc::c_float * f1;
            *output
                .offset(
                    2 as libc::c_int as isize,
                ) = *input.offset(2 as libc::c_int as isize) as libc::c_int
                as libc::c_float * f1;
        }
        if req_comp == 2 as libc::c_int {
            *output
                .offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_float;
        }
        if req_comp == 4 as libc::c_int {
            *output
                .offset(3 as libc::c_int as isize) = 1 as libc::c_int as libc::c_float;
        }
    } else {
        let mut current_block_15: u64;
        match req_comp {
            4 => {
                *output
                    .offset(
                        3 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_float;
                current_block_15 = 1109218150738618163;
            }
            3 => {
                current_block_15 = 1109218150738618163;
            }
            2 => {
                *output
                    .offset(
                        1 as libc::c_int as isize,
                    ) = 1 as libc::c_int as libc::c_float;
                current_block_15 = 14423536109209140104;
            }
            1 => {
                current_block_15 = 14423536109209140104;
            }
            _ => {
                current_block_15 = 17407779659766490442;
            }
        }
        match current_block_15 {
            1109218150738618163 => {
                let ref mut fresh87 = *output.offset(2 as libc::c_int as isize);
                *fresh87 = 0 as libc::c_int as libc::c_float;
                let ref mut fresh88 = *output.offset(1 as libc::c_int as isize);
                *fresh88 = *fresh87;
                *output.offset(0 as libc::c_int as isize) = *fresh88;
            }
            14423536109209140104 => {
                *output
                    .offset(
                        0 as libc::c_int as isize,
                    ) = 0 as libc::c_int as libc::c_float;
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn stbi__hdr_load(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_float {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valid: libc::c_int = 0 as libc::c_int;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut scanline: *mut stbi_uc = 0 as *mut stbi_uc;
    let mut hdr_data: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut len: libc::c_int = 0;
    let mut count: libc::c_uchar = 0;
    let mut value: libc::c_uchar = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut headerToken: *const libc::c_char = 0 as *const libc::c_char;
    headerToken = stbi__hdr_gettoken(s, buffer.as_mut_ptr());
    if strcmp(headerToken, b"#?RADIANCE\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
        && strcmp(headerToken, b"#?RGBE\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
    {
        return (if stbi__err(b"Corrupt HDR image\0" as *const u8 as *const libc::c_char)
            != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    loop {
        token = stbi__hdr_gettoken(s, buffer.as_mut_ptr());
        if *token.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            break;
        }
        if strcmp(token, b"FORMAT=32-bit_rle_rgbe\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            valid = 1 as libc::c_int;
        }
    }
    if valid == 0 {
        return (if stbi__err(
            b"Unsupported HDR format\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    token = stbi__hdr_gettoken(s, buffer.as_mut_ptr());
    if strncmp(
        token,
        b"-Y \0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return (if stbi__err(
            b"Unsupported HDR format\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    token = token.offset(3 as libc::c_int as isize);
    height = strtol(token, &mut token, 10 as libc::c_int) as libc::c_int;
    while *token as libc::c_int == ' ' as i32 {
        token = token.offset(1);
        token;
    }
    if strncmp(
        token,
        b"+X \0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        return (if stbi__err(
            b"Unsupported HDR format\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    token = token.offset(3 as libc::c_int as isize);
    width = strtol(token, 0 as *mut *mut libc::c_char, 10 as libc::c_int) as libc::c_int;
    if height > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    if width > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    *x = width;
    *y = height;
    if !comp.is_null() {
        *comp = 3 as libc::c_int;
    }
    if req_comp == 0 as libc::c_int {
        req_comp = 3 as libc::c_int;
    }
    if stbi__mad4sizes_valid(
        width,
        height,
        req_comp,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong as libc::c_int,
        0 as libc::c_int,
    ) == 0
    {
        return (if stbi__err(
            b"HDR image is too large\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    hdr_data = stbi__malloc_mad4(
        width,
        height,
        req_comp,
        ::core::mem::size_of::<libc::c_float>() as libc::c_ulong as libc::c_int,
        0 as libc::c_int,
    ) as *mut libc::c_float;
    if hdr_data.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_float;
    }
    's_445: {
        let mut rgbe: [stbi_uc; 4] = [0; 4];
        let mut current_block_99: u64;
        if width < 8 as libc::c_int || width >= 32768 as libc::c_int {
            j = 0 as libc::c_int;
            current_block_99 = 18377268871191777778;
        } else {
            scanline = 0 as *mut stbi_uc;
            j = 0 as libc::c_int;
            loop {
                if !(j < height) {
                    current_block_99 = 10213293998891106930;
                    break;
                }
                c1 = stbi__get8(s) as libc::c_int;
                c2 = stbi__get8(s) as libc::c_int;
                len = stbi__get8(s) as libc::c_int;
                if c1 != 2 as libc::c_int || c2 != 2 as libc::c_int
                    || len & 0x80 as libc::c_int != 0
                {
                    let mut rgbe_0: [stbi_uc; 4] = [0; 4];
                    rgbe_0[0 as libc::c_int as usize] = c1 as stbi_uc;
                    rgbe_0[1 as libc::c_int as usize] = c2 as stbi_uc;
                    rgbe_0[2 as libc::c_int as usize] = len as stbi_uc;
                    rgbe_0[3 as libc::c_int as usize] = stbi__get8(s);
                    stbi__hdr_convert(hdr_data, rgbe_0.as_mut_ptr(), req_comp);
                    i = 1 as libc::c_int;
                    j = 0 as libc::c_int;
                    stbi_free(scanline as *mut libc::c_void);
                    current_block_99 = 11398755241686030367;
                    break;
                } else {
                    len <<= 8 as libc::c_int;
                    len |= stbi__get8(s) as libc::c_int;
                    if len != width {
                        stbi_free(hdr_data as *mut libc::c_void);
                        stbi_free(scanline as *mut libc::c_void);
                        return (if stbi__err(
                            b"corrupt HDR\0" as *const u8 as *const libc::c_char,
                        ) != 0
                        {
                            0 as *mut libc::c_void
                        } else {
                            0 as *mut libc::c_void
                        }) as size_t as *mut libc::c_float;
                    }
                    if scanline.is_null() {
                        scanline = stbi__malloc_mad2(
                            width,
                            4 as libc::c_int,
                            0 as libc::c_int,
                        ) as *mut stbi_uc;
                        if scanline.is_null() {
                            stbi_free(hdr_data as *mut libc::c_void);
                            return (if stbi__err(
                                b"Out of memory\0" as *const u8 as *const libc::c_char,
                            ) != 0
                            {
                                0 as *mut libc::c_void
                            } else {
                                0 as *mut libc::c_void
                            }) as size_t as *mut libc::c_float;
                        }
                    }
                    k = 0 as libc::c_int;
                    while k < 4 as libc::c_int {
                        let mut nleft: libc::c_int = 0;
                        i = 0 as libc::c_int;
                        loop {
                            nleft = width - i;
                            if !(nleft > 0 as libc::c_int) {
                                break;
                            }
                            count = stbi__get8(s);
                            if count as libc::c_int > 128 as libc::c_int {
                                value = stbi__get8(s);
                                count = (count as libc::c_int - 128 as libc::c_int)
                                    as libc::c_uchar;
                                if count as libc::c_int > nleft {
                                    stbi_free(hdr_data as *mut libc::c_void);
                                    stbi_free(scanline as *mut libc::c_void);
                                    return (if stbi__err(
                                        b"bad RLE data in HDR\0" as *const u8 as *const libc::c_char,
                                    ) != 0
                                    {
                                        0 as *mut libc::c_void
                                    } else {
                                        0 as *mut libc::c_void
                                    }) as size_t as *mut libc::c_float;
                                }
                                z = 0 as libc::c_int;
                                while z < count as libc::c_int {
                                    let fresh89 = i;
                                    i = i + 1;
                                    *scanline
                                        .offset((fresh89 * 4 as libc::c_int + k) as isize) = value;
                                    z += 1;
                                    z;
                                }
                            } else {
                                if count as libc::c_int > nleft {
                                    stbi_free(hdr_data as *mut libc::c_void);
                                    stbi_free(scanline as *mut libc::c_void);
                                    return (if stbi__err(
                                        b"bad RLE data in HDR\0" as *const u8 as *const libc::c_char,
                                    ) != 0
                                    {
                                        0 as *mut libc::c_void
                                    } else {
                                        0 as *mut libc::c_void
                                    }) as size_t as *mut libc::c_float;
                                }
                                z = 0 as libc::c_int;
                                while z < count as libc::c_int {
                                    let fresh90 = i;
                                    i = i + 1;
                                    *scanline
                                        .offset(
                                            (fresh90 * 4 as libc::c_int + k) as isize,
                                        ) = stbi__get8(s);
                                    z += 1;
                                    z;
                                }
                            }
                        }
                        k += 1;
                        k;
                    }
                    i = 0 as libc::c_int;
                    while i < width {
                        stbi__hdr_convert(
                            hdr_data.offset(((j * width + i) * req_comp) as isize),
                            scanline.offset((i * 4 as libc::c_int) as isize),
                            req_comp,
                        );
                        i += 1;
                        i;
                    }
                    j += 1;
                    j;
                }
            }
            match current_block_99 {
                11398755241686030367 => {}
                _ => {
                    if !scanline.is_null() {
                        stbi_free(scanline as *mut libc::c_void);
                    }
                    current_block_99 = 12705158477165241210;
                }
            }
        }
        loop {
            match current_block_99 {
                12705158477165241210 => {
                    break 's_445;
                }
                18377268871191777778 => {
                    if !(j < height) {
                        current_block_99 = 12705158477165241210;
                        continue;
                    }
                    i = 0 as libc::c_int;
                }
                _ => {
                    stbi__getn(s, rgbe.as_mut_ptr(), 4 as libc::c_int);
                    stbi__hdr_convert(
                        hdr_data
                            .offset((j * width * req_comp) as isize)
                            .offset((i * req_comp) as isize),
                        rgbe.as_mut_ptr(),
                        req_comp,
                    );
                    i += 1;
                    i;
                }
            }
            if i < width {
                rgbe = [0; 4];
                current_block_99 = 11398755241686030367;
            } else {
                j += 1;
                j;
                current_block_99 = 18377268871191777778;
            }
        }
    }
    return hdr_data;
}
unsafe extern "C" fn stbi__rewind(mut s: *mut stbi__context) {
    (*s).img_buffer = (*s).img_buffer_original;
    (*s).img_buffer_end = (*s).img_buffer_original_end;
}
unsafe extern "C" fn stbi__tga_test(mut s: *mut stbi__context) -> libc::c_int {
    let mut current_block: u64;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut sz: libc::c_int = 0;
    let mut tga_color_type: libc::c_int = 0;
    stbi__get8(s);
    tga_color_type = stbi__get8(s) as libc::c_int;
    if !(tga_color_type > 1 as libc::c_int) {
        sz = stbi__get8(s) as libc::c_int;
        if tga_color_type == 1 as libc::c_int {
            if sz != 1 as libc::c_int && sz != 9 as libc::c_int {
                current_block = 13306728422825613545;
            } else {
                stbi__skip(s, 4 as libc::c_int);
                sz = stbi__get8(s) as libc::c_int;
                if sz != 8 as libc::c_int && sz != 15 as libc::c_int
                    && sz != 16 as libc::c_int && sz != 24 as libc::c_int
                    && sz != 32 as libc::c_int
                {
                    current_block = 13306728422825613545;
                } else {
                    stbi__skip(s, 4 as libc::c_int);
                    current_block = 13536709405535804910;
                }
            }
        } else if sz != 2 as libc::c_int && sz != 3 as libc::c_int
            && sz != 10 as libc::c_int && sz != 11 as libc::c_int
        {
            current_block = 13306728422825613545;
        } else {
            stbi__skip(s, 9 as libc::c_int);
            current_block = 13536709405535804910;
        }
        match current_block {
            13306728422825613545 => {}
            _ => {
                if !(stbi__get16le(s) < 1 as libc::c_int) {
                    if !(stbi__get16le(s) < 1 as libc::c_int) {
                        sz = stbi__get8(s) as libc::c_int;
                        if !(tga_color_type == 1 as libc::c_int && sz != 8 as libc::c_int
                            && sz != 16 as libc::c_int)
                        {
                            if !(sz != 8 as libc::c_int && sz != 15 as libc::c_int
                                && sz != 16 as libc::c_int && sz != 24 as libc::c_int
                                && sz != 32 as libc::c_int)
                            {
                                res = 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    stbi__rewind(s);
    return res;
}
unsafe extern "C" fn stbi__tga_get_comp(
    mut bits_per_pixel: libc::c_int,
    mut is_grey: libc::c_int,
    mut is_rgb16: *mut libc::c_int,
) -> libc::c_int {
    if !is_rgb16.is_null() {
        *is_rgb16 = 0 as libc::c_int;
    }
    match bits_per_pixel {
        8 => return STBI_grey as libc::c_int,
        16 => {
            if is_grey != 0 {
                return STBI_grey_alpha as libc::c_int;
            }
        }
        15 => {}
        24 | 32 => return bits_per_pixel / 8 as libc::c_int,
        _ => return 0 as libc::c_int,
    }
    if !is_rgb16.is_null() {
        *is_rgb16 = 1 as libc::c_int;
    }
    return STBI_rgb as libc::c_int;
}
unsafe extern "C" fn stbi__skip(mut s: *mut stbi__context, mut n: libc::c_int) {
    if n == 0 as libc::c_int {
        return;
    }
    if n < 0 as libc::c_int {
        (*s).img_buffer = (*s).img_buffer_end;
        return;
    }
    if ((*s).io.read).is_some() {
        let mut blen: libc::c_int = ((*s).img_buffer_end).offset_from((*s).img_buffer)
            as libc::c_long as libc::c_int;
        if blen < n {
            (*s).img_buffer = (*s).img_buffer_end;
            ((*s).io.skip)
                .expect("non-null function pointer")((*s).io_user_data, n - blen);
            return;
        }
    }
    (*s).img_buffer = ((*s).img_buffer).offset(n as isize);
}
unsafe extern "C" fn stbi__mad2sizes_valid(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut add: libc::c_int,
) -> libc::c_int {
    return (stbi__mul2sizes_valid(a, b) != 0 && stbi__addsizes_valid(a * b, add) != 0)
        as libc::c_int;
}
unsafe extern "C" fn stbi__malloc_mad2(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut add: libc::c_int,
) -> *mut libc::c_void {
    if stbi__mad2sizes_valid(a, b, add) == 0 {
        return 0 as *mut libc::c_void;
    }
    return stbi__malloc((a * b + add) as size_t);
}
unsafe extern "C" fn stbi__getn(
    mut s: *mut stbi__context,
    mut buffer: *mut stbi_uc,
    mut n: libc::c_int,
) -> libc::c_int {
    if ((*s).io.read).is_some() {
        let mut blen: libc::c_int = ((*s).img_buffer_end).offset_from((*s).img_buffer)
            as libc::c_long as libc::c_int;
        if blen < n {
            let mut res: libc::c_int = 0;
            let mut count: libc::c_int = 0;
            memcpy(
                buffer as *mut libc::c_void,
                (*s).img_buffer as *const libc::c_void,
                blen as libc::c_ulong,
            );
            count = ((*s).io.read)
                .expect(
                    "non-null function pointer",
                )(
                (*s).io_user_data,
                (buffer as *mut libc::c_char).offset(blen as isize),
                n - blen,
            );
            res = (count == n - blen) as libc::c_int;
            (*s).img_buffer = (*s).img_buffer_end;
            return res;
        }
    }
    if ((*s).img_buffer).offset(n as isize) <= (*s).img_buffer_end {
        memcpy(
            buffer as *mut libc::c_void,
            (*s).img_buffer as *const libc::c_void,
            n as libc::c_ulong,
        );
        (*s).img_buffer = ((*s).img_buffer).offset(n as isize);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn stbi__tga_read_rgb16(
    mut s: *mut stbi__context,
    mut out: *mut stbi_uc,
) {
    let mut px: stbi__uint16 = stbi__get16le(s) as stbi__uint16;
    let mut fiveBitMask: stbi__uint16 = 31 as libc::c_int as stbi__uint16;
    let mut r: libc::c_int = px as libc::c_int >> 10 as libc::c_int
        & fiveBitMask as libc::c_int;
    let mut g: libc::c_int = px as libc::c_int >> 5 as libc::c_int
        & fiveBitMask as libc::c_int;
    let mut b: libc::c_int = px as libc::c_int & fiveBitMask as libc::c_int;
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = (r * 255 as libc::c_int / 31 as libc::c_int) as stbi_uc;
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = (g * 255 as libc::c_int / 31 as libc::c_int) as stbi_uc;
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = (b * 255 as libc::c_int / 31 as libc::c_int) as stbi_uc;
}
unsafe extern "C" fn stbi__mul2sizes_valid(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_int {
    if a < 0 as libc::c_int || b < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if b == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return (a <= 2147483647 as libc::c_int / b) as libc::c_int;
}
unsafe extern "C" fn stbi__mad3sizes_valid(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut add: libc::c_int,
) -> libc::c_int {
    return (stbi__mul2sizes_valid(a, b) != 0 && stbi__mul2sizes_valid(a * b, c) != 0
        && stbi__addsizes_valid(a * b * c, add) != 0) as libc::c_int;
}
unsafe extern "C" fn stbi__malloc(mut size: size_t) -> *mut libc::c_void {
    return stbi_malloc(size);
}
unsafe extern "C" fn stbi__malloc_mad3(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
    mut add: libc::c_int,
) -> *mut libc::c_void {
    if stbi__mad3sizes_valid(a, b, c, add) == 0 {
        return 0 as *mut libc::c_void;
    }
    return stbi__malloc((a * b * c + add) as size_t);
}
unsafe extern "C" fn stbi__compute_y(
    mut r: libc::c_int,
    mut g: libc::c_int,
    mut b: libc::c_int,
) -> stbi_uc {
    return (r * 77 as libc::c_int + g * 150 as libc::c_int + 29 as libc::c_int * b
        >> 8 as libc::c_int) as stbi_uc;
}
unsafe extern "C" fn stbi__convert_format(
    mut data: *mut libc::c_uchar,
    mut img_n: libc::c_int,
    mut req_comp: libc::c_int,
    mut x: libc::c_uint,
    mut y: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut good: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if req_comp == img_n {
        return data;
    }
    if req_comp >= 1 as libc::c_int && req_comp <= 4 as libc::c_int {} else {
        __assert_fail(
            b"req_comp >= 1 && req_comp <= 4\0" as *const u8 as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            1741 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[libc::c_char; 91],
            >(
                b"unsigned char *stbi__convert_format(unsigned char *, int, int, unsigned int, unsigned int)\0",
            ))
                .as_ptr(),
        );
    };
    good = stbi__malloc_mad3(
        req_comp,
        x as libc::c_int,
        y as libc::c_int,
        0 as libc::c_int,
    ) as *mut libc::c_uchar;
    if good.is_null() {
        stbi_free(data as *mut libc::c_void);
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar;
    }
    j = 0 as libc::c_int;
    while j < y as libc::c_int {
        let mut src: *mut libc::c_uchar = data
            .offset(
                (j as libc::c_uint).wrapping_mul(x).wrapping_mul(img_n as libc::c_uint)
                    as isize,
            );
        let mut dest: *mut libc::c_uchar = good
            .offset(
                (j as libc::c_uint)
                    .wrapping_mul(x)
                    .wrapping_mul(req_comp as libc::c_uint) as isize,
            );
        match img_n * 8 as libc::c_int + req_comp {
            10 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = 255 as libc::c_int as libc::c_uchar;
                    i -= 1;
                    i;
                    src = src.offset(1 as libc::c_int as isize);
                    dest = dest.offset(2 as libc::c_int as isize);
                }
            }
            11 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh91 = *dest.offset(2 as libc::c_int as isize);
                    *fresh91 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh92 = *dest.offset(1 as libc::c_int as isize);
                    *fresh92 = *fresh91;
                    *dest.offset(0 as libc::c_int as isize) = *fresh92;
                    i -= 1;
                    i;
                    src = src.offset(1 as libc::c_int as isize);
                    dest = dest.offset(3 as libc::c_int as isize);
                }
            }
            12 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh93 = *dest.offset(2 as libc::c_int as isize);
                    *fresh93 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh94 = *dest.offset(1 as libc::c_int as isize);
                    *fresh94 = *fresh93;
                    *dest.offset(0 as libc::c_int as isize) = *fresh94;
                    *dest
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 255 as libc::c_int as libc::c_uchar;
                    i -= 1;
                    i;
                    src = src.offset(1 as libc::c_int as isize);
                    dest = dest.offset(4 as libc::c_int as isize);
                }
            }
            17 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(2 as libc::c_int as isize);
                    dest = dest.offset(1 as libc::c_int as isize);
                }
            }
            19 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh95 = *dest.offset(2 as libc::c_int as isize);
                    *fresh95 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh96 = *dest.offset(1 as libc::c_int as isize);
                    *fresh96 = *fresh95;
                    *dest.offset(0 as libc::c_int as isize) = *fresh96;
                    i -= 1;
                    i;
                    src = src.offset(2 as libc::c_int as isize);
                    dest = dest.offset(3 as libc::c_int as isize);
                }
            }
            20 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    let ref mut fresh97 = *dest.offset(2 as libc::c_int as isize);
                    *fresh97 = *src.offset(0 as libc::c_int as isize);
                    let ref mut fresh98 = *dest.offset(1 as libc::c_int as isize);
                    *fresh98 = *fresh97;
                    *dest.offset(0 as libc::c_int as isize) = *fresh98;
                    *dest
                        .offset(
                            3 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(2 as libc::c_int as isize);
                    dest = dest.offset(4 as libc::c_int as isize);
                }
            }
            28 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    *dest
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                    *dest
                        .offset(
                            3 as libc::c_int as isize,
                        ) = 255 as libc::c_int as libc::c_uchar;
                    i -= 1;
                    i;
                    src = src.offset(3 as libc::c_int as isize);
                    dest = dest.offset(4 as libc::c_int as isize);
                }
            }
            25 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    i -= 1;
                    i;
                    src = src.offset(3 as libc::c_int as isize);
                    dest = dest.offset(1 as libc::c_int as isize);
                }
            }
            26 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = 255 as libc::c_int as libc::c_uchar;
                    i -= 1;
                    i;
                    src = src.offset(3 as libc::c_int as isize);
                    dest = dest.offset(2 as libc::c_int as isize);
                }
            }
            33 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    i -= 1;
                    i;
                    src = src.offset(4 as libc::c_int as isize);
                    dest = dest.offset(1 as libc::c_int as isize);
                }
            }
            34 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = stbi__compute_y(
                        *src.offset(0 as libc::c_int as isize) as libc::c_int,
                        *src.offset(1 as libc::c_int as isize) as libc::c_int,
                        *src.offset(2 as libc::c_int as isize) as libc::c_int,
                    );
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(3 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(4 as libc::c_int as isize);
                    dest = dest.offset(2 as libc::c_int as isize);
                }
            }
            35 => {
                i = x.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
                while i >= 0 as libc::c_int {
                    *dest
                        .offset(
                            0 as libc::c_int as isize,
                        ) = *src.offset(0 as libc::c_int as isize);
                    *dest
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *src.offset(1 as libc::c_int as isize);
                    *dest
                        .offset(
                            2 as libc::c_int as isize,
                        ) = *src.offset(2 as libc::c_int as isize);
                    i -= 1;
                    i;
                    src = src.offset(4 as libc::c_int as isize);
                    dest = dest.offset(3 as libc::c_int as isize);
                }
            }
            _ => {
                if 0 as libc::c_int != 0 {} else {
                    __assert_fail(
                        b"0\0" as *const u8 as *const libc::c_char,
                        b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                        1770 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 91],
                            &[libc::c_char; 91],
                        >(
                            b"unsigned char *stbi__convert_format(unsigned char *, int, int, unsigned int, unsigned int)\0",
                        ))
                            .as_ptr(),
                    );
                };
                stbi_free(data as *mut libc::c_void);
                stbi_free(good as *mut libc::c_void);
                return (if stbi__err(
                    b"Unsupported format conversion\0" as *const u8
                        as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar;
            }
        }
        j += 1;
        j;
    }
    stbi_free(data as *mut libc::c_void);
    return good;
}
unsafe extern "C" fn stbi__refill_buffer(mut s: *mut stbi__context) {
    let mut n: libc::c_int = ((*s).io.read)
        .expect(
            "non-null function pointer",
        )(
        (*s).io_user_data,
        ((*s).buffer_start).as_mut_ptr() as *mut libc::c_char,
        (*s).buflen,
    );
    (*s).callback_already_read
        += ((*s).img_buffer).offset_from((*s).img_buffer_original) as libc::c_long
            as libc::c_int;
    if n == 0 as libc::c_int {
        (*s).read_from_callbacks = 0 as libc::c_int;
        (*s).img_buffer = ((*s).buffer_start).as_mut_ptr();
        (*s)
            .img_buffer_end = ((*s).buffer_start)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize);
        *(*s).img_buffer = 0 as libc::c_int as stbi_uc;
    } else {
        (*s).img_buffer = ((*s).buffer_start).as_mut_ptr();
        (*s).img_buffer_end = ((*s).buffer_start).as_mut_ptr().offset(n as isize);
    };
}
unsafe extern "C" fn stbi__get8(mut s: *mut stbi__context) -> stbi_uc {
    if (*s).img_buffer < (*s).img_buffer_end {
        let fresh99 = (*s).img_buffer;
        (*s).img_buffer = ((*s).img_buffer).offset(1);
        return *fresh99;
    }
    if (*s).read_from_callbacks != 0 {
        stbi__refill_buffer(s);
        let fresh100 = (*s).img_buffer;
        (*s).img_buffer = ((*s).img_buffer).offset(1);
        return *fresh100;
    }
    return 0 as libc::c_int as stbi_uc;
}
unsafe extern "C" fn stbi__get16le(mut s: *mut stbi__context) -> libc::c_int {
    let mut z: libc::c_int = stbi__get8(s) as libc::c_int;
    return z + ((stbi__get8(s) as libc::c_int) << 8 as libc::c_int);
}
unsafe extern "C" fn stbi__tga_load(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
) -> *mut libc::c_void {
    let mut tga_offset: libc::c_int = stbi__get8(s) as libc::c_int;
    let mut tga_indexed: libc::c_int = stbi__get8(s) as libc::c_int;
    let mut tga_image_type: libc::c_int = stbi__get8(s) as libc::c_int;
    let mut tga_is_RLE: libc::c_int = 0 as libc::c_int;
    let mut tga_palette_start: libc::c_int = stbi__get16le(s);
    let mut tga_palette_len: libc::c_int = stbi__get16le(s);
    let mut tga_palette_bits: libc::c_int = stbi__get8(s) as libc::c_int;
    let mut tga_x_origin: libc::c_int = stbi__get16le(s);
    let mut tga_y_origin: libc::c_int = stbi__get16le(s);
    let mut tga_width: libc::c_int = stbi__get16le(s);
    let mut tga_height: libc::c_int = stbi__get16le(s);
    let mut tga_bits_per_pixel: libc::c_int = stbi__get8(s) as libc::c_int;
    let mut tga_comp: libc::c_int = 0;
    let mut tga_rgb16: libc::c_int = 0 as libc::c_int;
    let mut tga_inverted: libc::c_int = stbi__get8(s) as libc::c_int;
    let mut tga_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tga_palette: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut raw_data: [libc::c_uchar; 4] = [0 as libc::c_int as libc::c_uchar, 0, 0, 0];
    let mut RLE_count: libc::c_int = 0 as libc::c_int;
    let mut RLE_repeating: libc::c_int = 0 as libc::c_int;
    let mut read_next_pixel: libc::c_int = 1 as libc::c_int;
    if tga_height > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if tga_width > (1 as libc::c_int) << 24 as libc::c_int {
        return (if stbi__err(
            b"Very large image (corrupt?)\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    if tga_image_type >= 8 as libc::c_int {
        tga_image_type -= 8 as libc::c_int;
        tga_is_RLE = 1 as libc::c_int;
    }
    tga_inverted = 1 as libc::c_int
        - (tga_inverted >> 5 as libc::c_int & 1 as libc::c_int);
    if tga_indexed != 0 {
        tga_comp = stbi__tga_get_comp(
            tga_palette_bits,
            0 as libc::c_int,
            &mut tga_rgb16,
        );
    } else {
        tga_comp = stbi__tga_get_comp(
            tga_bits_per_pixel,
            (tga_image_type == 3 as libc::c_int) as libc::c_int,
            &mut tga_rgb16,
        );
    }
    if tga_comp == 0 {
        return (if stbi__err(
            b"Can't find out TGA pixelformat\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    *x = tga_width;
    *y = tga_height;
    if !comp.is_null() {
        *comp = tga_comp;
    }
    if stbi__mad3sizes_valid(tga_width, tga_height, tga_comp, 0 as libc::c_int) == 0 {
        return (if stbi__err(b"Corrupt TGA\0" as *const u8 as *const libc::c_char) != 0 {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    tga_data = stbi__malloc_mad3(tga_width, tga_height, tga_comp, 0 as libc::c_int)
        as *mut libc::c_uchar;
    if tga_data.is_null() {
        return (if stbi__err(b"Out of memory\0" as *const u8 as *const libc::c_char) != 0
        {
            0 as *mut libc::c_void
        } else {
            0 as *mut libc::c_void
        }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
    }
    stbi__skip(s, tga_offset);
    if tga_indexed == 0 && tga_is_RLE == 0 && tga_rgb16 == 0 {
        i = 0 as libc::c_int;
        while i < tga_height {
            let mut row: libc::c_int = if tga_inverted != 0 {
                tga_height - i - 1 as libc::c_int
            } else {
                i
            };
            let mut tga_row: *mut stbi_uc = tga_data
                .offset((row * tga_width * tga_comp) as isize);
            stbi__getn(s, tga_row, tga_width * tga_comp);
            i += 1;
            i;
        }
    } else {
        if tga_indexed != 0 {
            if tga_palette_len == 0 as libc::c_int {
                stbi_free(tga_data as *mut libc::c_void);
                return (if stbi__err(
                    b"Corrupt TGA\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
            stbi__skip(s, tga_palette_start);
            tga_palette = stbi__malloc_mad2(tga_palette_len, tga_comp, 0 as libc::c_int)
                as *mut libc::c_uchar;
            if tga_palette.is_null() {
                stbi_free(tga_data as *mut libc::c_void);
                return (if stbi__err(
                    b"Out of memory\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
            if tga_rgb16 != 0 {
                let mut pal_entry: *mut stbi_uc = tga_palette;
                if tga_comp == STBI_rgb as libc::c_int {} else {
                    __assert_fail(
                        b"tga_comp == STBI_rgb\0" as *const u8 as *const libc::c_char,
                        b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                        5880 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 85],
                            &[libc::c_char; 85],
                        >(
                            b"void *stbi__tga_load(stbi__context *, int *, int *, int *, int, stbi__result_info *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                i = 0 as libc::c_int;
                while i < tga_palette_len {
                    stbi__tga_read_rgb16(s, pal_entry);
                    pal_entry = pal_entry.offset(tga_comp as isize);
                    i += 1;
                    i;
                }
            } else if stbi__getn(s, tga_palette, tga_palette_len * tga_comp) == 0 {
                stbi_free(tga_data as *mut libc::c_void);
                stbi_free(tga_palette as *mut libc::c_void);
                return (if stbi__err(
                    b"Corrupt TGA\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    0 as *mut libc::c_void
                } else {
                    0 as *mut libc::c_void
                }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
            }
        }
        i = 0 as libc::c_int;
        while i < tga_width * tga_height {
            if tga_is_RLE != 0 {
                if RLE_count == 0 as libc::c_int {
                    let mut RLE_cmd: libc::c_int = stbi__get8(s) as libc::c_int;
                    RLE_count = 1 as libc::c_int + (RLE_cmd & 127 as libc::c_int);
                    RLE_repeating = RLE_cmd >> 7 as libc::c_int;
                    read_next_pixel = 1 as libc::c_int;
                } else if RLE_repeating == 0 {
                    read_next_pixel = 1 as libc::c_int;
                }
            } else {
                read_next_pixel = 1 as libc::c_int;
            }
            if read_next_pixel != 0 {
                if tga_indexed != 0 {
                    let mut pal_idx: libc::c_int = if tga_bits_per_pixel
                        == 8 as libc::c_int
                    {
                        stbi__get8(s) as libc::c_int
                    } else {
                        stbi__get16le(s)
                    };
                    if pal_idx >= tga_palette_len {
                        pal_idx = 0 as libc::c_int;
                    }
                    pal_idx *= tga_comp;
                    j = 0 as libc::c_int;
                    while j < tga_comp {
                        raw_data[j
                            as usize] = *tga_palette.offset((pal_idx + j) as isize);
                        j += 1;
                        j;
                    }
                } else if tga_rgb16 != 0 {
                    if tga_comp == STBI_rgb as libc::c_int {} else {
                        __assert_fail(
                            b"tga_comp == STBI_rgb\0" as *const u8
                                as *const libc::c_char,
                            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
                            5929 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 85],
                                &[libc::c_char; 85],
                            >(
                                b"void *stbi__tga_load(stbi__context *, int *, int *, int *, int, stbi__result_info *)\0",
                            ))
                                .as_ptr(),
                        );
                    };
                    stbi__tga_read_rgb16(s, raw_data.as_mut_ptr());
                } else {
                    j = 0 as libc::c_int;
                    while j < tga_comp {
                        raw_data[j as usize] = stbi__get8(s);
                        j += 1;
                        j;
                    }
                }
                read_next_pixel = 0 as libc::c_int;
            }
            j = 0 as libc::c_int;
            while j < tga_comp {
                *tga_data.offset((i * tga_comp + j) as isize) = raw_data[j as usize];
                j += 1;
                j;
            }
            RLE_count -= 1;
            RLE_count;
            i += 1;
            i;
        }
        if tga_inverted != 0 {
            j = 0 as libc::c_int;
            while (j * 2 as libc::c_int) < tga_height {
                let mut index1: libc::c_int = j * tga_width * tga_comp;
                let mut index2: libc::c_int = (tga_height - 1 as libc::c_int - j)
                    * tga_width * tga_comp;
                i = tga_width * tga_comp;
                while i > 0 as libc::c_int {
                    let mut temp: libc::c_uchar = *tga_data.offset(index1 as isize);
                    *tga_data
                        .offset(index1 as isize) = *tga_data.offset(index2 as isize);
                    *tga_data.offset(index2 as isize) = temp;
                    index1 += 1;
                    index1;
                    index2 += 1;
                    index2;
                    i -= 1;
                    i;
                }
                j += 1;
                j;
            }
        }
        if !tga_palette.is_null() {
            stbi_free(tga_palette as *mut libc::c_void);
        }
    }
    if tga_comp >= 3 as libc::c_int && tga_rgb16 == 0 {
        let mut tga_pixel: *mut libc::c_uchar = tga_data;
        i = 0 as libc::c_int;
        while i < tga_width * tga_height {
            let mut temp_0: libc::c_uchar = *tga_pixel.offset(0 as libc::c_int as isize);
            *tga_pixel
                .offset(
                    0 as libc::c_int as isize,
                ) = *tga_pixel.offset(2 as libc::c_int as isize);
            *tga_pixel.offset(2 as libc::c_int as isize) = temp_0;
            tga_pixel = tga_pixel.offset(tga_comp as isize);
            i += 1;
            i;
        }
    }
    if req_comp != 0 && req_comp != tga_comp {
        tga_data = stbi__convert_format(
            tga_data,
            tga_comp,
            req_comp,
            tga_width as libc::c_uint,
            tga_height as libc::c_uint,
        );
    }
    tga_y_origin = 0 as libc::c_int;
    tga_x_origin = tga_y_origin;
    tga_palette_bits = tga_x_origin;
    tga_palette_len = tga_palette_bits;
    tga_palette_start = tga_palette_len;
    return tga_data as *mut libc::c_void;
}
unsafe extern "C" fn stbi__err(mut str: *const libc::c_char) -> libc::c_int {
    stbi__g_failure_reason = str;
    return 0 as libc::c_int;
}
unsafe extern "C" fn stbi__load_main(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
    mut ri: *mut stbi__result_info,
    mut bpc: libc::c_int,
) -> *mut libc::c_void {
    memset(
        ri as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<stbi__result_info>() as libc::c_ulong,
    );
    (*ri).bits_per_channel = 8 as libc::c_int;
    (*ri).channel_order = STBI_ORDER_RGB as libc::c_int;
    (*ri).num_channels = 0 as libc::c_int;
    if stbi__png_test(s) != 0 {
        return stbi__png_load(s, x, y, comp, req_comp, ri);
    }
    if stbi__bmp_test(s) != 0 {
        return stbi__bmp_load(s, x, y, comp, req_comp, ri);
    }
    if stbi__psd_test(s) != 0 {
        return stbi__psd_load(s, x, y, comp, req_comp, ri, bpc);
    }
    if stbi__pic_test(s) != 0 {
        return stbi__pic_load(s, x, y, comp, req_comp, ri);
    }
    if stbi__jpeg_test(s) != 0 {
        return stbi__jpeg_load(s, x, y, comp, req_comp, ri);
    }
    if stbi__hdr_test(s) != 0 {
        let mut hdr: *mut libc::c_float = stbi__hdr_load(s, x, y, comp, req_comp, ri);
        return stbi__hdr_to_ldr(
            hdr,
            *x,
            *y,
            if req_comp != 0 { req_comp } else { *comp },
        ) as *mut libc::c_void;
    }
    if stbi__tga_test(s) != 0 {
        return stbi__tga_load(s, x, y, comp, req_comp, ri);
    }
    return (if stbi__err(
        b"Image not of any known type, or corrupt\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        0 as *mut libc::c_void
    } else {
        0 as *mut libc::c_void
    }) as size_t as *mut libc::c_uchar as *mut libc::c_void;
}
unsafe extern "C" fn stbi__load_and_postprocess_8bit(
    mut s: *mut stbi__context,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
    mut comp: *mut libc::c_int,
    mut req_comp: libc::c_int,
) -> *mut libc::c_uchar {
    let mut ri: stbi__result_info = stbi__result_info {
        bits_per_channel: 0,
        num_channels: 0,
        channel_order: 0,
    };
    let mut result: *mut libc::c_void = stbi__load_main(
        s,
        x,
        y,
        comp,
        req_comp,
        &mut ri,
        8 as libc::c_int,
    );
    if result.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    if ri.bits_per_channel == 8 as libc::c_int
        || ri.bits_per_channel == 16 as libc::c_int
    {} else {
        __assert_fail(
            b"ri.bits_per_channel == 8 || ri.bits_per_channel == 16\0" as *const u8
                as *const libc::c_char,
            b"../src/stb_image.h\0" as *const u8 as *const libc::c_char,
            1249 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"unsigned char *stbi__load_and_postprocess_8bit(stbi__context *, int *, int *, int *, int)\0",
            ))
                .as_ptr(),
        );
    };
    if ri.bits_per_channel != 8 as libc::c_int {
        result = stbi__convert_16_to_8(
            result as *mut stbi__uint16,
            *x,
            *y,
            if req_comp == 0 as libc::c_int { *comp } else { req_comp },
        ) as *mut libc::c_void;
        ri.bits_per_channel = 8 as libc::c_int;
    }
    if if stbi__vertically_flip_on_load_set != 0 {
        stbi__vertically_flip_on_load_local
    } else {
        stbi__vertically_flip_on_load_global
    } != 0
    {
        let mut channels: libc::c_int = if req_comp != 0 { req_comp } else { *comp };
        stbi__vertical_flip(
            result,
            *x,
            *y,
            (channels as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<stbi_uc>() as libc::c_ulong)
                as libc::c_int,
        );
    }
    return result as *mut libc::c_uchar;
}
#[thread_local]
static mut stbi__g_failure_reason: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn stbi_failure_reason() -> *const libc::c_char {
    return stbi__g_failure_reason;
}
#[no_mangle]
pub unsafe extern "C" fn stbi_convert_iphone_png_to_rgb(
    mut flag_true_if_should_convert: libc::c_int,
) {
    stbi__de_iphone_flag_global = flag_true_if_should_convert;
}
#[no_mangle]
pub unsafe extern "C" fn stbi__unpremultiply_on_load_thread(
    mut flag_true_if_should_unpremultiply: libc::c_int,
) {
    stbi__unpremultiply_on_load_local = flag_true_if_should_unpremultiply;
    stbi__unpremultiply_on_load_set = 1 as libc::c_int;
}
unsafe extern "C" fn load_jpeg(
    mut result: *mut *mut libc::c_uchar,
    mut data: *mut libc::c_uchar,
    mut datasize: size_t,
    mut pwidth: *mut libc::c_int,
    mut pheight: *mut libc::c_int,
    mut ppixelformat: *mut libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int | 0x600 as libc::c_int;
    let mut row_stride: JDIMENSION = 0;
    let mut size: size_t = 0;
    let mut buffer: JSAMPARRAY = 0 as *mut JSAMPROW;
    let mut cinfo: jpeg_decompress_struct = jpeg_decompress_struct {
        err: 0 as *mut jpeg_error_mgr,
        mem: 0 as *mut jpeg_memory_mgr,
        progress: 0 as *mut jpeg_progress_mgr,
        client_data: 0 as *mut libc::c_void,
        is_decompressor: 0,
        global_state: 0,
        src: 0 as *mut jpeg_source_mgr,
        image_width: 0,
        image_height: 0,
        num_components: 0,
        jpeg_color_space: JCS_UNKNOWN,
        out_color_space: JCS_UNKNOWN,
        scale_num: 0,
        scale_denom: 0,
        output_gamma: 0.,
        buffered_image: 0,
        raw_data_out: 0,
        dct_method: JDCT_ISLOW,
        do_fancy_upsampling: 0,
        do_block_smoothing: 0,
        quantize_colors: 0,
        dither_mode: JDITHER_NONE,
        two_pass_quantize: 0,
        desired_number_of_colors: 0,
        enable_1pass_quant: 0,
        enable_external_quant: 0,
        enable_2pass_quant: 0,
        output_width: 0,
        output_height: 0,
        out_color_components: 0,
        output_components: 0,
        rec_outbuf_height: 0,
        actual_number_of_colors: 0,
        colormap: 0 as *mut JSAMPROW,
        output_scanline: 0,
        input_scan_number: 0,
        input_iMCU_row: 0,
        output_scan_number: 0,
        output_iMCU_row: 0,
        coef_bits: 0 as *mut [libc::c_int; 64],
        quant_tbl_ptrs: [0 as *mut JQUANT_TBL; 4],
        dc_huff_tbl_ptrs: [0 as *mut JHUFF_TBL; 4],
        ac_huff_tbl_ptrs: [0 as *mut JHUFF_TBL; 4],
        data_precision: 0,
        comp_info: 0 as *mut jpeg_component_info,
        is_baseline: 0,
        progressive_mode: 0,
        arith_code: 0,
        arith_dc_L: [0; 16],
        arith_dc_U: [0; 16],
        arith_ac_K: [0; 16],
        restart_interval: 0,
        saw_JFIF_marker: 0,
        JFIF_major_version: 0,
        JFIF_minor_version: 0,
        density_unit: 0,
        X_density: 0,
        Y_density: 0,
        saw_Adobe_marker: 0,
        Adobe_transform: 0,
        CCIR601_sampling: 0,
        marker_list: 0 as *mut jpeg_marker_struct,
        max_h_samp_factor: 0,
        max_v_samp_factor: 0,
        min_DCT_h_scaled_size: 0,
        min_DCT_v_scaled_size: 0,
        total_iMCU_rows: 0,
        sample_range_limit: 0 as *mut JSAMPLE,
        comps_in_scan: 0,
        cur_comp_info: [0 as *mut jpeg_component_info; 4],
        MCUs_per_row: 0,
        MCU_rows_in_scan: 0,
        blocks_in_MCU: 0,
        MCU_membership: [0; 10],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
        block_size: 0,
        natural_order: 0 as *const libc::c_int,
        lim_Se: 0,
        unread_marker: 0,
        master: 0 as *mut jpeg_decomp_master,
        main: 0 as *mut jpeg_d_main_controller,
        coef: 0 as *mut jpeg_d_coef_controller,
        post: 0 as *mut jpeg_d_post_controller,
        inputctl: 0 as *mut jpeg_input_controller,
        marker: 0 as *mut jpeg_marker_reader,
        entropy: 0 as *mut jpeg_entropy_decoder,
        idct: 0 as *mut jpeg_inverse_dct,
        upsample: 0 as *mut jpeg_upsampler,
        cconvert: 0 as *mut jpeg_color_deconverter,
        cquantize: 0 as *mut jpeg_color_quantizer,
    };
    let mut pub_0: jpeg_error_mgr = jpeg_error_mgr {
        error_exit: None,
        emit_message: None,
        output_message: None,
        format_message: None,
        reset_error_mgr: None,
        msg_code: 0,
        msg_parm: C2RustUnnamed { i: [0; 8] },
        trace_level: 0,
        num_warnings: 0,
        jpeg_message_table: 0 as *const *const libc::c_char,
        last_jpeg_message: 0,
        addon_message_table: 0 as *const *const libc::c_char,
        first_addon_message: 0,
        last_addon_message: 0,
    };
    cinfo.err = jpeg_std_error(&mut pub_0);
    jpeg_CreateDecompress(
        &mut cinfo,
        80 as libc::c_int,
        ::core::mem::size_of::<jpeg_decompress_struct>() as libc::c_ulong,
    );
    jpeg_mem_src(&mut cinfo, data, datasize);
    jpeg_read_header(&mut cinfo, 1 as libc::c_int);
    cinfo.quantize_colors = 0 as libc::c_int;
    cinfo.out_color_space = JCS_RGB;
    jpeg_start_decompress(&mut cinfo);
    if cinfo.output_components != 3 as libc::c_int {
        sixel_helper_set_additional_message(
            b"load_jpeg: unknown pixel format.\0" as *const u8 as *const libc::c_char,
        );
        status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x3 as libc::c_int;
    } else {
        *ppixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
        *pwidth = cinfo.output_width as libc::c_int;
        *pheight = cinfo.output_height as libc::c_int;
        size = (*pwidth * *pheight * cinfo.output_components) as size_t;
        *result = sixel_allocator_malloc(allocator, size) as *mut libc::c_uchar;
        if (*result).is_null() {
            sixel_helper_set_additional_message(
                b"load_jpeg: sixel_allocator_malloc() failed.\0" as *const u8
                    as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x100 as libc::c_int | 0x1 as libc::c_int;
        } else {
            row_stride = (cinfo.output_width)
                .wrapping_mul(cinfo.output_components as libc::c_uint);
            buffer = (Some(
                ((*cinfo.mem).alloc_sarray).expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                &mut cinfo as *mut jpeg_decompress_struct as j_common_ptr,
                1 as libc::c_int,
                row_stride,
                1 as libc::c_int as JDIMENSION,
            );
            loop {
                if !(cinfo.output_scanline < cinfo.output_height) {
                    current_block = 5601891728916014340;
                    break;
                }
                jpeg_read_scanlines(&mut cinfo, buffer, 1 as libc::c_int as JDIMENSION);
                if (*cinfo.err).num_warnings > 0 as libc::c_int as libc::c_long {
                    sixel_helper_set_additional_message(
                        b"jpeg_read_scanlines: error/warining occuered.\0" as *const u8
                            as *const libc::c_char,
                    );
                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                        | 0x3 as libc::c_int;
                    current_block = 12241330675383869954;
                    break;
                } else {
                    memcpy(
                        (*result)
                            .offset(
                                (cinfo.output_scanline)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    .wrapping_mul(row_stride) as isize,
                            ) as *mut libc::c_void,
                        *buffer.offset(0 as libc::c_int as isize) as *const libc::c_void,
                        row_stride as libc::c_ulong,
                    );
                }
            }
            match current_block {
                12241330675383869954 => {}
                _ => {
                    status = 0 as libc::c_int;
                }
            }
        }
    }
    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);
    return status;
}
unsafe extern "C" fn read_png(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: png_size_t,
) {
    let mut pchunk: *mut sixel_chunk_t = png_get_io_ptr(png_ptr as *const png_struct)
        as *mut sixel_chunk_t;
    if length > (*pchunk).size {
        length = (*pchunk).size;
    }
    if length > 0 as libc::c_int as libc::c_ulong {
        memcpy(
            data as *mut libc::c_void,
            (*pchunk).buffer as *const libc::c_void,
            length,
        );
        (*pchunk).buffer = ((*pchunk).buffer).offset(length as isize);
        (*pchunk)
            .size = ((*pchunk).size as libc::c_ulong).wrapping_sub(length) as size_t
            as size_t;
    }
}
unsafe extern "C" fn read_palette(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut palette: *mut libc::c_uchar,
    mut ncolors: libc::c_int,
    mut png_palette: *mut png_color,
    mut pbackground: *mut png_color_16,
    mut transparent: *mut libc::c_int,
) {
    let mut trans: png_bytep = 0 as png_bytep;
    let mut num_trans: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if png_get_valid(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        0x10 as libc::c_uint,
    ) != 0
    {
        png_get_tRNS(
            png_ptr as *const png_struct,
            info_ptr,
            &mut trans,
            &mut num_trans,
            0 as *mut png_color_16p,
        );
    }
    if num_trans > 0 as libc::c_int {
        *transparent = *trans.offset(0 as libc::c_int as isize) as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < ncolors {
        if !pbackground.is_null() && i < num_trans {
            *palette
                .offset(
                    (i * 3 as libc::c_int + 0 as libc::c_int) as isize,
                ) = ((0xff as libc::c_int - *trans.offset(i as isize) as libc::c_int)
                * (*pbackground).red as libc::c_int
                + *trans.offset(i as isize) as libc::c_int
                    * (*png_palette.offset(i as isize)).red as libc::c_int
                >> 8 as libc::c_int) as libc::c_uchar;
            *palette
                .offset(
                    (i * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = ((0xff as libc::c_int - *trans.offset(i as isize) as libc::c_int)
                * (*pbackground).green as libc::c_int
                + *trans.offset(i as isize) as libc::c_int
                    * (*png_palette.offset(i as isize)).green as libc::c_int
                >> 8 as libc::c_int) as libc::c_uchar;
            *palette
                .offset(
                    (i * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = ((0xff as libc::c_int - *trans.offset(i as isize) as libc::c_int)
                * (*pbackground).blue as libc::c_int
                + *trans.offset(i as isize) as libc::c_int
                    * (*png_palette.offset(i as isize)).blue as libc::c_int
                >> 8 as libc::c_int) as libc::c_uchar;
        } else {
            *palette
                .offset(
                    (i * 3 as libc::c_int + 0 as libc::c_int) as isize,
                ) = (*png_palette.offset(i as isize)).red;
            *palette
                .offset(
                    (i * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (*png_palette.offset(i as isize)).green;
            *palette
                .offset(
                    (i * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = (*png_palette.offset(i as isize)).blue;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut jmpbuf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn png_error_callback(
    mut png_ptr: png_structp,
    mut error_message: png_const_charp,
) {
    sixel_helper_set_additional_message(error_message);
    longjmp(jmpbuf.as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn load_png(
    mut result: *mut *mut libc::c_uchar,
    mut buffer: *mut libc::c_uchar,
    mut size: size_t,
    mut psx: *mut libc::c_int,
    mut psy: *mut libc::c_int,
    mut ppalette: *mut *mut libc::c_uchar,
    mut pncolors: *mut libc::c_int,
    mut reqcolors: libc::c_int,
    mut pixelformat: *mut libc::c_int,
    mut bgcolor: *mut libc::c_uchar,
    mut transparent: *mut libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0;
    let mut read_chunk: sixel_chunk_t = sixel_chunk_t {
        buffer: 0 as *mut libc::c_uchar,
        size: 0,
        max_size: 0,
        allocator: 0 as *mut sixel_allocator_t,
    };
    let mut bitdepth: png_uint_32 = 0;
    let mut png_status: png_uint_32 = 0;
    let mut png_ptr: png_structp = 0 as *mut png_struct;
    let mut info_ptr: png_infop = 0 as *mut png_info;
    let mut rows: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    let mut png_palette: *mut png_color = 0 as *mut png_color;
    let mut background: png_color_16 = png_color_16 {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
    };
    let mut default_background: png_color_16p = 0 as *mut png_color_16;
    let mut i: libc::c_int = 0;
    let mut depth: libc::c_int = 0;
    if _setjmp(jmpbuf.as_mut_ptr()) != 0 as libc::c_int {
        sixel_allocator_free(allocator, *result as *mut libc::c_void);
        *result = 0 as *mut libc::c_uchar;
        status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
    } else {
        status = 0x1000 as libc::c_int;
        *result = 0 as *mut libc::c_uchar;
        png_ptr = png_create_read_struct(
            b"1.6.40\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
            Some(
                png_error_callback
                    as unsafe extern "C" fn(png_structp, png_const_charp) -> (),
            ),
            None,
        );
        if png_ptr.is_null() {
            sixel_helper_set_additional_message(
                b"png_create_read_struct() failed.\0" as *const u8 as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
        } else if size < 67 as libc::c_int as libc::c_ulong {
            sixel_helper_set_additional_message(
                b"PNG data too small to be valid!\0" as *const u8 as *const libc::c_char,
            );
            status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
        } else if _setjmp(
            (*png_set_longjmp_fn(
                png_ptr,
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !>,
                    png_longjmp_ptr,
                >(
                    Some(
                        longjmp
                            as unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                    ),
                ),
                ::core::mem::size_of::<jmp_buf>() as libc::c_ulong,
            ))
                .as_mut_ptr(),
        ) != 0 as libc::c_int
        {
            sixel_allocator_free(allocator, *result as *mut libc::c_void);
            *result = 0 as *mut libc::c_uchar;
            status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
        } else {
            info_ptr = png_create_info_struct(png_ptr as *const png_struct);
            if info_ptr.is_null() {
                sixel_helper_set_additional_message(
                    b"png_create_info_struct() failed.\0" as *const u8
                        as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
                png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
            } else {
                read_chunk.buffer = buffer;
                read_chunk.size = size;
                png_set_read_fn(
                    png_ptr,
                    &mut read_chunk as *mut sixel_chunk_t as png_voidp,
                    Some(
                        read_png
                            as unsafe extern "C" fn(
                                png_structp,
                                png_bytep,
                                png_size_t,
                            ) -> (),
                    ),
                );
                png_read_info(png_ptr, info_ptr);
                *psx = png_get_image_width(
                    png_ptr as *const png_struct,
                    info_ptr as *const png_info,
                ) as libc::c_int;
                *psy = png_get_image_height(
                    png_ptr as *const png_struct,
                    info_ptr as *const png_info,
                ) as libc::c_int;
                bitdepth = png_get_bit_depth(
                    png_ptr as *const png_struct,
                    info_ptr as *const png_info,
                ) as png_uint_32;
                if bitdepth == 16 as libc::c_int as libc::c_uint {
                    png_set_strip_16(png_ptr);
                    bitdepth = 8 as libc::c_int as png_uint_32;
                }
                if !bgcolor.is_null() {
                    background
                        .red = *bgcolor.offset(0 as libc::c_int as isize) as png_uint_16;
                    background
                        .green = *bgcolor.offset(1 as libc::c_int as isize)
                        as png_uint_16;
                    background
                        .blue = *bgcolor.offset(2 as libc::c_int as isize)
                        as png_uint_16;
                    background
                        .gray = ((*bgcolor.offset(0 as libc::c_int as isize)
                        as libc::c_int
                        + *bgcolor.offset(1 as libc::c_int as isize) as libc::c_int
                        + *bgcolor.offset(2 as libc::c_int as isize) as libc::c_int)
                        / 3 as libc::c_int) as png_uint_16;
                } else if png_get_bKGD(
                    png_ptr as *const png_struct,
                    info_ptr,
                    &mut default_background,
                ) == 0x20 as libc::c_uint
                {
                    memcpy(
                        &mut background as *mut png_color_16 as *mut libc::c_void,
                        default_background as *const libc::c_void,
                        ::core::mem::size_of::<png_color_16>() as libc::c_ulong,
                    );
                } else {
                    background.red = 0 as libc::c_int as png_uint_16;
                    background.green = 0 as libc::c_int as png_uint_16;
                    background.blue = 0 as libc::c_int as png_uint_16;
                    background.gray = 0 as libc::c_int as png_uint_16;
                }
                match png_get_color_type(
                    png_ptr as *const png_struct,
                    info_ptr as *const png_info,
                ) as libc::c_int
                {
                    3 => {
                        current_block = 1349680389701616210;
                        match current_block {
                            2339125390404385036 => {
                                if (1 as libc::c_int) << bitdepth > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_gray_to_rgb(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                } else {
                                    match bitdepth {
                                        1 | 2 | 4 => {
                                            if !ppalette.is_null() {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        8 => {
                                            if !ppalette.is_null() {
                                                *pixelformat = (1 as libc::c_int) << 6 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_gray_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                        }
                                    }
                                }
                                current_block = 17372050596571538954;
                            }
                            4166128649646877775 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                png_set_gray_to_rgb(png_ptr);
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14433601038658658338 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14194937224381651786 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            _ => {
                                png_status = png_get_PLTE(
                                    png_ptr as *const png_struct,
                                    info_ptr,
                                    &mut png_palette,
                                    pncolors,
                                );
                                if png_status != 0x8 as libc::c_uint
                                    || png_palette.is_null()
                                {
                                    sixel_helper_set_additional_message(
                                        b"PLTE chunk not found\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
                                    current_block = 15439961272960371067;
                                } else if ppalette.is_null() || *pncolors > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_palette_to_rgb(png_ptr);
                                    png_set_strip_alpha(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                    current_block = 17372050596571538954;
                                } else {
                                    match bitdepth {
                                        1 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        2 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        4 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x2 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        8 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_palette_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            current_block = 17372050596571538954;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            15439961272960371067 => {}
                            _ => {
                                depth = sixel_helper_compute_depth(*pixelformat);
                                *result = sixel_allocator_malloc(
                                    allocator,
                                    (*psx * *psy * depth) as size_t,
                                ) as *mut libc::c_uchar;
                                if (*result).is_null() {
                                    sixel_helper_set_additional_message(
                                        b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                        | 0x1 as libc::c_int;
                                } else {
                                    rows = sixel_allocator_malloc(
                                        allocator,
                                        (*psy as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<*mut libc::c_uchar>()
                                                    as libc::c_ulong,
                                            ),
                                    ) as *mut *mut libc::c_uchar;
                                    if rows.is_null() {
                                        sixel_helper_set_additional_message(
                                            b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                            | 0x1 as libc::c_int;
                                    } else {
                                        match *pixelformat {
                                            128 | 129 | 130 => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh101 = *rows.offset(i as isize);
                                                    *fresh101 = (*result)
                                                        .offset(
                                                            ((depth * *psx * bitdepth as libc::c_int + 7 as libc::c_int)
                                                                / 8 as libc::c_int * i) as isize,
                                                        );
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                            _ => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh102 = *rows.offset(i as isize);
                                                    *fresh102 = (*result).offset((depth * *psx * i) as isize);
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                        }
                                        png_read_image(png_ptr, rows);
                                        status = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                    0 => {
                        current_block = 2339125390404385036;
                        match current_block {
                            2339125390404385036 => {
                                if (1 as libc::c_int) << bitdepth > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_gray_to_rgb(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                } else {
                                    match bitdepth {
                                        1 | 2 | 4 => {
                                            if !ppalette.is_null() {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        8 => {
                                            if !ppalette.is_null() {
                                                *pixelformat = (1 as libc::c_int) << 6 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_gray_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                        }
                                    }
                                }
                                current_block = 17372050596571538954;
                            }
                            4166128649646877775 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                png_set_gray_to_rgb(png_ptr);
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14433601038658658338 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14194937224381651786 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            _ => {
                                png_status = png_get_PLTE(
                                    png_ptr as *const png_struct,
                                    info_ptr,
                                    &mut png_palette,
                                    pncolors,
                                );
                                if png_status != 0x8 as libc::c_uint
                                    || png_palette.is_null()
                                {
                                    sixel_helper_set_additional_message(
                                        b"PLTE chunk not found\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
                                    current_block = 15439961272960371067;
                                } else if ppalette.is_null() || *pncolors > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_palette_to_rgb(png_ptr);
                                    png_set_strip_alpha(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                    current_block = 17372050596571538954;
                                } else {
                                    match bitdepth {
                                        1 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        2 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        4 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x2 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        8 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_palette_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            current_block = 17372050596571538954;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            15439961272960371067 => {}
                            _ => {
                                depth = sixel_helper_compute_depth(*pixelformat);
                                *result = sixel_allocator_malloc(
                                    allocator,
                                    (*psx * *psy * depth) as size_t,
                                ) as *mut libc::c_uchar;
                                if (*result).is_null() {
                                    sixel_helper_set_additional_message(
                                        b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                        | 0x1 as libc::c_int;
                                } else {
                                    rows = sixel_allocator_malloc(
                                        allocator,
                                        (*psy as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<*mut libc::c_uchar>()
                                                    as libc::c_ulong,
                                            ),
                                    ) as *mut *mut libc::c_uchar;
                                    if rows.is_null() {
                                        sixel_helper_set_additional_message(
                                            b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                            | 0x1 as libc::c_int;
                                    } else {
                                        match *pixelformat {
                                            128 | 129 | 130 => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh101 = *rows.offset(i as isize);
                                                    *fresh101 = (*result)
                                                        .offset(
                                                            ((depth * *psx * bitdepth as libc::c_int + 7 as libc::c_int)
                                                                / 8 as libc::c_int * i) as isize,
                                                        );
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                            _ => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh102 = *rows.offset(i as isize);
                                                    *fresh102 = (*result).offset((depth * *psx * i) as isize);
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                        }
                                        png_read_image(png_ptr, rows);
                                        status = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                    4 => {
                        current_block = 4166128649646877775;
                        match current_block {
                            2339125390404385036 => {
                                if (1 as libc::c_int) << bitdepth > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_gray_to_rgb(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                } else {
                                    match bitdepth {
                                        1 | 2 | 4 => {
                                            if !ppalette.is_null() {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        8 => {
                                            if !ppalette.is_null() {
                                                *pixelformat = (1 as libc::c_int) << 6 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_gray_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                        }
                                    }
                                }
                                current_block = 17372050596571538954;
                            }
                            4166128649646877775 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                png_set_gray_to_rgb(png_ptr);
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14433601038658658338 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14194937224381651786 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            _ => {
                                png_status = png_get_PLTE(
                                    png_ptr as *const png_struct,
                                    info_ptr,
                                    &mut png_palette,
                                    pncolors,
                                );
                                if png_status != 0x8 as libc::c_uint
                                    || png_palette.is_null()
                                {
                                    sixel_helper_set_additional_message(
                                        b"PLTE chunk not found\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
                                    current_block = 15439961272960371067;
                                } else if ppalette.is_null() || *pncolors > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_palette_to_rgb(png_ptr);
                                    png_set_strip_alpha(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                    current_block = 17372050596571538954;
                                } else {
                                    match bitdepth {
                                        1 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        2 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        4 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x2 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        8 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_palette_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            current_block = 17372050596571538954;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            15439961272960371067 => {}
                            _ => {
                                depth = sixel_helper_compute_depth(*pixelformat);
                                *result = sixel_allocator_malloc(
                                    allocator,
                                    (*psx * *psy * depth) as size_t,
                                ) as *mut libc::c_uchar;
                                if (*result).is_null() {
                                    sixel_helper_set_additional_message(
                                        b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                        | 0x1 as libc::c_int;
                                } else {
                                    rows = sixel_allocator_malloc(
                                        allocator,
                                        (*psy as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<*mut libc::c_uchar>()
                                                    as libc::c_ulong,
                                            ),
                                    ) as *mut *mut libc::c_uchar;
                                    if rows.is_null() {
                                        sixel_helper_set_additional_message(
                                            b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                            | 0x1 as libc::c_int;
                                    } else {
                                        match *pixelformat {
                                            128 | 129 | 130 => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh101 = *rows.offset(i as isize);
                                                    *fresh101 = (*result)
                                                        .offset(
                                                            ((depth * *psx * bitdepth as libc::c_int + 7 as libc::c_int)
                                                                / 8 as libc::c_int * i) as isize,
                                                        );
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                            _ => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh102 = *rows.offset(i as isize);
                                                    *fresh102 = (*result).offset((depth * *psx * i) as isize);
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                        }
                                        png_read_image(png_ptr, rows);
                                        status = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                    6 => {
                        current_block = 14433601038658658338;
                        match current_block {
                            2339125390404385036 => {
                                if (1 as libc::c_int) << bitdepth > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_gray_to_rgb(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                } else {
                                    match bitdepth {
                                        1 | 2 | 4 => {
                                            if !ppalette.is_null() {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        8 => {
                                            if !ppalette.is_null() {
                                                *pixelformat = (1 as libc::c_int) << 6 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_gray_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                        }
                                    }
                                }
                                current_block = 17372050596571538954;
                            }
                            4166128649646877775 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                png_set_gray_to_rgb(png_ptr);
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14433601038658658338 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14194937224381651786 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            _ => {
                                png_status = png_get_PLTE(
                                    png_ptr as *const png_struct,
                                    info_ptr,
                                    &mut png_palette,
                                    pncolors,
                                );
                                if png_status != 0x8 as libc::c_uint
                                    || png_palette.is_null()
                                {
                                    sixel_helper_set_additional_message(
                                        b"PLTE chunk not found\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
                                    current_block = 15439961272960371067;
                                } else if ppalette.is_null() || *pncolors > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_palette_to_rgb(png_ptr);
                                    png_set_strip_alpha(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                    current_block = 17372050596571538954;
                                } else {
                                    match bitdepth {
                                        1 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        2 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        4 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x2 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        8 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_palette_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            current_block = 17372050596571538954;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            15439961272960371067 => {}
                            _ => {
                                depth = sixel_helper_compute_depth(*pixelformat);
                                *result = sixel_allocator_malloc(
                                    allocator,
                                    (*psx * *psy * depth) as size_t,
                                ) as *mut libc::c_uchar;
                                if (*result).is_null() {
                                    sixel_helper_set_additional_message(
                                        b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                        | 0x1 as libc::c_int;
                                } else {
                                    rows = sixel_allocator_malloc(
                                        allocator,
                                        (*psy as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<*mut libc::c_uchar>()
                                                    as libc::c_ulong,
                                            ),
                                    ) as *mut *mut libc::c_uchar;
                                    if rows.is_null() {
                                        sixel_helper_set_additional_message(
                                            b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                            | 0x1 as libc::c_int;
                                    } else {
                                        match *pixelformat {
                                            128 | 129 | 130 => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh101 = *rows.offset(i as isize);
                                                    *fresh101 = (*result)
                                                        .offset(
                                                            ((depth * *psx * bitdepth as libc::c_int + 7 as libc::c_int)
                                                                / 8 as libc::c_int * i) as isize,
                                                        );
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                            _ => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh102 = *rows.offset(i as isize);
                                                    *fresh102 = (*result).offset((depth * *psx * i) as isize);
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                        }
                                        png_read_image(png_ptr, rows);
                                        status = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 14194937224381651786;
                        match current_block {
                            2339125390404385036 => {
                                if (1 as libc::c_int) << bitdepth > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_gray_to_rgb(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                } else {
                                    match bitdepth {
                                        1 | 2 | 4 => {
                                            if !ppalette.is_null() {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        8 => {
                                            if !ppalette.is_null() {
                                                *pixelformat = (1 as libc::c_int) << 6 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                            } else {
                                                png_set_background(
                                                    png_ptr,
                                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                                    1 as libc::c_int,
                                                    0 as libc::c_int,
                                                    1.0f64,
                                                );
                                                png_set_gray_to_rgb(png_ptr);
                                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_gray_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                        }
                                    }
                                }
                                current_block = 17372050596571538954;
                            }
                            4166128649646877775 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                png_set_gray_to_rgb(png_ptr);
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14433601038658658338 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            14194937224381651786 => {
                                png_set_background(
                                    png_ptr,
                                    &mut background as *mut png_color_16 as png_const_color_16p,
                                    1 as libc::c_int,
                                    0 as libc::c_int,
                                    1.0f64,
                                );
                                *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                current_block = 17372050596571538954;
                            }
                            _ => {
                                png_status = png_get_PLTE(
                                    png_ptr as *const png_struct,
                                    info_ptr,
                                    &mut png_palette,
                                    pncolors,
                                );
                                if png_status != 0x8 as libc::c_uint
                                    || png_palette.is_null()
                                {
                                    sixel_helper_set_additional_message(
                                        b"PLTE chunk not found\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x700 as libc::c_int;
                                    current_block = 15439961272960371067;
                                } else if ppalette.is_null() || *pncolors > reqcolors {
                                    png_set_background(
                                        png_ptr,
                                        &mut background as *mut png_color_16 as png_const_color_16p,
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        1.0f64,
                                    );
                                    png_set_palette_to_rgb(png_ptr);
                                    png_set_strip_alpha(png_ptr);
                                    *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                    current_block = 17372050596571538954;
                                } else {
                                    match bitdepth {
                                        1 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        2 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        4 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x2 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        8 => {
                                            *ppalette = sixel_allocator_malloc(
                                                allocator,
                                                (*pncolors as size_t)
                                                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
                                            ) as *mut libc::c_uchar;
                                            if (*ppalette).is_null() {
                                                sixel_helper_set_additional_message(
                                                    b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                                    | 0x1 as libc::c_int;
                                                current_block = 15439961272960371067;
                                            } else {
                                                read_palette(
                                                    png_ptr,
                                                    info_ptr,
                                                    *ppalette,
                                                    *pncolors,
                                                    png_palette,
                                                    &mut background,
                                                    transparent,
                                                );
                                                *pixelformat = (1 as libc::c_int) << 7 as libc::c_int
                                                    | 0x3 as libc::c_int;
                                                current_block = 17372050596571538954;
                                            }
                                        }
                                        _ => {
                                            png_set_background(
                                                png_ptr,
                                                &mut background as *mut png_color_16 as png_const_color_16p,
                                                1 as libc::c_int,
                                                0 as libc::c_int,
                                                1.0f64,
                                            );
                                            png_set_palette_to_rgb(png_ptr);
                                            *pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                                            current_block = 17372050596571538954;
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            15439961272960371067 => {}
                            _ => {
                                depth = sixel_helper_compute_depth(*pixelformat);
                                *result = sixel_allocator_malloc(
                                    allocator,
                                    (*psx * *psy * depth) as size_t,
                                ) as *mut libc::c_uchar;
                                if (*result).is_null() {
                                    sixel_helper_set_additional_message(
                                        b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                        | 0x1 as libc::c_int;
                                } else {
                                    rows = sixel_allocator_malloc(
                                        allocator,
                                        (*psy as size_t)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<*mut libc::c_uchar>()
                                                    as libc::c_ulong,
                                            ),
                                    ) as *mut *mut libc::c_uchar;
                                    if rows.is_null() {
                                        sixel_helper_set_additional_message(
                                            b"load_png: sixel_allocator_malloc() failed.\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                                            | 0x1 as libc::c_int;
                                    } else {
                                        match *pixelformat {
                                            128 | 129 | 130 => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh101 = *rows.offset(i as isize);
                                                    *fresh101 = (*result)
                                                        .offset(
                                                            ((depth * *psx * bitdepth as libc::c_int + 7 as libc::c_int)
                                                                / 8 as libc::c_int * i) as isize,
                                                        );
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                            _ => {
                                                i = 0 as libc::c_int;
                                                while i < *psy {
                                                    let ref mut fresh102 = *rows.offset(i as isize);
                                                    *fresh102 = (*result).offset((depth * *psx * i) as isize);
                                                    i += 1;
                                                    i;
                                                }
                                            }
                                        }
                                        png_read_image(png_ptr, rows);
                                        status = 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    png_destroy_read_struct(&mut png_ptr, &mut info_ptr, 0 as png_infopp);
    if !rows.is_null() {
        sixel_allocator_free(allocator, rows as *mut libc::c_void);
    }
    return status;
}
unsafe extern "C" fn load_sixel(
    mut result: *mut *mut libc::c_uchar,
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
    mut psx: *mut libc::c_int,
    mut psy: *mut libc::c_int,
    mut ppalette: *mut *mut libc::c_uchar,
    mut pncolors: *mut libc::c_int,
    mut reqcolors: libc::c_int,
    mut ppixelformat: *mut libc::c_int,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut palette: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut colors: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    status = sixel_decode_raw(
        buffer,
        size,
        &mut p,
        psx,
        psy,
        &mut palette,
        &mut colors,
        allocator,
    );
    if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
        if ppalette.is_null() || colors > reqcolors {
            *ppixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
            *result = sixel_allocator_malloc(
                allocator,
                (*psx * *psy * 3 as libc::c_int) as size_t,
            ) as *mut libc::c_uchar;
            if (*result).is_null() {
                sixel_helper_set_additional_message(
                    b"load_sixel: sixel_allocator_malloc() failed.\0" as *const u8
                        as *const libc::c_char,
                );
                status = 0x1000 as libc::c_int | 0x100 as libc::c_int
                    | 0x1 as libc::c_int;
            } else {
                i = 0 as libc::c_int;
                while i < *psx * *psy {
                    *(*result)
                        .offset(
                            (i * 3 as libc::c_int + 0 as libc::c_int) as isize,
                        ) = *palette
                        .offset(
                            (*p.offset(i as isize) as libc::c_int * 3 as libc::c_int
                                + 0 as libc::c_int) as isize,
                        );
                    *(*result)
                        .offset(
                            (i * 3 as libc::c_int + 1 as libc::c_int) as isize,
                        ) = *palette
                        .offset(
                            (*p.offset(i as isize) as libc::c_int * 3 as libc::c_int
                                + 1 as libc::c_int) as isize,
                        );
                    *(*result)
                        .offset(
                            (i * 3 as libc::c_int + 2 as libc::c_int) as isize,
                        ) = *palette
                        .offset(
                            (*p.offset(i as isize) as libc::c_int * 3 as libc::c_int
                                + 2 as libc::c_int) as isize,
                        );
                    i += 1;
                    i;
                }
            }
        } else {
            *ppixelformat = (1 as libc::c_int) << 7 as libc::c_int | 0x3 as libc::c_int;
            *result = p;
            *ppalette = palette;
            *pncolors = colors;
            p = 0 as *mut libc::c_uchar;
            palette = 0 as *mut libc::c_uchar;
        }
    }
    sixel_allocator_free(allocator, palette as *mut libc::c_void);
    sixel_allocator_free(allocator, p as *mut libc::c_void);
    return status;
}
unsafe extern "C" fn chunk_is_sixel(mut chunk: *const sixel_chunk_t) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = (*chunk).buffer;
    end = p.offset((*chunk).size as isize);
    if (*chunk).size < 3 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    p = p.offset(1);
    p;
    if p >= end {
        return 0 as libc::c_int;
    }
    if *p.offset(-(1 as libc::c_int as isize)) as libc::c_int == 0x90 as libc::c_int
        || *p.offset(-(1 as libc::c_int as isize)) as libc::c_int == 0x1b as libc::c_int
            && *p as libc::c_int == 0x50 as libc::c_int
    {
        loop {
            let fresh103 = p;
            p = p.offset(1);
            if !(fresh103 < end) {
                break;
            }
            if *p as libc::c_int == 0x71 as libc::c_int {
                return 1 as libc::c_int
            } else if *p as libc::c_int == 0x18 as libc::c_int
                || *p as libc::c_int == 0x1a as libc::c_int
            {
                return 0 as libc::c_int
            } else {
                if (*p as libc::c_int) < 0x20 as libc::c_int {
                    continue;
                }
                if (*p as libc::c_int) < 0x30 as libc::c_int {
                    return 0 as libc::c_int
                } else {
                    (*p as libc::c_int) < 0x40 as libc::c_int;
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn chunk_is_pnm(mut chunk: *const sixel_chunk_t) -> libc::c_int {
    if (*chunk).size < 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if *((*chunk).buffer).offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
        && *((*chunk).buffer).offset(1 as libc::c_int as isize) as libc::c_int
            >= '1' as i32
        && *((*chunk).buffer).offset(1 as libc::c_int as isize) as libc::c_int
            <= '6' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn chunk_is_png(mut chunk: *const sixel_chunk_t) -> libc::c_int {
    if (*chunk).size < 8 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if png_sig_cmp(
        (*chunk).buffer as png_const_bytep,
        0 as libc::c_int as size_t,
        8 as libc::c_int as size_t,
    ) == 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn chunk_is_gif(mut chunk: *const sixel_chunk_t) -> libc::c_int {
    if (*chunk).size < 6 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if *((*chunk).buffer).offset(0 as libc::c_int as isize) as libc::c_int == 'G' as i32
        && *((*chunk).buffer).offset(1 as libc::c_int as isize) as libc::c_int
            == 'I' as i32
        && *((*chunk).buffer).offset(2 as libc::c_int as isize) as libc::c_int
            == 'F' as i32
        && *((*chunk).buffer).offset(3 as libc::c_int as isize) as libc::c_int
            == '8' as i32
        && (*((*chunk).buffer).offset(4 as libc::c_int as isize) as libc::c_int
            == '7' as i32
            || *((*chunk).buffer).offset(4 as libc::c_int as isize) as libc::c_int
                == '9' as i32)
        && *((*chunk).buffer).offset(5 as libc::c_int as isize) as libc::c_int
            == 'a' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn chunk_is_jpeg(mut chunk: *const sixel_chunk_t) -> libc::c_int {
    if (*chunk).size < 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if memcmp(
        b"\xFF\xD8\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        (*chunk).buffer as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn load_with_builtin(
    mut pchunk: *const sixel_chunk_t,
    mut fstatic: libc::c_int,
    mut fuse_palette: libc::c_int,
    mut reqcolors: libc::c_int,
    mut bgcolor: *mut libc::c_uchar,
    mut loop_control: libc::c_int,
    mut fn_load: sixel_load_image_function,
    mut context: *mut libc::c_void,
) -> SIXELSTATUS {
    let mut current_block: u64;
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut frame: *mut sixel_frame_t = 0 as *mut sixel_frame_t;
    let mut message: [libc::c_char; 256] = [0; 256];
    let mut nwrite: libc::c_int = 0;
    let mut fnp: fn_pointer = _fn_pointer { fn_0: None };
    if chunk_is_sixel(pchunk) != 0 {
        status = sixel_frame_new(&mut frame, (*pchunk).allocator);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            current_block = 3544816217675459574;
        } else {
            status = load_sixel(
                &mut (*frame).pixels,
                (*pchunk).buffer,
                (*pchunk).size as libc::c_int,
                &mut (*frame).width,
                &mut (*frame).height,
                if fuse_palette != 0 {
                    &mut (*frame).palette
                } else {
                    0 as *mut *mut libc::c_uchar
                },
                &mut (*frame).ncolors,
                reqcolors,
                &mut (*frame).pixelformat,
                (*pchunk).allocator,
            );
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                current_block = 3544816217675459574;
            } else {
                current_block = 721385680381463314;
            }
        }
    } else if chunk_is_pnm(pchunk) != 0 {
        status = sixel_frame_new(&mut frame, (*pchunk).allocator);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            current_block = 3544816217675459574;
        } else {
            status = load_pnm(
                (*pchunk).buffer,
                (*pchunk).size as libc::c_int,
                (*frame).allocator,
                &mut (*frame).pixels,
                &mut (*frame).width,
                &mut (*frame).height,
                if fuse_palette != 0 {
                    &mut (*frame).palette
                } else {
                    0 as *mut *mut libc::c_uchar
                },
                &mut (*frame).ncolors,
                &mut (*frame).pixelformat,
            );
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                current_block = 3544816217675459574;
            } else {
                current_block = 721385680381463314;
            }
        }
    } else if chunk_is_jpeg(pchunk) != 0 {
        status = sixel_frame_new(&mut frame, (*pchunk).allocator);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            current_block = 3544816217675459574;
        } else {
            status = load_jpeg(
                &mut (*frame).pixels,
                (*pchunk).buffer,
                (*pchunk).size,
                &mut (*frame).width,
                &mut (*frame).height,
                &mut (*frame).pixelformat,
                (*pchunk).allocator,
            );
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                current_block = 3544816217675459574;
            } else {
                current_block = 721385680381463314;
            }
        }
    } else if chunk_is_png(pchunk) != 0 {
        status = sixel_frame_new(&mut frame, (*pchunk).allocator);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            current_block = 3544816217675459574;
        } else {
            status = load_png(
                &mut (*frame).pixels,
                (*pchunk).buffer,
                (*pchunk).size,
                &mut (*frame).width,
                &mut (*frame).height,
                if fuse_palette != 0 {
                    &mut (*frame).palette
                } else {
                    0 as *mut *mut libc::c_uchar
                },
                &mut (*frame).ncolors,
                reqcolors,
                &mut (*frame).pixelformat,
                bgcolor,
                &mut (*frame).transparent,
                (*pchunk).allocator,
            );
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                current_block = 3544816217675459574;
            } else {
                current_block = 721385680381463314;
            }
        }
    } else if chunk_is_gif(pchunk) != 0 {
        fnp.fn_0 = fn_load;
        status = load_gif(
            (*pchunk).buffer,
            (*pchunk).size as libc::c_int,
            bgcolor,
            reqcolors,
            fuse_palette,
            fstatic,
            loop_control,
            fnp.p,
            context,
            (*pchunk).allocator,
        );
        status & 0x1000 as libc::c_int != 0 as libc::c_int;
        current_block = 3544816217675459574;
    } else {
        let mut s: stbi__context = stbi__context {
            img_x: 0,
            img_y: 0,
            img_n: 0,
            img_out_n: 0,
            io: stbi_io_callbacks {
                read: None,
                skip: None,
                eof: None,
            },
            io_user_data: 0 as *mut libc::c_void,
            read_from_callbacks: 0,
            buflen: 0,
            buffer_start: [0; 128],
            callback_already_read: 0,
            img_buffer: 0 as *mut stbi_uc,
            img_buffer_end: 0 as *mut stbi_uc,
            img_buffer_original: 0 as *mut stbi_uc,
            img_buffer_original_end: 0 as *mut stbi_uc,
        };
        let mut depth: libc::c_int = 0;
        status = sixel_frame_new(&mut frame, (*pchunk).allocator);
        if status & 0x1000 as libc::c_int != 0 as libc::c_int {
            current_block = 3544816217675459574;
        } else {
            stbi_allocator = (*pchunk).allocator;
            stbi__start_mem(&mut s, (*pchunk).buffer, (*pchunk).size as libc::c_int);
            (*frame)
                .pixels = stbi__load_and_postprocess_8bit(
                &mut s,
                &mut (*frame).width,
                &mut (*frame).height,
                &mut depth,
                3 as libc::c_int,
            );
            if ((*frame).pixels).is_null() {
                sixel_helper_set_additional_message(stbi_failure_reason());
                status = 0x1000 as libc::c_int | 0xa00 as libc::c_int;
                current_block = 3544816217675459574;
            } else {
                (*frame).loop_count = 1 as libc::c_int;
                match depth {
                    1 | 3 | 4 => {
                        (*frame).pixelformat = 0 as libc::c_int | 0x3 as libc::c_int;
                        current_block = 721385680381463314;
                    }
                    _ => {
                        nwrite = sprintf(
                            message.as_mut_ptr(),
                            b"load_with_builtin() failed.\nreason: unknown pixel-format.(depth: %d)\n\0"
                                as *const u8 as *const libc::c_char,
                            depth,
                        );
                        if nwrite > 0 as libc::c_int {
                            sixel_helper_set_additional_message(message.as_mut_ptr());
                        }
                        current_block = 3544816217675459574;
                    }
                }
            }
        }
    }
    match current_block {
        721385680381463314 => {
            status = sixel_frame_strip_alpha(frame, bgcolor);
            if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                status = fn_load.expect("non-null function pointer")(frame, context);
                if !(status & 0x1000 as libc::c_int != 0 as libc::c_int) {
                    status = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    sixel_frame_unref(frame);
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn sixel_helper_load_image_file(
    mut filename: *const libc::c_char,
    mut fstatic: libc::c_int,
    mut fuse_palette: libc::c_int,
    mut reqcolors: libc::c_int,
    mut bgcolor: *mut libc::c_uchar,
    mut loop_control: libc::c_int,
    mut fn_load: sixel_load_image_function,
    mut finsecure: libc::c_int,
    mut cancel_flag: *const libc::c_int,
    mut context: *mut libc::c_void,
    mut allocator: *mut sixel_allocator_t,
) -> SIXELSTATUS {
    let mut status: SIXELSTATUS = 0x1000 as libc::c_int;
    let mut pchunk: *mut sixel_chunk_t = 0 as *mut sixel_chunk_t;
    if reqcolors > 256 as libc::c_int {
        reqcolors = 256 as libc::c_int;
    }
    status = sixel_chunk_new(&mut pchunk, filename, finsecure, cancel_flag, allocator);
    if !(status != 0 as libc::c_int) {
        if (*pchunk).size == 0 as libc::c_int as libc::c_ulong
            || (*pchunk).size == 1 as libc::c_int as libc::c_ulong
                && *(*pchunk).buffer as libc::c_int == '\n' as i32
        {
            status = 0 as libc::c_int;
        } else if ((*pchunk).buffer).is_null()
            || (*pchunk).max_size == 0 as libc::c_int as libc::c_ulong
        {
            status = 0x1000 as libc::c_int | 0x200 as libc::c_int;
        } else {
            status = 0x1000 as libc::c_int;
            if status & 0x1000 as libc::c_int != 0 as libc::c_int {
                status = load_with_builtin(
                    pchunk,
                    fstatic,
                    fuse_palette,
                    reqcolors,
                    bgcolor,
                    loop_control,
                    fn_load,
                    context,
                );
            }
            status & 0x1000 as libc::c_int != 0 as libc::c_int;
        }
    }
    sixel_chunk_destroy(pchunk);
    return status;
}
