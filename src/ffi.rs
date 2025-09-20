// SPDX-FileCopyrightText: © The `magic` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Internal Foreign Function Interface module for `magic_sys` / `libmagic`
//!
//! Contains `unsafe` as a medium level binding.

#![allow(unsafe_code)]

use magic_sys as libmagic;

#[derive(Debug)]
// non-copy wrapper around raw pointer
#[repr(transparent)]
pub(crate) struct Cookie(libmagic::magic_t);

impl Cookie {
    pub fn new(cookie: &mut Self) -> Self {
        Self(cookie.0)
    }
}

/// Error for opened `magic_t` instance
#[derive(thiserror::Error, Debug)]
pub(crate) enum CookieError {
    #[error("cookie error: {0}")]
    Error(#[from] Error),
    #[error("cookie API violation: {0}")]
    ApiViolation(#[from] ApiViolation),
}

/// Combined error value from `magic_erro` and `magic_errno`
#[derive(thiserror::Error, Debug)]
#[error("libmagic error ({}): {}",
match .errno {
    Some(errno) => format!("OS errno: {}", errno),
    None => "no OS errno".to_string(),
},
.explanation.to_string_lossy()
)]
pub(crate) struct Error {
    explanation: std::ffi::CString,
    errno: Option<std::io::Error>,
}

/// Violation of the documented `libmagic` API
#[derive(thiserror::Error, Debug)]
pub(crate) enum ApiViolation {
    /// `magic_error` returned no error (`NULL`)
    #[error("{0}")]
    MissingError(&'static str),
    /// A `libmagic` function returned an unexpected value
    #[error("{0}")]
    UnexpectedReturnValue(String),
}

fn last_error(cookie: &Cookie) -> Option<Error> {
    let error = unsafe { libmagic::magic_error(cookie.0) };
    let errno = unsafe { libmagic::magic_errno(cookie.0) };

    if error.is_null() {
        None
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(error) };
        Some(Error {
            explanation: c_str.into(),
            errno: match errno {
                0 => None,
                _ => Some(std::io::Error::from_raw_os_error(errno)),
            },
        })
    }
}

fn expect_error(cookie: &Cookie, description: &'static str) -> CookieError {
    match last_error(cookie) {
        Some(error) => error.into(),
        None => ApiViolation::MissingError(description).into(),
    }
}

pub(crate) fn close(cookie: &mut Cookie) {
    unsafe { libmagic::magic_close(cookie.0) }
}

pub(crate) fn file(
    cookie: &Cookie,
    filename: &std::ffi::CStr, // TODO: Support NULL
) -> Result<std::ffi::CString, CookieError> {
    let filename_ptr = filename.as_ptr();
    let res = unsafe { libmagic::magic_file(cookie.0, filename_ptr) };

    if res.is_null() {
        Err(expect_error(
            cookie,
            "`magic_file()` did not set last error",
        ))
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn buffer(cookie: &Cookie, buffer: &[u8]) -> Result<std::ffi::CString, CookieError> {
    let buffer_ptr = buffer.as_ptr();
    let buffer_len = buffer.len() as libc::size_t;
    let res = unsafe { libmagic::magic_buffer(cookie.0, buffer_ptr, buffer_len) };

    if res.is_null() {
        Err(expect_error(
            cookie,
            "`magic_buffer()` did not set last error",
        ))
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn setflags(cookie: &Cookie, flags: libc::c_int) -> Result<(), SetFlagsError> {
    let ret = unsafe { libmagic::magic_setflags(cookie.0, flags) };
    match ret {
        -1 => Err(SetFlagsError { flags }),
        _ => Ok(()),
    }
}

#[derive(thiserror::Error, Debug)]
#[error("could not set magic cookie flags {}", .flags)]
pub(crate) struct SetFlagsError {
    flags: libc::c_int,
}

pub(crate) fn check(cookie: &Cookie, filename: Option<&std::ffi::CStr>) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_check(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_check()` did not set last error",
        )),
        res => Err(ApiViolation::UnexpectedReturnValue(format!(
            "expected 0 or -1 but `magic_check()` returned {}",
            res
        ))
        .into()),
    }
}

pub(crate) fn compile(
    cookie: &Cookie,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_compile(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_compile()` did not set last error",
        )),
        res => Err(ApiViolation::UnexpectedReturnValue(format!(
            "Expected 0 or -1 but `magic_compile()` returned {}",
            res
        ))
        .into()),
    }
}

pub(crate) fn list(cookie: &Cookie, filename: Option<&std::ffi::CStr>) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_list(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_list()` did not set last error",
        )),
        res => Err(ApiViolation::UnexpectedReturnValue(format!(
            "Expected 0 or -1 but `magic_list()` returned {}",
            res
        ))
        .into()),
    }
}

pub(crate) fn load(cookie: &Cookie, filename: Option<&std::ffi::CStr>) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { libmagic::magic_load(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load()` did not set last error",
        )),
        res => Err(ApiViolation::UnexpectedReturnValue(format!(
            "Expected 0 or -1 but `magic_load()` returned {}",
            res
        ))
        .into()),
    }
}

pub(crate) fn load_buffers(cookie: &Cookie, buffers: &[&[u8]]) -> Result<(), CookieError> {
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
        libmagic::magic_load_buffers(cookie.0, ffi_buffers_ptr, ffi_sizes_ptr, ffi_nbuffers)
    };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load_buffers()` did not set last error",
        )),
        res => Err(ApiViolation::UnexpectedReturnValue(format!(
            "Expected 0 or -1 but `magic_load_buffers()` returned {}",
            res
        ))
        .into()),
    }
}

pub(crate) fn open(flags: libc::c_int) -> Result<Cookie, OpenError> {
    let cookie = unsafe { libmagic::magic_open(flags) };

    if cookie.is_null() {
        Err(OpenError {
            flags,
            // note that libmagic only really cares about MAGIC_PRESERVE_ATIME
            // invalid bits in `flags` still result in a valid cookie pointer
            errno: std::io::Error::last_os_error(),
        })
    } else {
        Ok(Cookie(cookie))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("could not open magic cookie with flags {}: {}", .flags, .errno)]
pub(crate) struct OpenError {
    flags: libc::c_int,
    errno: std::io::Error,
}

impl OpenError {
    pub fn errno(&self) -> &std::io::Error {
        &self.errno
    }
}

pub(crate) fn version() -> libc::c_int {
    unsafe { libmagic::magic_version() }
}

#[cfg(test)]
mod tests {
    use super::{ApiViolation, CookieError, Error, OpenError, SetFlagsError};

    fn assert_impl_debug<T: std::fmt::Debug>() {}
    fn assert_impl_display<T: std::fmt::Display>() {}
    fn assert_impl_error<T: std::error::Error>() {}

    #[test]
    fn error_impls() {
        assert_impl_debug::<Error>();
        assert_impl_display::<Error>();
        assert_impl_error::<Error>();
    }

    #[test]
    fn apiviolation_impls() {
        assert_impl_debug::<ApiViolation>();
        assert_impl_display::<ApiViolation>();
        assert_impl_error::<ApiViolation>();
    }

    #[test]
    fn cookieerror_impls() {
        assert_impl_debug::<CookieError>();
        assert_impl_display::<CookieError>();
        assert_impl_error::<CookieError>();
    }

    #[test]
    fn openerror_impls() {
        assert_impl_debug::<OpenError>();
        assert_impl_display::<OpenError>();
        assert_impl_error::<OpenError>();
    }

    #[test]
    fn setflagserror_impls() {
        assert_impl_debug::<SetFlagsError>();
        assert_impl_display::<SetFlagsError>();
        assert_impl_error::<SetFlagsError>();
    }
}
