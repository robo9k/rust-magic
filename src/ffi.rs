//! Internal Foreign Function Interface module for `magic_sys` / `libmagic`
//!
//! Contains `unsafe` as a medium level binding.

#![allow(unsafe_code)]

extern crate libc;
extern crate magic_sys as libmagic;
extern crate thiserror;

use std::ffi::CStr;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub(crate) enum LibmagicError {
    /// Error during `magic_open`
    #[error("Error calling `magic_open`, errno: {errno}")]
    Open {
        #[source]
        errno: errno::Errno,
    },

    /// Error for opened `magic_t` instance
    #[error("Error for cookie call ({}): {explanation}", match .errno {
        Some(errno) => format!("OS errno: {}", errno),
        None => "no OS errno".to_string(),
    })]
    Cookie {
        explanation: String,
        #[source]
        errno: Option<errno::Errno>,
    },
    #[error("Error calling `magic_setflags`, unsupported flags: {flags}")]
    UnsupportedFlags { flags: libc::c_int },
}

pub(crate) fn last_error(cookie: self::libmagic::magic_t) -> Option<LibmagicError> {
    unsafe {
        let error = self::libmagic::magic_error(cookie);
        let errno = self::libmagic::magic_errno(cookie);
        if error.is_null() {
            None
        } else {
            let slice = CStr::from_ptr(error).to_bytes();
            Some(LibmagicError::Cookie {
                explanation: std::str::from_utf8(slice).unwrap().to_string(),
                errno: match errno {
                    0 => None,
                    _ => Some(errno::Errno(errno)),
                },
            })
        }
    }
}

pub(crate) fn close(cookie: self::libmagic::magic_t) {
    unsafe { self::libmagic::magic_close(cookie) }
}

pub(crate) fn file(
    cookie: self::libmagic::magic_t,
    filename: &std::ffi::CStr, // TODO: Support NULL
) -> Result<std::ffi::CString, LibmagicError> {
    let filename_ptr = filename.as_ptr();
    let res = unsafe { self::libmagic::magic_file(cookie, filename_ptr) };

    if res.is_null() {
        Err(last_error(cookie).unwrap())
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn buffer(
    cookie: self::libmagic::magic_t,
    buffer: &[u8],
) -> Result<std::ffi::CString, LibmagicError> {
    let buffer_ptr = buffer.as_ptr();
    let buffer_len = buffer.len() as libc::size_t;
    let res = unsafe { self::libmagic::magic_buffer(cookie, buffer_ptr, buffer_len) };

    if res.is_null() {
        Err(last_error(cookie).unwrap())
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn setflags(
    cookie: self::libmagic::magic_t,
    flags: libc::c_int,
) -> Result<(), LibmagicError> {
    let ret = unsafe { self::libmagic::magic_setflags(cookie, flags) };
    match ret {
        -1 => Err(LibmagicError::UnsupportedFlags { flags }),
        _ => Ok(()),
    }
}

pub(crate) fn check(
    cookie: self::libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { self::libmagic::magic_check(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(last_error(cookie).unwrap()),
        res => panic!("libmagic API violation: `magic_check()` returned {}", res),
    }
}

pub(crate) fn compile(
    cookie: self::libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { self::libmagic::magic_compile(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(last_error(cookie).unwrap()),
        res => panic!("libmagic API violation: `magic_compile()` returned {}", res),
    }
}

pub(crate) fn list(
    cookie: self::libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { self::libmagic::magic_list(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(last_error(cookie).unwrap()),
        res => panic!("libmagic API violation: `magic_list()` returned {}", res),
    }
}

pub(crate) fn load(
    cookie: self::libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { self::libmagic::magic_load(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(last_error(cookie).unwrap()),
        res => panic!("libmagic API violation: `magic_load()` returned {}", res),
    }
}

pub(crate) fn load_buffers(
    cookie: self::libmagic::magic_t,
    buffers: &[&[u8]],
) -> Result<(), LibmagicError> {
    let mut ffi_buffers: Vec<*const u8> = Vec::with_capacity(buffers.len());
    let mut ffi_sizes: Vec<libc::size_t> = Vec::with_capacity(buffers.len());
    let ffi_nbuffers = buffers.len() as libc::size_t;

    for slice in buffers {
        ffi_buffers.push((*slice).as_ptr());
        ffi_sizes.push(slice.len() as libc::size_t);
    }

    let ffi_buffers_ptr = ffi_buffers.as_mut_ptr() as *mut *mut libc::c_void;
    let ffi_sizes_ptr = ffi_sizes.as_mut_ptr();

    let res = unsafe {
        self::libmagic::magic_load_buffers(cookie, ffi_buffers_ptr, ffi_sizes_ptr, ffi_nbuffers)
    };

    match res {
        0 => Ok(()),
        -1 => Err(last_error(cookie).unwrap()),
        res => panic!(
            "libmagic API violation: `magic_load_buffers()` returned {}",
            res
        ),
    }
}

pub(crate) fn open(flags: libc::c_int) -> Result<self::libmagic::magic_t, LibmagicError> {
    let cookie = unsafe { self::libmagic::magic_open(flags) };

    if cookie.is_null() {
        Err(LibmagicError::Open {
            errno: errno::errno(),
        })
    } else {
        Ok(cookie)
    }
}
