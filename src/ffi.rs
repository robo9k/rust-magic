// SPDX-FileCopyrightText: © The `magic` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Internal Foreign Function Interface module for `magic_sys` / `libmagic`
//!
//! Contains `unsafe` as a medium level binding.

#![allow(unsafe_code)]

use magic_sys as libmagic;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub(crate) enum LibmagicError {
    /// Error during `magic_open`
    #[error("Error calling `magic_open`, errno: {errno}")]
    Open { errno: errno::Errno },

    /// Error for opened `magic_t` instance
    #[error("Error for cookie call ({}): {}",
        match .errno {
            Some(errno) => format!("OS errno: {}", errno),
            None => "no OS errno".to_string(),
        },
        .explanation.to_string_lossy()
    )]
    Cookie {
        explanation: std::ffi::CString,
        errno: Option<errno::Errno>,
    },

    /// Error during `magic_setflags`
    #[error("Error calling `magic_setflags`, unsupported flags: {flags}")]
    UnsupportedFlags { flags: libc::c_int },

    /// `libmagic` did not behave according to its API
    #[error("Error in `libmagic` behavior, violated its API: {description}")]
    ApiViolation { description: String },
}

fn last_error(cookie: libmagic::magic_t) -> Option<LibmagicError> {
    let error = unsafe { libmagic::magic_error(cookie) };
    let errno = unsafe { libmagic::magic_errno(cookie) };

    if error.is_null() {
        None
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(error) };
        Some(LibmagicError::Cookie {
            explanation: c_str.into(),
            errno: match errno {
                0 => None,
                _ => Some(errno::Errno(errno)),
            },
        })
    }
}

fn expect_error(cookie: libmagic::magic_t, description: String) -> LibmagicError {
    match last_error(cookie) {
        Some(err) => err,
        None => LibmagicError::ApiViolation { description },
    }
}

pub(crate) fn close(cookie: libmagic::magic_t) {
    unsafe { libmagic::magic_close(cookie) }
}

pub(crate) fn file(
    cookie: libmagic::magic_t,
    filename: &std::ffi::CStr, // TODO: Support NULL
) -> Result<std::ffi::CString, LibmagicError> {
    let filename_ptr = filename.as_ptr();
    let res = unsafe { libmagic::magic_file(cookie, filename_ptr) };

    if res.is_null() {
        Err(expect_error(
            cookie,
            "`magic_file()` did not set last error".to_string(),
        ))
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn buffer(
    cookie: libmagic::magic_t,
    buffer: &[u8],
) -> Result<std::ffi::CString, LibmagicError> {
    let buffer_ptr = buffer.as_ptr();
    let buffer_len = buffer.len() as libc::size_t;
    let res = unsafe { libmagic::magic_buffer(cookie, buffer_ptr, buffer_len) };

    if res.is_null() {
        Err(expect_error(
            cookie,
            "`magic_buffer()` did not set last error".to_string(),
        ))
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn setflags(cookie: libmagic::magic_t, flags: libc::c_int) -> Result<(), LibmagicError> {
    let ret = unsafe { libmagic::magic_setflags(cookie, flags) };
    match ret {
        -1 => Err(LibmagicError::UnsupportedFlags { flags }),
        _ => Ok(()),
    }
}

pub(crate) fn check(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_check(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_check()` did not set last error".to_string(),
        )),
        res => Err(LibmagicError::ApiViolation {
            description: format!("Expected 0 or -1 but `magic_check()` returned {}", res),
        }),
    }
}

pub(crate) fn compile(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_compile(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_compile()` did not set last error".to_string(),
        )),
        res => Err(LibmagicError::ApiViolation {
            description: format!("Expected 0 or -1 but `magic_compile()` returned {}", res),
        }),
    }
}

pub(crate) fn list(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_list(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_list()` did not set last error".to_string(),
        )),
        res => Err(LibmagicError::ApiViolation {
            description: format!("Expected 0 or -1 but `magic_list()` returned {}", res),
        }),
    }
}

pub(crate) fn load(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), LibmagicError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_load(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load()` did not set last error".to_string(),
        )),
        res => Err(LibmagicError::ApiViolation {
            description: format!("Expected 0 or -1 but `magic_load()` returned {}", res),
        }),
    }
}

pub(crate) fn load_buffers(
    cookie: libmagic::magic_t,
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
        libmagic::magic_load_buffers(cookie, ffi_buffers_ptr, ffi_sizes_ptr, ffi_nbuffers)
    };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load_buffers()` did not set last error".to_string(),
        )),
        res => Err(LibmagicError::ApiViolation {
            description: format!(
                "Expected 0 or -1 but `magic_load_buffers()` returned {}",
                res
            ),
        }),
    }
}

pub(crate) fn open(flags: libc::c_int) -> Result<libmagic::magic_t, LibmagicError> {
    let cookie = unsafe { libmagic::magic_open(flags) };

    if cookie.is_null() {
        Err(LibmagicError::Open {
            errno: errno::errno(),
        })
    } else {
        Ok(cookie)
    }
}
