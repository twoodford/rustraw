extern crate libc;

use std::error;
use std::fmt;
use std::path;
use std::ffi::CString;
use std::marker::PhantomData;
use libc::types::os::arch::c95::{c_char, time_t};
use libc::types::common::c95::c_void;
use libc::funcs::c95::stdlib::free;

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
struct LibrawColorData {
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
struct LibrawPh1 {
    format: i32,
    key_off: i32,
    t_black: i32,
    black_off: i32,
    split_col: i32,
    tag_21a: i32,
    tag_210: f32,
}

#[repr(C)]
struct LibrawImgOther {
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
struct LibrawThumb {
    tformat: LibrawThumbnailFormat,
    twidth: u16,
    theight: u16,
    tlength: u32,
    tcolors: i32,
    thumb: *mut c_char,
}

#[repr(C)]
struct LibrawRawData {
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
struct LibrawInternalOutputParams {
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

// Errors
pub enum LibrawError {
    NulError(std::ffi::NulError),
    AllocError(LibrawAllocError),
    IOError(std::io::Error),
    LibError(LibrawLibraryError),
}

#[derive(Debug)]
pub struct LibrawAllocError;

impl fmt::Display for LibrawAllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "image data allocation failed".fmt(f)
    }
}

impl error::Error for LibrawAllocError {
    fn description(&self) -> &str {
        "image data allocation failed"
    }
}

#[derive(Debug)]
pub struct LibrawLibraryError;

impl fmt::Display for LibrawLibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "internal libraw library error".fmt(f)
    }
}

impl error::Error for LibrawLibraryError {
    fn description(&self) -> &str {
        "internal libraw library error"
    }
}

// Higher-level unsafe methods
// These methods may change as I find better ways to implement this
// Specifically, it might be better to read the file in Rust and pass 
// the resulting buffer to libraw
unsafe fn init_data() -> Result<*mut LibrawData, LibrawError> {
    // 0 = no flags
    // 1 = no memory error callback
    // 2 = non data error callback
    // 3 = both
    let data = libraw_init(3);
    if data.is_null() {
        Err(LibrawError::AllocError(LibrawAllocError))
    } else {
        Ok(data)
    }
}

unsafe fn read_img_at_path(filepath: &str) -> Result<*mut LibrawData, LibrawError> {
    let fpath = match CString::new(filepath) {
        Ok(v) => v,
        Err(e) => return Err(LibrawError::NulError(e)),
    };
    let data = try!(init_data());
    let res: i32 = libraw_open_file(data, fpath.as_ptr());
    if res > 0 {
        Err(LibrawError::IOError(std::io::Error::from_raw_os_error(res)))
    } else if res < 0 {
        Err(LibrawError::LibError(LibrawLibraryError))
    } else {
        Ok(data)
    }
}

// Public interface
pub fn load_raw_at_path<'a>(fpath: &'a path::Path) -> Result<RawData<'a>, LibrawError> {
    let pathstr = match fpath.to_str() {
        Some(v) => v,
        None => return Err(LibrawError::AllocError(LibrawAllocError)),
    };
    unsafe {
        let unsafe_val = match read_img_at_path(pathstr) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
    
        let raw_dat: RawData = RawData{
            libraw_data: unsafe_val,
            phantom: PhantomData
        };
        Ok(raw_dat)
    }
}

pub struct RawData<'a> {
    libraw_data: *mut LibrawData, // Needs to be mutable for free()
    phantom: PhantomData<&'a LibrawData>,
}

impl<'a> Drop for RawData<'a> {
    fn drop(&mut self) {
        unsafe {
            free((self.libraw_data as *mut c_void));
        }
    }
}
