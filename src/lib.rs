pub mod points;

use std::ffi::c_void;

#[repr(C)]
pub struct VecInfo {
    ptr: *mut c_void,
    length: usize,
    capacity: usize,
}

#[no_mangle]
pub extern "C" fn free_vec_info(ptr: *mut VecInfo) {
    unsafe { drop(Box::from_raw(ptr)) }
}