pub mod points;

use std::ffi::c_void;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum AlignN {
    Align4,
    Align8,
    Align16,
}

#[repr(C)]
pub struct VecInfo {
    pub ptr: *mut c_void,
    pub length: usize,
    pub capacity: usize,
}