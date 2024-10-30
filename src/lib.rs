pub mod points;

use std::ffi::c_void;

#[repr(C)]
pub struct VecInfo {
    pub ptr: *mut c_void,
    pub length: usize,
    pub capacity: usize,
}