use libc;
use std::ffi::CString;
use std::slice;
use libsolv_sys::solv_free;
use std::ptr;
use std::marker;

use ::errors::*;

pub trait SolvTake {
    type Input;

    unsafe fn solv_take(ptr: &mut *const Self::Input) -> Result<Self> where Self: marker::Sized;

    unsafe fn solv_take_mut(ptr: &mut *mut Self::Input) -> Result<Self> where Self: marker::Sized;
}

impl SolvTake for CString {
    type Input = libc::c_char;

    unsafe fn solv_take(ptr: &mut *const Self::Input) -> Result<Self> where Self: marker::Sized {
        let len = libc::strlen(*ptr);
        let slice = slice::from_raw_parts(*ptr as *const libc::c_uchar, len as usize);
        let cstr = CString::new(slice);
        solv_free(*ptr as *mut libc::c_void);
        *ptr = ptr::null();
        cstr.chain_err(|| "invalid cstr ptr")
    }

    unsafe fn solv_take_mut(ptr: &mut *mut Self::Input) -> Result<Self> where Self: marker::Sized {
        let len = libc::strlen(*ptr);
        let slice = slice::from_raw_parts(*ptr as *const libc::c_uchar, len as usize);
        let cstr = CString::new(slice);
        solv_free(*ptr as *mut libc::c_void);
        *ptr = ptr::null_mut();
        cstr.chain_err(|| "invalid cstr ptr")
    }
}
