extern crate libc;

use std::ffi::CString;

// C function definitions
#[link(name = "raw")]
extern {
    fn libraw_init(u32 flags) -> *mut LibrawData;
    fn libraw_open_file(imgdat: *mut LibrawData, fpath: *const libc::c_char) -> i32;
    fn libraw_open_buffer(imgdat: *mut LibrawData, buffer: &[u8], bufsize: libc::size_t) -> i32;
    fn libraw_unpack(imgdat: *mut LibrawData) -> i32;
    fn libraw_unpack_thumb(imgdat: *mut LibrawData) -> i32;
}

// C struct definitions
#[repr(C)]
struct LibrawData {
    image: [*u16, 4],
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
struct LibrawImageSizes {
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
struct LibrawIparams {
    make: [libc::types::os::arch::c_char; 64],
    model: [libc::types::os::arch::c_char; 64],
    raw_count: u32,
    dng_version: u32,
    is_foveon: u32,
    colors: i32,
    filters: u32,
    xtrans: [[libc::types::os::arch::c_char; 6]; 6],
    cdesc: [libc::types::os::arch::c_char; 5],
}
