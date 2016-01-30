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
//! use magic::{Cookie, flags};
//!
//! fn main() {
//!     // Create a new configuration with no special flags
//!     let cookie = Cookie::open(flags::NONE).ok().unwrap();
//!     // Load a specific magic database
//!     let magic_db = vec!["data/tests/db-images-png"];
//!     assert!(cookie.load(&magic_db).is_ok());
//!
//!     // Recognize the magic of a test file
//!     let test_file = "data/tests/rust-logo-128x128-blk.png";
//!     let expected = "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced";
//!     assert_eq!(cookie.file(&test_file).ok().unwrap(), expected);
//! }
//! ```


// OsStr.to_cstring()
#![feature(convert)]

extern crate libc;
extern crate magic_sys as ffi;
#[macro_use]
extern crate bitflags;

use libc::{size_t, c_char};
use std::path::Path;
use std::str;
use std::ptr;
use std::error;
use std::fmt::Display;
use std::ffi::CStr;


/// Bitmask flags which control `libmagic` behaviour
pub mod flags {
    use libc::c_int;

    bitflags! {
        #[doc = "Bitmask flags that specify how `Cookie` functions should behave"]
        flags CookieFlags: c_int {
            #[doc = "No flags"]
            const NONE              = 0x000000,

            #[doc = "Turn on debugging"]
            const DEBUG             = 0x000001,

            #[doc = "Follow symlinks"]
            const SYMLINK           = 0x000002,

            #[doc = "Check inside compressed files"]
            const COMPRESS          = 0x000004,

            #[doc = "Look at the contents of devices"]
            const DEVICES           = 0x000008,

            #[doc = "Return the MIME type"]
            const MIME_TYPE         = 0x000010,

            #[doc = "Return all matches"]
            const CONTINUE          = 0x000020,

            #[doc = "Print warnings to stderr"]
            const CHECK             = 0x000040,

            #[doc = "Restore access time on exit"]
            const PRESERVE_ATIME    = 0x000080,

            #[doc = "Don't translate unprintable chars"]
            const RAW               = 0x000100,

            #[doc = "Handle `ENOENT` etc as real errors"]
            const ERROR             = 0x000200,

            #[doc = "Return the MIME encoding"]
            const MIME_ENCODING     = 0x000400,

            #[doc = "Return the MIME type and encoding"]
            const MIME              = MIME_TYPE.bits
                                     | MIME_ENCODING.bits,

            #[doc = "Return the Apple creator and type"]
            const APPLE             = 0x000800,

            #[doc = "Don't check for compressed files"]
            const NO_CHECK_COMPRESS = 0x001000,

            #[doc = "Don't check for tar files"]
            const NO_CHECK_TAR      = 0x002000,

            #[doc = "Don't check magic entries"]
            const NO_CHECK_SOFT     = 0x004000,

            #[doc = "Don't check application type"]
            const NO_CHECK_APPTYPE  = 0x008000,

            #[doc = "Don't check for elf details"]
            const NO_CHECK_ELF      = 0x010000,

            #[doc = "Don't check for text files"]
            const NO_CHECK_TEXT     = 0x020000,

            #[doc = "Don't check for cdf files"]
            const NO_CHECK_CDF      = 0x040000,

            #[doc = "Don't check tokens"]
            const NO_CHECK_TOKENS   = 0x100000,

            #[doc = "Don't check text encodings"]
            const NO_CHECK_ENCODING = 0x200000,

            #[doc = "No built-in tests; only consult the magic file"]
            const NO_CHECK_BUILTIN  = NO_CHECK_COMPRESS.bits
                                     | NO_CHECK_TAR.bits
                                     | NO_CHECK_APPTYPE.bits
                                     | NO_CHECK_ELF.bits
                                     | NO_CHECK_TEXT.bits
                                     | NO_CHECK_CDF.bits
                                     | NO_CHECK_TOKENS.bits
                                     | NO_CHECK_ENCODING.bits
                                     | 0,
	    }
    }
}


/// Returns the version of this crate in the format `MAJOR.MINOR.PATCH`.
pub fn version() -> &'static str {
    // TODO: There's also an optional _PRE part
    concat!(
        env!("CARGO_PKG_VERSION_MAJOR"), ".",
        env!("CARGO_PKG_VERSION_MINOR"), ".",
        env!("CARGO_PKG_VERSION_PATCH"),
    )
}


fn db_filenames<P: AsRef<Path>>(filenames: &[P]) -> *const c_char {
    match filenames.len() {
        0 => ptr::null(),
        // FIXME: This is just plain wrong. I'm surprised it works at all..
        1 => filenames[0].as_ref().as_os_str().to_cstring().unwrap().as_ptr(),
        _ => unimplemented!(),
    }
}


/// Represents a magic error.
/// For the most part you should be using the `Error` trait
/// to interact with rather than this struct.
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


pub struct Cookie {
    cookie: *const self::ffi::Magic,
}

impl Drop for Cookie {
    fn drop(&mut self) { unsafe { self::ffi::magic_close(self.cookie) } }
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
                Some(self::MagicError{desc: str::from_utf8(slice).unwrap().to_string(),})
            }
        }
    }

    fn magic_failure(&self) -> MagicError {
        match self.last_error() {
            Some(e) => e,
            None => self::MagicError{desc: "unknown error".to_string(),}
        }
    }

    pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, MagicError> {
        let cookie = self.cookie;
        let f = filename.as_ref().as_os_str().to_cstring().unwrap().as_ptr();
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

    pub fn set_flags(&self, flags: self::flags::CookieFlags) -> bool {
        unsafe {
            self::ffi::magic_setflags(self.cookie, flags.bits()) != -1
        }
    }

    // TODO: check, compile, list and load mostly do the same, refactor!
    // TODO: ^ also needs to implement multiple databases, possibly waiting for the Path reform

    pub fn check<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_check(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    pub fn compile<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_compile(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    pub fn list<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_list(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    pub fn load<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_load(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    pub fn open(flags: self::flags::CookieFlags) -> Result<Cookie, MagicError> {
        let cookie;
        unsafe {
            cookie = self::ffi::magic_open((flags | self::flags::ERROR).bits());
        }
        if cookie.is_null() { Err(self::MagicError{desc: "errno".to_string(),}) } else { Ok(Cookie {cookie: cookie,}) }
    }
}

#[cfg(test)]
mod tests {
    extern crate regex;

    use super::Cookie;
	use super::flags;

    // Using relative paths to test files should be fine, since cargo doc
    // http://doc.crates.io/build-script.html#inputs-to-the-build-script
    // states that cwd == CARGO_MANIFEST_DIR

    #[test]
    fn file() {
        let cookie = Cookie::open(flags::NONE).ok().unwrap();
        assert!(cookie.load(&vec!["data/tests/db-images-png"]).is_ok());

        let path = "data/tests/rust-logo-128x128-blk.png";

        assert_eq!(cookie.file(&path).ok().unwrap(), "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

        cookie.set_flags(flags::MIME_TYPE);
        assert_eq!(cookie.file(&path).ok().unwrap(), "image/png");

        cookie.set_flags(flags::MIME_TYPE | flags::MIME_ENCODING);
        assert_eq!(cookie.file(&path).ok().unwrap(), "image/png; charset=binary");
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(flags::NONE).ok().unwrap();
        assert!(cookie.load(&vec!["data/tests/db-python"].as_slice()).is_ok());

        let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
        assert_eq!(cookie.buffer(s).ok().unwrap(), "Python script, ASCII text executable");

        cookie.set_flags(flags::MIME_TYPE);
        assert_eq!(cookie.buffer(s).ok().unwrap(), "text/x-python");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(flags::NONE | flags::ERROR).ok().unwrap();
        assert!(cookie.load::<&str>(&[]).is_ok());

        let ret = cookie.file("non-existent_file.txt");
        assert!(ret.is_err());
        assert_eq!(ret.err().unwrap().desc, "cannot stat `non-existent_file.txt' (No such file or directory)");
    }

    #[test]
    fn load_default() {
        let cookie = Cookie::open(flags::NONE | flags::ERROR).ok().unwrap();
        assert!(cookie.load::<&str>(&[]).is_ok());
    }

    #[test]
    fn load_one() {
        let cookie = Cookie::open(flags::NONE | flags::ERROR).ok().unwrap();
        assert!(cookie.load(&vec![
                                    "data/tests/db-images-png"
                                ]).is_ok());
    }

    #[test]
    // TODO: This should not really fail
    #[should_panic(expected = "not yet implemented")]
    fn load_multiple() {
        let cookie = Cookie::open(flags::NONE | flags::ERROR).ok().unwrap();
        assert!(cookie.load(&vec![
                                "data/tests/db-images-png",
                                "data/tests/db-python",
                            ]).is_ok());
    }

    #[test]
    fn version() {
        assert!(regex::is_match(r"\d+\.\d+.\d+", super::version()).ok().unwrap());
    }
}
