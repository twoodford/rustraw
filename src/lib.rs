extern crate libc;

use std::ffi::CString;
use libc::types::os::arch::c95::{c_char, time_t};

// C function definitions
#[link(name = "raw")]
extern {
    fn libraw_init(flags: u32) -> *mut LibrawData;
    fn libraw_open_file(imgdat: *mut LibrawData, fpath: *const libc::c_char) -> i32;
    fn libraw_open_buffer(imgdat: *mut LibrawData, buffer: &[u8], bufsize: libc::size_t) -> i32;
    fn libraw_unpack(imgdat: *mut LibrawData) -> i32;
    fn libraw_unpack_thumb(imgdat: *mut LibrawData) -> i32;
}

// C struct definitions
#[repr(C)]
pub struct LibrawData {
    image: [*mut u16; 4],
    sizes: LibrawImageSizes,
    idata: LibrawIparams,
    progress_flags: u32,
    process_warnings: u32,
    color: LibrawColorData,
    other: LibrawImgOther,
    thumbnail: LibrawThumb,
    rawdata: LibrawRawData,
    parent_class: *mut libc::c_void,
}

#[repr(C)]
pub struct LibrawImageSizes {
    raw_height: u16,
    raw_width: u16,
    height: u16,
    width: u16,
    top_margin: u16,
    left_margin: u16,
    iheight: u16,
    iwidth: u16,
    raw_pitch: u32,
    pixel_aspect: f64,
    flip: i32,
    mask: [[u32; 8]; 4],
}

#[repr(C)]
pub struct LibrawIparams {
    make: [c_char; 64],
    model: [c_char; 64],
    raw_count: u32,
    dng_version: u32,
    is_foveon: u32,
    colors: i32,
    filters: u32,
    xtrans: [[c_char; 6]; 6],
    cdesc: [c_char; 5],
}

#[repr(C)]
pub struct LibrawColorData {
    make: [c_char; 64],
    model: [c_char; 64],
    raw_count: u32,
    dng_version: u32,
    is_foveon: u32,
    colors: i32,
    filters: u32,
    xtrans: [[c_char; 6]; 6],
    cdesc: [c_char; 5],
    phase_one_data: LibrawPh1,
    flash_used: f32,
    canon_ev: f32,
    model2: [c_char; 64],
    profile: *mut libc::c_void,
    profile_length: u32,
    black_stat: [u32; 8],
}

#[repr(C)]
pub struct LibrawPh1 {
    format: i32,
    key_off: i32,
    t_black: i32,
    black_off: i32,
    split_col: i32,
    tag_21a: i32,
    tag_210: f32,
}

#[repr(C)]
pub struct LibrawImgOther {
    iso_speed: f32,
    shutter: f32,
    aperture: f32,
    focal_len: f32,
    timestamp: time_t,
    shot_order: u32,
    gpsdata: [u32; 32],
    desc: [c_char; 512],
    artist: [c_char; 64],
}

#[repr(C)]
pub struct LibrawThumb {
    tformat: LibrawThumbnailFormat,
    twidth: u16,
    theight: u16,
    tlength: u32,
    tcolors: i32,
    thumb: *mut c_char,
}

#[repr(C)]
pub struct LibrawRawData {
    raw_alloc: *mut libc::c_void,
    raw_image: *mut u16,
    color4_image: [*mut u16; 4],
    color3_image: [*mut u16; 3],
    ph1_black: [*mut i16; 2],
    iparams: LibrawIparams,
    sizes: LibrawImageSizes,
    ioparams: LibrawInternalOutputParams,
    color: LibrawColorData,
}

#[repr(C)]
pub struct LibrawInternalOutputParams {
    mix_green: u32,
    raw_color: u32,
    zero_is_bad: u32,
    shrink: u16,
    fuji_width: u16,
}

// C enum definitions
#[repr(C)]
enum LibrawThumbnailFormat {
    Unknown = 0,
    Jpeg = 1,
    Bitmap = 2,
    Layer = 4,
    Rollei = 5,
}
