//! # About
//!
//! This crate provides bindings for the `libmagic` C library, which recognizes the
//! type of data contained in a file (or buffer).
//!
//! You might be familiar with `libmagic`'s CLI; [`file`](https://www.darwinsys.com/file/):
//!
//! ```sh
//! $ file data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//! ```
//!
//! ## `libmagic`
//!
//! Understanding how the `libmagic` C library and thus this crate works requires a bit of glossary.
//!
//! `libmagic` at its core can analyze a file or buffer and return a mostly unstructured text that describes the analysis result.
//! There are built-in tests for special cases and there are magic databases with signatures which can be supplied by the user for the generic cases.
//!
//! The analysis behaviour can be influenced by so-called flags and parameters.
//! Flags are either set or unset and do not have a value, parameters have a value.
//!
//! Databases can be in text form or compiled binary form for faster access. They can be loaded from files on disk or from in-memory buffers.
//! A regular `libmagic` / `file` installation contains a default database file that includes a plethora of file formats.
//!
//! Most `libmagic` functionality requires a configured instance which is called a "magic cookie".
//! Creating a cookie instance requires initial flags and usually loaded databases.
//!
//! # Usage example
//!
//! ```rust
//! # fn main() -> Result<(), magic::MagicError> {
//! // Open a new configuration with flags
//! let cookie = magic::Cookie::open(magic::CookieFlags::ERROR)?;
//! // Load the default database
//! cookie.load::<&str>(&[])?;
//! // Load an additional database for demonstration purposes
//! cookie.load(&vec!["data/tests/db-images-png"])?;
//!
//! // Analyze a test file
//! let file_to_analyze = "data/tests/rust-logo-128x128-blk.png";
//! let expected_analysis_result = "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced";
//! assert_eq!(cookie.file(&file_to_analyze)?, expected_analysis_result);
//! # Ok(())
//! # }
//! ```
//! # Further reading
//!
//! * [`Cookie::open`]
//! * [`CookieFlags`], in particular [`CookieFlags::ERROR`], [`CookieFlags::NO_CHECK_BUILTIN`]
//! * [`Cookie::load`], [`Cookie::load_buffers`]
//! * [`Cookie::file`], [`Cookie::buffer`]
//!
//! # Safety
//!
//! This crate is a binding to the `libmagic` C library and as such subject to its security problems.
//! Please note that `libmagic` has several CVEs, listed on e.g. [Repology](https://repology.org/project/file/cves).
//! Make sure that you are using an up-to-date version of `libmagic` and ideally
//! add additional security layers such as sandboxing (which this crate does _not_ provide)
//! and __do not use it on untrusted input__ e.g. from users on the internet!
//!
//! The Rust code of this crate needs to use some `unsafe` for interacting with the `libmagic` C FFI.
//!
//! This crate has not been audited nor is it ready for production use.
//!

extern crate libc;
extern crate magic_sys as ffi;
#[macro_use]
extern crate bitflags;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;
extern crate errno;
extern crate thiserror;

use libc::{c_char, c_int, size_t};
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::str;
use thiserror::Error;

bitflags! {
    /// Bitmask flags that specify how `Cookie` functions should behave
    ///
    /// NOTE: The descriptions are taken from `man libmagic 3`.
    ///
    /// `MAGIC_NONE` is the default, meaning "No special handling".
    /// ```
    /// let default_flags: magic::CookieFlags = Default::default();
    /// assert_eq!(default_flags, magic::CookieFlags::empty());
    /// ```
    #[derive(Default)]
    pub struct CookieFlags: c_int {
        // MAGIC_NONE is 0/default, see https://docs.rs/bitflags/1.3.2/bitflags/#zero-flags

        /// Print debugging messages to `stderr`
        ///
        /// NOTE: Those messages are printed by `libmagic` itself, no this Rust crate.
        #[doc(alias = "MAGIC_DEBUG")]
        const DEBUG             = self::ffi::MAGIC_DEBUG;

        /// If the file queried is a symlink, follow it
        #[doc(alias = "MAGIC_SYMLINK")]
        const SYMLINK           = self::ffi::MAGIC_SYMLINK;

        /// If the file is compressed, unpack it and look at the contents
        #[doc(alias = "MAGIC_COMPRESS")]
        const COMPRESS          = self::ffi::MAGIC_COMPRESS;

        /// If the file is a block or character special device, then open the device and try to look in its contents
        #[doc(alias = "MAGIC_DEVICES")]
        const DEVICES           = self::ffi::MAGIC_DEVICES;

        /// Return a MIME type string, instead of a textual description
        #[doc(alias = "MAGIC_MIME_TYPE")]
        const MIME_TYPE         = self::ffi::MAGIC_MIME_TYPE;

        /// Return all matches, not just the first
        #[doc(alias = "MAGIC_CONTINUE")]
        const CONTINUE          = self::ffi::MAGIC_CONTINUE;

        /// Check the magic database for consistency and print warnings to `stderr`
        ///
        /// NOTE: Those warnings are printed by `libmagic` itself, no this Rust crate.
        #[doc(alias = "MAGIC_CHECK")]
        const CHECK             = self::ffi::MAGIC_CHECK;

        /// On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed
        #[doc(alias = "MAGIC_PRESERVE_ATIME")]
        const PRESERVE_ATIME    = self::ffi::MAGIC_PRESERVE_ATIME;

        /// Don't translate unprintable characters to a `\\ooo` octal representation
        #[doc(alias = "MAGIC_RAW")]
        const RAW               = self::ffi::MAGIC_RAW;

        /// Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer
        #[doc(alias = "MAGIC_ERROR")]
        const ERROR             = self::ffi::MAGIC_ERROR;

        /// Return a MIME encoding, instead of a textual description
        #[doc(alias = "MAGIC_MIME_ENCODING")]
        const MIME_ENCODING     = self::ffi::MAGIC_MIME_ENCODING;

        /// A shorthand for `MIME_TYPE | MIME_ENCODING`
        #[doc(alias = "MAGIC_MIME")]
        const MIME              = Self::MIME_TYPE.bits
                                | Self::MIME_ENCODING.bits;

        /// Return the Apple creator and type
        #[doc(alias = "MAGIC_APPLE")]
        const APPLE             = self::ffi::MAGIC_APPLE;

        /// Return a slash-separated list of extensions for this file type
        #[doc(alias = "MAGIC_EXTENSION")]
        const EXTENSION         = self::ffi::MAGIC_EXTENSION;

        /// Don't report on compression, only report about the uncompressed data
        #[doc(alias = "MAGIC_COMPRESS_TRANSP")]
        const COMPRESS_TRANSP   = self::ffi::MAGIC_COMPRESS_TRANSP;

        /// A shorthand for `EXTENSION | MIME | APPLE`
        #[doc(alias = "MAGIC_NODESC")]
        const NODESC            = Self::EXTENSION.bits
                                | Self::MIME.bits
                                | Self::APPLE.bits;

        /// Don't look inside compressed files
        #[doc(alias = "MAGIC_NO_CHECK_COMPRESS")]
        const NO_CHECK_COMPRESS = self::ffi::MAGIC_NO_CHECK_COMPRESS;

        /// Don't examine tar files
        #[doc(alias = "MAGIC_NO_CHECK_TAR")]
        const NO_CHECK_TAR      = self::ffi::MAGIC_NO_CHECK_TAR;

        /// Don't consult magic files
        #[doc(alias = "MAGIC_NO_CHECK_SOFT")]
        const NO_CHECK_SOFT     = self::ffi::MAGIC_NO_CHECK_SOFT;

        /// Check for EMX application type (only on EMX)
        #[doc(alias = "MAGIC_NO_CHECK_APPTYPE")]
        const NO_CHECK_APPTYPE  = self::ffi::MAGIC_NO_CHECK_APPTYPE;

        /// Don't print ELF details
        #[doc(alias = "MAGIC_NO_CHECK_ELF")]
        const NO_CHECK_ELF      = self::ffi::MAGIC_NO_CHECK_ELF;

        /// Don't check for various types of text files
        #[doc(alias = "MAGIC_NO_CHECK_TEXT")]
        const NO_CHECK_TEXT     = self::ffi::MAGIC_NO_CHECK_TEXT;

        /// Don't get extra information on MS Composite Document Files
        #[doc(alias = "MAGIC_NO_CHECK_CDF")]
        const NO_CHECK_CDF      = self::ffi::MAGIC_NO_CHECK_CDF;

        /// Don't examine CSV files
        #[doc(alias = "MAGIC_NO_CHECK_CSV")]
        const NO_CHECK_CSV      = self::ffi::MAGIC_NO_CHECK_CSV;

        /// Don't look for known tokens inside ascii files
        #[doc(alias = "MAGIC_NO_CHECK_TOKENS")]
        const NO_CHECK_TOKENS   = self::ffi::MAGIC_NO_CHECK_TOKENS;

        /// Don't check text encodings
        #[doc(alias = "MAGIC_NO_CHECK_ENCODING")]
        const NO_CHECK_ENCODING = self::ffi::MAGIC_NO_CHECK_ENCODING;

        /// Don't examine JSON files
        #[doc(alias = "MAGIC_NO_CHECK_JSON")]
        const NO_CHECK_JSON     = self::ffi::MAGIC_NO_CHECK_JSON;

        /// No built-in tests; only consult the magic file
        #[doc(alias = "MAGIC_NO_CHECK_BUILTIN")]
        const NO_CHECK_BUILTIN  = Self::NO_CHECK_COMPRESS.bits
                                | Self::NO_CHECK_TAR.bits
                                | Self::NO_CHECK_APPTYPE.bits
                                | Self::NO_CHECK_ELF.bits
                                | Self::NO_CHECK_TEXT.bits
                                | Self::NO_CHECK_CSV.bits
                                | Self::NO_CHECK_CDF.bits
                                | Self::NO_CHECK_TOKENS.bits
                                | Self::NO_CHECK_ENCODING.bits
                                | Self::NO_CHECK_JSON.bits;
    }
}

/// Returns the version of this crate in the format `MAJOR.MINOR.PATCH`.
pub fn version() -> &'static str {
    // TODO: There's also an optional _PRE part
    concat!(
        env!("CARGO_PKG_VERSION_MAJOR"),
        ".",
        env!("CARGO_PKG_VERSION_MINOR"),
        ".",
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}

fn db_filenames<P: AsRef<Path>>(filenames: &[P]) -> *const c_char {
    match filenames.len() {
        0 => ptr::null(),
        // FIXME: This is just plain wrong. I'm surprised it works at all..
        1 => CString::new(filenames[0].as_ref().to_string_lossy().into_owned())
            .unwrap()
            .into_raw(),
        _ => unimplemented!(),
    }
}

/// Generic `libmagic` error type for successfuly opened [`Cookie`] instances
#[doc(alias = "magic_error")]
#[derive(Error, Debug)]
#[error("`libmagic` error ({}): {explanation}", match .errno {
    Some(errno) => format!("OS errno: {}", errno),
    None => "no OS errno".to_string(),
})]
pub struct LibmagicError {
    explanation: String,
    #[source]
    errno: Option<errno::Errno>,
}

/// `libmagic` error type for [`Cookie::open`]
#[derive(Error, Debug)]
#[error("`libmagic` error for `magic_open`, errno: {errno}")]
pub struct LibmagicOpenError {
    #[source]
    errno: errno::Errno,
}

/// The error type used in this crate
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum MagicError {
    #[error(transparent)]
    Libmagic(#[from] LibmagicError),
    #[error(transparent)]
    LibmagicOpen(#[from] LibmagicOpenError),
    #[error("`libmagic` flag {0:?} is not supported on this system")]
    LibmagicFlagUnsupported(CookieFlags),
    #[error("unknown error")]
    Unknown,
}

/// Configuration of which `CookieFlags` and magic databases to use
#[derive(Debug)]
#[doc(alias = "magic_t")]
#[doc(alias = "magic_set")]
pub struct Cookie {
    cookie: self::ffi::magic_t,
}

impl Drop for Cookie {
    /// Closes the magic database and deallocates any resources used
    #[doc(alias = "magic_close")]
    fn drop(&mut self) {
        unsafe { self::ffi::magic_close(self.cookie) }
    }
}

impl Cookie {
    fn last_error(&self) -> Option<MagicError> {
        let cookie = self.cookie;

        unsafe {
            let error = self::ffi::magic_error(cookie);
            let errno = self::ffi::magic_errno(cookie);
            if error.is_null() {
                None
            } else {
                let slice = CStr::from_ptr(error).to_bytes();
                Some(
                    LibmagicError {
                        explanation: str::from_utf8(slice).unwrap().to_string(),
                        errno: match errno {
                            0 => None,
                            _ => Some(errno::Errno(errno)),
                        },
                    }
                    .into(),
                )
            }
        }
    }

    fn magic_failure(&self) -> MagicError {
        match self.last_error() {
            Some(e) => e,
            None => MagicError::Unknown,
        }
    }

    /// Returns a textual description of the contents of the `filename`
    #[doc(alias = "magic_file")]
    pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, MagicError> {
        let cookie = self.cookie;
        let f = CString::new(filename.as_ref().to_string_lossy().into_owned())
            .unwrap()
            .into_raw();
        unsafe {
            let str = self::ffi::magic_file(cookie, f);
            if str.is_null() {
                Err(self.magic_failure())
            } else {
                let slice = CStr::from_ptr(str).to_bytes();
                Ok(str::from_utf8(slice).unwrap().to_string())
            }
        }
    }

    /// Returns a textual description of the contents of the `buffer`
    #[doc(alias = "magic_buffer")]
    pub fn buffer(&self, buffer: &[u8]) -> Result<String, MagicError> {
        let buffer_len = buffer.len() as size_t;
        let pbuffer = buffer.as_ptr();
        unsafe {
            let str = self::ffi::magic_buffer(self.cookie, pbuffer, buffer_len);
            if str.is_null() {
                Err(self.magic_failure())
            } else {
                let slice = CStr::from_ptr(str).to_bytes();
                Ok(str::from_utf8(slice).unwrap().to_string())
            }
        }
    }

    /// Sets the flags to use
    ///
    /// Overwrites any previously set flags, e.g. those from `load()`.
    #[doc(alias = "magic_setflags")]
    pub fn set_flags(&self, flags: self::CookieFlags) -> Result<(), MagicError> {
        let ret = unsafe { self::ffi::magic_setflags(self.cookie, flags.bits()) };
        match ret {
            -1 => Err(MagicError::LibmagicFlagUnsupported(
                CookieFlags::PRESERVE_ATIME,
            )),
            _ => Ok(()),
        }
    }

    // TODO: check, compile, list and load mostly do the same, refactor!
    // TODO: ^ also needs to implement multiple databases, possibly waiting for the Path reform

    /// Check the validity of entries in the database `filenames`
    #[doc(alias = "magic_check")]
    pub fn check<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_check(cookie, db_filenames);
        }
        if 0 == ret {
            Ok(())
        } else {
            Err(self.magic_failure())
        }
    }

    /// Compiles the given database `filenames` for faster access
    ///
    /// The compiled files created are named from the `basename` of each file argument with '.mgc' appended to it.
    #[doc(alias = "magic_compile")]
    pub fn compile<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_compile(cookie, db_filenames);
        }
        if 0 == ret {
            Ok(())
        } else {
            Err(self.magic_failure())
        }
    }

    /// Dumps all magic entries in the given database `filenames` in a human readable format
    #[doc(alias = "magic_list")]
    pub fn list<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_list(cookie, db_filenames);
        }
        if 0 == ret {
            Ok(())
        } else {
            Err(self.magic_failure())
        }
    }

    /// Loads the given database `filenames` for further queries
    ///
    /// Adds '.mgc' to the database filenames as appropriate.
    #[doc(alias = "magic_load")]
    pub fn load<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_load(cookie, db_filenames);
        }
        if 0 == ret {
            Ok(())
        } else {
            Err(self.magic_failure())
        }
    }

    /// Loads the given compiled databases for further queries
    ///
    /// This function can be used in environment where `libmagic` does
    /// not have direct access to the filesystem, but can access the magic
    /// database via shared memory or other IPC means.
    #[doc(alias = "magic_load_buffers")]
    pub fn load_buffers(&self, buffers: &[&[u8]]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let mut ffi_buffers: Vec<*const u8> = Vec::with_capacity(buffers.len());
        let mut ffi_sizes: Vec<libc::size_t> = Vec::with_capacity(buffers.len());
        let ffi_nbuffers = buffers.len() as libc::size_t;
        let ret;

        for slice in buffers {
            ffi_buffers.push((*slice).as_ptr());
            ffi_sizes.push(slice.len() as libc::size_t);
        }

        unsafe {
            ret = self::ffi::magic_load_buffers(
                cookie,
                ffi_buffers.as_mut_ptr() as *mut *mut libc::c_void,
                ffi_sizes.as_mut_ptr(),
                ffi_nbuffers,
            )
        };

        if 0 == ret {
            Ok(())
        } else {
            Err(self.magic_failure())
        }
    }

    /// Creates a new configuration, `flags` specify how other functions should behave
    ///
    /// This does not `load()` any databases yet.
    #[doc(alias = "magic_open")]
    pub fn open(flags: self::CookieFlags) -> Result<Cookie, MagicError> {
        let cookie;
        unsafe {
            cookie = self::ffi::magic_open((flags | self::CookieFlags::ERROR).bits());
        }
        if cookie.is_null() {
            Err(LibmagicOpenError {
                errno: errno::errno(),
            }
            .into())
        } else {
            Ok(Cookie { cookie })
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate regex;

    use super::Cookie;
    use super::CookieFlags;
    use super::MagicError;

    // Using relative paths to test files should be fine, since cargo doc
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#inputs-to-the-build-script
    // states that cwd == CARGO_MANIFEST_DIR

    #[test]
    fn file() {
        let cookie = Cookie::open(Default::default()).ok().unwrap();
        assert!(cookie.load(&vec!["data/tests/db-images-png"]).is_ok());

        let path = "data/tests/rust-logo-128x128-blk.png";

        assert_eq!(
            cookie.file(&path).ok().unwrap(),
            "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
        );

        cookie.set_flags(CookieFlags::MIME_TYPE).unwrap();
        assert_eq!(cookie.file(&path).ok().unwrap(), "image/png");

        cookie
            .set_flags(CookieFlags::MIME_TYPE | CookieFlags::MIME_ENCODING)
            .unwrap();
        assert_eq!(
            cookie.file(&path).ok().unwrap(),
            "image/png; charset=binary"
        );
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(Default::default()).ok().unwrap();
        assert!(cookie
            .load(&vec!["data/tests/db-python"].as_slice())
            .is_ok());

        let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
        assert_eq!(
            cookie.buffer(s).ok().unwrap(),
            "Python script, ASCII text executable"
        );

        cookie.set_flags(CookieFlags::MIME_TYPE).unwrap();
        assert_eq!(cookie.buffer(s).ok().unwrap(), "text/x-python");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(CookieFlags::ERROR).ok().unwrap();
        assert!(cookie.load::<&str>(&[]).is_ok());

        let ret = cookie.file("non-existent_file.txt");
        match ret {
            Err(e @ MagicError::Libmagic { .. }) => println!("{}", e),
            ref e => panic!("result is not a `Libmagic` error: {:?}", e),
        }
    }

    #[test]
    fn load_default() {
        let cookie = Cookie::open(CookieFlags::ERROR).ok().unwrap();
        assert!(cookie.load::<&str>(&[]).is_ok());
    }

    #[test]
    fn load_one() {
        let cookie = Cookie::open(CookieFlags::ERROR).ok().unwrap();
        assert!(cookie.load(&vec!["data/tests/db-images-png"]).is_ok());
    }

    #[test]
    // TODO: This should not really fail
    #[should_panic(expected = "not implemented")]
    fn load_multiple() {
        let cookie = Cookie::open(CookieFlags::ERROR).ok().unwrap();
        assert!(cookie
            .load(&vec!["data/tests/db-images-png", "data/tests/db-python",])
            .is_ok());
    }

    #[test]
    fn version() {
        let version_regex = regex::Regex::new(r"\d+\.\d+.\d+").unwrap();
        assert!(version_regex.is_match(super::version()));
    }

    assert_impl_all!(Cookie: std::fmt::Debug);

    #[test]
    fn load_buffers_file() {
        let cookie = Cookie::open(Default::default()).ok().unwrap();
        // file --compile --magic-file data/tests/db-images-png
        let magic_database = std::fs::read("data/tests/db-images-png-precompiled.mgc").unwrap();
        let buffers = vec![magic_database.as_slice()];
        cookie.load_buffers(&*buffers).unwrap();

        let path = "data/tests/rust-logo-128x128-blk.png";
        assert_eq!(
            cookie.file(&path).ok().unwrap(),
            "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
        );
    }
}
