// SPDX-FileCopyrightText: © The `magic` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Internal Foreign Function Interface module for `magic_sys` / `libmagic`
//!
//! Contains `unsafe` as a medium level binding.

#![allow(unsafe_code)]

use magic_sys as libmagic;

/// Error for opened `magic_t` instance
#[derive(thiserror::Error, Debug)]
#[error("magic cookie error ({}): {}",
match .errno {
    Some(errno) => format!("OS errno: {}", errno),
    None => "no OS errno".to_string(),
},
.explanation.to_string_lossy()
)]
pub(crate) struct CookieError {
    explanation: std::ffi::CString,
    errno: Option<std::io::Error>,
}

fn last_error(cookie: libmagic::magic_t) -> Option<CookieError> {
    let error = unsafe { libmagic::magic_error(cookie) };
    let errno = unsafe { libmagic::magic_errno(cookie) };

    if error.is_null() {
        None
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(error) };
        Some(CookieError {
            explanation: c_str.into(),
            errno: match errno {
                0 => None,
                _ => Some(std::io::Error::from_raw_os_error(errno)),
            },
        })
    }
}

fn api_violation(cookie: libmagic::magic_t, description: String) -> ! {
    panic!(
        "`libmagic` API violation for magic cookie {:?}: {}",
        cookie, description
    );
}

fn expect_error(cookie: libmagic::magic_t, description: String) -> CookieError {
    match last_error(cookie) {
        Some(err) => err,
        _ => api_violation(cookie, description),
    }
}

pub(crate) fn close(cookie: libmagic::magic_t) {
    unsafe { libmagic::magic_close(cookie) }
}

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error.
pub(crate) fn file(
    cookie: libmagic::magic_t,
    filename: &std::ffi::CStr, // TODO: Support NULL
) -> Result<std::ffi::CString, CookieError> {
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

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error.
pub(crate) fn buffer(
    cookie: libmagic::magic_t,
    buffer: &[u8],
) -> Result<std::ffi::CString, CookieError> {
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

pub(crate) fn setflags(cookie: libmagic::magic_t, flags: libc::c_int) -> Result<(), SetFlagsError> {
    let ret = unsafe { libmagic::magic_setflags(cookie, flags) };
    match ret {
        -1 => Err(SetFlagsError { flags }),
        _ => Ok(()),
    }
}

#[derive(thiserror::Error, Debug)]
#[error("could not set magic cookie flags")]
pub(crate) struct SetFlagsError {
    flags: libc::c_int,
}

impl SetFlagsError {
    pub fn flags(&self) -> libc::c_int {
        self.flags
    }
}

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn check(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_check(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_check()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("expected 0 or -1 but `magic_check()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn compile(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_compile(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_compile()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("Expected 0 or -1 but `magic_compile()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn list(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_list(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_list()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("Expected 0 or -1 but `magic_list()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn load(
    cookie: libmagic::magic_t,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_load(cookie, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("Expected 0 or -1 but `magic_load()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn load_buffers(
    cookie: libmagic::magic_t,
    buffers: &[&[u8]],
) -> Result<(), CookieError> {
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
        res => api_violation(
            cookie,
            format!(
                "Expected 0 or -1 but `magic_load_buffers()` returned {}",
                res
            ),
        ),
    }
}

pub(crate) fn open(flags: libc::c_int) -> Result<libmagic::magic_t, OpenError> {
    let cookie = unsafe { libmagic::magic_open(flags) };

    if cookie.is_null() {
        Err(OpenError {
            flags,
            // note that libmagic only really cares about MAGIC_PRESERVE_ATIME
            // invalid bits in `flags` still result in a valid cookie pointer
            errno: std::io::Error::last_os_error(),
        })
    } else {
        Ok(cookie)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("could not open magic cookie")]
pub(crate) struct OpenError {
    flags: libc::c_int,
    errno: std::io::Error,
}

impl OpenError {
    pub fn flags(&self) -> libc::c_int {
        self.flags
    }

    pub fn errno(&self) -> &std::io::Error {
        &self.errno
    }
}

pub(crate) fn version() -> libc::c_int {
    unsafe { libmagic::magic_version() }
}
