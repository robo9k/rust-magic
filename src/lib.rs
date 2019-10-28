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

use libc::{size_t, c_char};
use std::path::Path;
use std::str;
use std::ptr;
use std::error;
use std::fmt::Display;
use std::ffi::{CStr, CString};

// Make it easier to use `CookieFlags::default()` and such
pub use self::flags::CookieFlags;


/// Bitmask flags which control `libmagic` behaviour
pub mod flags {
    use libc::c_int;

    bitflags! {
        #[doc = "Bitmask flags that specify how `Cookie` functions should behave\n\nNOTE: The descriptions are taken from `man libmagic 3`."]
        pub flags CookieFlags: c_int {
            #[doc = "No special handling"]
            const NONE              = 0x000000,

            #[doc = "Print debugging messages to `stderr`\n\nNOTE: Those messages are printed by `libmagic` itself, no this Rust crate."]
            const DEBUG             = 0x000001,

            #[doc = "If the file queried is a symlink, follow it"]
            const SYMLINK           = 0x000002,

            #[doc = "If the file is compressed, unpack it and look at the contents"]
            const COMPRESS          = 0x000004,

            #[doc = "If the file is a block or character special device, then open the device and try to look in its contents"]
            const DEVICES           = 0x000008,

            #[doc = "Return a MIME type string, instead of a textual description"]
            const MIME_TYPE         = 0x000010,

            #[doc = "Return all matches, not just the first"]
            const CONTINUE          = 0x000020,

            #[doc = "Check the magic database for consistency and print warnings to `stderr`\n\nNOTE: Those warnings are printed by `libmagic` itself, no this Rust crate."]
            const CHECK             = 0x000040,

            #[doc = "On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed"]
            const PRESERVE_ATIME    = 0x000080,

            #[doc = "Don't translate unprintable characters to a `\\ooo` octal representation"]
            const RAW               = 0x000100,

            #[doc = "Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer"]
            const ERROR             = 0x000200,

            #[doc = "Return a MIME encoding, instead of a textual description"]
            const MIME_ENCODING     = 0x000400,

            #[doc = "A shorthand for `MIME_TYPE | MIME_ENCODING`"]
            const MIME              = MIME_TYPE.bits
                                     | MIME_ENCODING.bits,

            #[doc = "Return the Apple creator and type"]
            const APPLE             = 0x000800,

            #[doc = "Don't look inside compressed files"]
            const NO_CHECK_COMPRESS = 0x001000,

            #[doc = "Don't examine tar files"]
            const NO_CHECK_TAR      = 0x002000,

            #[doc = "Don't consult magic files"]
            const NO_CHECK_SOFT     = 0x004000,

            #[doc = "Check for EMX application type (only on EMX)"]
            const NO_CHECK_APPTYPE  = 0x008000,

            #[doc = "Don't print ELF details"]
            const NO_CHECK_ELF      = 0x010000,

            #[doc = "Don't check for various types of text files"]
            const NO_CHECK_TEXT     = 0x020000,

            #[doc = "Don't get extra information on MS Composite Document Files"]
            const NO_CHECK_CDF      = 0x040000,

            #[doc = "Don't look for known tokens inside ascii files"]
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
                                     | NO_CHECK_ENCODING.bits,
	    }
    }

    impl Default for CookieFlags {
        /// Returns `NONE`
        fn default() -> CookieFlags {
            NONE
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
        1 => CString::new(filenames[0].as_ref().to_string_lossy().into_owned()).unwrap().into_raw(),
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
    cookie: *const self::ffi::Magic,
}

impl Drop for Cookie {
    /// Closes the magic database and deallocates any resources used
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

    /// Returns a textual description of the contents of the `filename`
    pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, MagicError> {
        let cookie = self.cookie;
        let f = CString::new(filename.as_ref().to_string_lossy().into_owned()).unwrap().into_raw();
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
    pub fn set_flags(&self, flags: self::flags::CookieFlags) -> bool {
        unsafe {
            self::ffi::magic_setflags(self.cookie, flags.bits()) != -1
        }
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
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
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
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Dumps all magic entries in the given database `filenames` in a human readable format
    pub fn list<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_list(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Loads the given database `filenames` for further queries
    ///
    /// Adds '.mgc'	to the database	filenames as appropriate.
    pub fn load<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = self::ffi::magic_load(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Loads one or several buffers loaded with contents of compiled magic
    /// databases.  This function can be used in environments where the magic
    /// library does not have direct access to the filesystem.
    pub fn load_buffers(&self, buffers: &[&[u8]]) -> Result<(), MagicError> {
        let cookie = self.cookie;
        let mut ffi_buffers: Vec<*const u8>    = Vec::with_capacity(buffers.len());
        let mut ffi_sizes:   Vec<libc::size_t> = Vec::with_capacity(buffers.len());
        let ffi_nbuffers = buffers.len() as libc::size_t;
        let ret;

        for slice in buffers {
            ffi_buffers.push((*slice).as_ptr());
            ffi_sizes.push(slice.len() as libc::size_t);
        }

        unsafe {
            ret = magic_sys::magic_load_buffers(
                cookie,
                ffi_buffers.as_ptr(),
                ffi_sizes.as_ptr(),
                ffi_nbuffers)
        };

        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Creates a new configuration, `flags` specify how other functions should behave
    ///
    /// This does not `load()` any databases yet.
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
