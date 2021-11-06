//! # About
//!
//! This crate provides bindings for `libmagic`, which recognizes the
//! type of data contained in a file (or buffer).
//!
//! You might be similar with `libmagic`'s CLI `file`:
//!
//! ```sh
//! $ file data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//! ```
//!
//! # Usage example
//!
//! Here's an example of using this crate:
//!
//! ```
//! extern crate magic;
//! use magic::{Cookie, CookieFlags};
//!
//! fn main() {
//!     // Create a new default configuration
//!     let cookie = Cookie::open(CookieFlags::default()).unwrap();
//!     // Load one specific magic database
//!     let databases = vec!["data/tests/db-images-png"];
//!     assert!(cookie.load(&databases).is_ok());
//!
//!     // Recognize the magic of a test file
//!     let test_file_path = "data/tests/rust-logo-128x128-blk.png";
//!     let expected_magic = "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced";
//!     assert_eq!(cookie.file(&test_file_path).unwrap(), expected_magic);
//! }
//! ```

extern crate libc;
extern crate magic_sys as ffi;
#[macro_use]
extern crate bitflags;

use libc::{c_char, c_int, size_t};
use std::error;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::path::Path;
use std::ptr;
use std::str;

bitflags! {
    /// Bitmask flags that specify how `Cookie` functions should behave
    ///
    /// NOTE: The descriptions are taken from `man libmagic 3`.
    pub struct CookieFlags: c_int {
        /// No special handling
        const NONE              = self::ffi::MAGIC_NONE;

        /// Print debugging messages to `stderr`
        ///
        /// NOTE: Those messages are printed by `libmagic` itself, no this Rust crate.
        const DEBUG             = self::ffi::MAGIC_DEBUG;

        /// If the file queried is a symlink, follow it
        const SYMLINK           = self::ffi::MAGIC_SYMLINK;

        /// If the file is compressed, unpack it and look at the contents
        const COMPRESS          = self::ffi::MAGIC_COMPRESS;

        /// If the file is a block or character special device, then open the device and try to look in its contents
        const DEVICES           = self::ffi::MAGIC_DEVICES;

        /// Return a MIME type string, instead of a textual description
        const MIME_TYPE         = self::ffi::MAGIC_MIME_TYPE;

        /// Return all matches, not just the first
        const CONTINUE          = self::ffi::MAGIC_CONTINUE;

        /// Check the magic database for consistency and print warnings to `stderr`
        ///
        /// NOTE: Those warnings are printed by `libmagic` itself, no this Rust crate.
        const CHECK             = self::ffi::MAGIC_CHECK;

        /// On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed
        const PRESERVE_ATIME    = self::ffi::MAGIC_PRESERVE_ATIME;

        /// Don't translate unprintable characters to a `\\ooo` octal representation
        const RAW               = self::ffi::MAGIC_RAW;

        /// Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer
        const ERROR             = self::ffi::MAGIC_ERROR;

        /// Return a MIME encoding, instead of a textual description
        const MIME_ENCODING     = self::ffi::MAGIC_MIME_ENCODING;

        /// A shorthand for `MIME_TYPE | MIME_ENCODING`
        const MIME              = Self::MIME_TYPE.bits
                                 | Self::MIME_ENCODING.bits;

        /// Return the Apple creator and type
        const APPLE             = self::ffi::MAGIC_APPLE;

        /// Don't look inside compressed files
        const NO_CHECK_COMPRESS = self::ffi::MAGIC_NO_CHECK_COMPRESS;

        /// Don't examine tar files
        const NO_CHECK_TAR      = self::ffi::MAGIC_NO_CHECK_TAR;

        /// Don't consult magic files
        const NO_CHECK_SOFT     = self::ffi::MAGIC_NO_CHECK_SOFT;

        /// Check for EMX application type (only on EMX)
        const NO_CHECK_APPTYPE  = self::ffi::MAGIC_NO_CHECK_APPTYPE;

        /// Don't print ELF details
        const NO_CHECK_ELF      = self::ffi::MAGIC_NO_CHECK_ELF;

        /// Don't check for various types of text files
        const NO_CHECK_TEXT     = self::ffi::MAGIC_NO_CHECK_TEXT;

        /// Don't get extra information on MS Composite Document Files
        const NO_CHECK_CDF      = self::ffi::MAGIC_NO_CHECK_CDF;

        /// Don't look for known tokens inside ascii files
        const NO_CHECK_TOKENS   = self::ffi::MAGIC_NO_CHECK_TOKENS;

        /// Don't check text encodings
        const NO_CHECK_ENCODING = self::ffi::MAGIC_NO_CHECK_ENCODING;

        /// No built-in tests; only consult the magic file
        const NO_CHECK_BUILTIN  = Self::NO_CHECK_COMPRESS.bits
                                 | Self::NO_CHECK_TAR.bits
                                 | Self::NO_CHECK_APPTYPE.bits
                                 | Self::NO_CHECK_ELF.bits
                                 | Self::NO_CHECK_TEXT.bits
                                 | Self::NO_CHECK_CDF.bits
                                 | Self::NO_CHECK_TOKENS.bits
                                 | Self::NO_CHECK_ENCODING.bits;
    }
}

impl Default for CookieFlags {
    /// Returns `NONE`
    fn default() -> CookieFlags {
        CookieFlags::NONE
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

/// The error type used in this crate
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MagicError {
    pub desc: String,
}

impl error::Error for MagicError {
    fn description(&self) -> &str {
        "internal libmagic error"
    }
}

impl Display for MagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

/// Configuration of which `CookieFlags` and magic databases to use
pub struct Cookie {
    cookie: self::ffi::magic_t,
}

impl Drop for Cookie {
    /// Closes the magic database and deallocates any resources used
    fn drop(&mut self) {
        unsafe { self::ffi::magic_close(self.cookie) }
    }
}

impl Cookie {
    fn last_error(&self) -> Option<MagicError> {
        let cookie = self.cookie;

        unsafe {
            let e = self::ffi::magic_error(cookie);
            if e.is_null() {
                None
            } else {
                let slice = CStr::from_ptr(e).to_bytes();
                Some(self::MagicError {
                    desc: str::from_utf8(slice).unwrap().to_string(),
                })
            }
        }
    }

    fn magic_failure(&self) -> MagicError {
        match self.last_error() {
            Some(e) => e,
            None => self::MagicError {
                desc: "unknown error".to_string(),
            },
        }
    }

    /// Returns a textual description of the contents of the `filename`
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

    /// Returns a textual explanation of the last error, if any
    ///
    /// You should not need to call this, since you can use the `MagicError` in
    /// the `Result` returned by the other functions.
    // TODO: Remove this entirely?
    pub fn error(&self) -> Option<String> {
        unsafe {
            let str = self::ffi::magic_error(self.cookie);
            if str.is_null() {
                None
            } else {
                let slice = CStr::from_ptr(str).to_bytes();
                Some(str::from_utf8(slice).unwrap().to_string())
            }
        }
    }

    /// Sets the flags to use
    ///
    /// Overwrites any previously set flags, e.g. those from `load()`.
    // TODO: libmagic itself has to magic_getflags, but we could remember them in Cookie?
    pub fn set_flags(&self, flags: self::CookieFlags) -> bool {
        unsafe { self::ffi::magic_setflags(self.cookie, flags.bits()) != -1 }
    }

    // TODO: check, compile, list and load mostly do the same, refactor!
    // TODO: ^ also needs to implement multiple databases, possibly waiting for the Path reform

    /// Check the validity of entries in the database `filenames`
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

    /// Creates a new configuration, `flags` specify how other functions should behave
    ///
    /// This does not `load()` any databases yet.
    pub fn open(flags: self::CookieFlags) -> Result<Cookie, MagicError> {
        let cookie;
        unsafe {
            cookie = self::ffi::magic_open((flags | self::CookieFlags::ERROR).bits());
        }
        if cookie.is_null() {
            Err(self::MagicError {
                desc: "errno".to_string(),
            })
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

    // Using relative paths to test files should be fine, since cargo doc
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#inputs-to-the-build-script
    // states that cwd == CARGO_MANIFEST_DIR

    #[test]
    fn file() {
        let cookie = Cookie::open(CookieFlags::NONE).ok().unwrap();
        assert!(cookie.load(&vec!["data/tests/db-images-png"]).is_ok());

        let path = "data/tests/rust-logo-128x128-blk.png";

        assert_eq!(
            cookie.file(&path).ok().unwrap(),
            "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
        );

        cookie.set_flags(CookieFlags::MIME_TYPE);
        assert_eq!(cookie.file(&path).ok().unwrap(), "image/png");

        cookie.set_flags(CookieFlags::MIME_TYPE | CookieFlags::MIME_ENCODING);
        assert_eq!(
            cookie.file(&path).ok().unwrap(),
            "image/png; charset=binary"
        );
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(CookieFlags::NONE).ok().unwrap();
        assert!(cookie
            .load(&vec!["data/tests/db-python"].as_slice())
            .is_ok());

        let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
        assert_eq!(
            cookie.buffer(s).ok().unwrap(),
            "Python script, ASCII text executable"
        );

        cookie.set_flags(CookieFlags::MIME_TYPE);
        assert_eq!(cookie.buffer(s).ok().unwrap(), "text/x-python");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(CookieFlags::NONE | CookieFlags::ERROR)
            .ok()
            .unwrap();
        assert!(cookie.load::<&str>(&[]).is_ok());

        let ret = cookie.file("non-existent_file.txt");
        assert!(ret.is_err());
        assert_eq!(
            ret.err().unwrap().desc,
            "cannot stat `non-existent_file.txt' (No such file or directory)"
        );
    }

    #[test]
    fn load_default() {
        let cookie = Cookie::open(CookieFlags::NONE | CookieFlags::ERROR)
            .ok()
            .unwrap();
        assert!(cookie.load::<&str>(&[]).is_ok());
    }

    #[test]
    fn load_one() {
        let cookie = Cookie::open(CookieFlags::NONE | CookieFlags::ERROR)
            .ok()
            .unwrap();
        assert!(cookie.load(&vec!["data/tests/db-images-png"]).is_ok());
    }

    #[test]
    // TODO: This should not really fail
    #[should_panic(expected = "not implemented")]
    fn load_multiple() {
        let cookie = Cookie::open(CookieFlags::NONE | CookieFlags::ERROR)
            .ok()
            .unwrap();
        assert!(cookie
            .load(&vec!["data/tests/db-images-png", "data/tests/db-python",])
            .is_ok());
    }

    #[test]
    fn version() {
        let version_regex = regex::Regex::new(r"\d+\.\d+.\d+").unwrap();
        assert!(version_regex.is_match(super::version()));
    }
}
