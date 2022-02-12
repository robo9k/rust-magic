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

#![warn(unsafe_code)]

extern crate libc;
extern crate magic_sys as libmagic;
#[macro_use]
extern crate bitflags;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;
extern crate errno;
extern crate thiserror;

use libc::c_int;
use std::ffi::CString;
use std::path::Path;
use std::str;
use thiserror::Error;

mod ffi;

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
        const DEBUG             = self::libmagic::MAGIC_DEBUG;

        /// If the file queried is a symlink, follow it
        #[doc(alias = "MAGIC_SYMLINK")]
        const SYMLINK           = self::libmagic::MAGIC_SYMLINK;

        /// If the file is compressed, unpack it and look at the contents
        #[doc(alias = "MAGIC_COMPRESS")]
        const COMPRESS          = self::libmagic::MAGIC_COMPRESS;

        /// If the file is a block or character special device, then open the device and try to look in its contents
        #[doc(alias = "MAGIC_DEVICES")]
        const DEVICES           = self::libmagic::MAGIC_DEVICES;

        /// Return a MIME type string, instead of a textual description
        #[doc(alias = "MAGIC_MIME_TYPE")]
        const MIME_TYPE         = self::libmagic::MAGIC_MIME_TYPE;

        /// Return all matches, not just the first
        #[doc(alias = "MAGIC_CONTINUE")]
        const CONTINUE          = self::libmagic::MAGIC_CONTINUE;

        /// Check the magic database for consistency and print warnings to `stderr`
        ///
        /// NOTE: Those warnings are printed by `libmagic` itself, no this Rust crate.
        #[doc(alias = "MAGIC_CHECK")]
        const CHECK             = self::libmagic::MAGIC_CHECK;

        /// On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed
        #[doc(alias = "MAGIC_PRESERVE_ATIME")]
        const PRESERVE_ATIME    = self::libmagic::MAGIC_PRESERVE_ATIME;

        /// Don't translate unprintable characters to a `\\ooo` octal representation
        #[doc(alias = "MAGIC_RAW")]
        const RAW               = self::libmagic::MAGIC_RAW;

        /// Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer
        #[doc(alias = "MAGIC_ERROR")]
        const ERROR             = self::libmagic::MAGIC_ERROR;

        /// Return a MIME encoding, instead of a textual description
        #[doc(alias = "MAGIC_MIME_ENCODING")]
        const MIME_ENCODING     = self::libmagic::MAGIC_MIME_ENCODING;

        /// A shorthand for `MIME_TYPE | MIME_ENCODING`
        #[doc(alias = "MAGIC_MIME")]
        const MIME              = Self::MIME_TYPE.bits
                                | Self::MIME_ENCODING.bits;

        /// Return the Apple creator and type
        #[doc(alias = "MAGIC_APPLE")]
        const APPLE             = self::libmagic::MAGIC_APPLE;

        /// Return a slash-separated list of extensions for this file type
        #[doc(alias = "MAGIC_EXTENSION")]
        const EXTENSION         = self::libmagic::MAGIC_EXTENSION;

        /// Don't report on compression, only report about the uncompressed data
        #[doc(alias = "MAGIC_COMPRESS_TRANSP")]
        const COMPRESS_TRANSP   = self::libmagic::MAGIC_COMPRESS_TRANSP;

        /// A shorthand for `EXTENSION | MIME | APPLE`
        #[doc(alias = "MAGIC_NODESC")]
        const NODESC            = Self::EXTENSION.bits
                                | Self::MIME.bits
                                | Self::APPLE.bits;

        /// Don't look inside compressed files
        #[doc(alias = "MAGIC_NO_CHECK_COMPRESS")]
        const NO_CHECK_COMPRESS = self::libmagic::MAGIC_NO_CHECK_COMPRESS;

        /// Don't examine tar files
        #[doc(alias = "MAGIC_NO_CHECK_TAR")]
        const NO_CHECK_TAR      = self::libmagic::MAGIC_NO_CHECK_TAR;

        /// Don't consult magic files
        #[doc(alias = "MAGIC_NO_CHECK_SOFT")]
        const NO_CHECK_SOFT     = self::libmagic::MAGIC_NO_CHECK_SOFT;

        /// Check for EMX application type (only on EMX)
        #[doc(alias = "MAGIC_NO_CHECK_APPTYPE")]
        const NO_CHECK_APPTYPE  = self::libmagic::MAGIC_NO_CHECK_APPTYPE;

        /// Don't print ELF details
        #[doc(alias = "MAGIC_NO_CHECK_ELF")]
        const NO_CHECK_ELF      = self::libmagic::MAGIC_NO_CHECK_ELF;

        /// Don't check for various types of text files
        #[doc(alias = "MAGIC_NO_CHECK_TEXT")]
        const NO_CHECK_TEXT     = self::libmagic::MAGIC_NO_CHECK_TEXT;

        /// Don't get extra information on MS Composite Document Files
        #[doc(alias = "MAGIC_NO_CHECK_CDF")]
        const NO_CHECK_CDF      = self::libmagic::MAGIC_NO_CHECK_CDF;

        /// Don't examine CSV files
        #[doc(alias = "MAGIC_NO_CHECK_CSV")]
        const NO_CHECK_CSV      = self::libmagic::MAGIC_NO_CHECK_CSV;

        /// Don't look for known tokens inside ascii files
        #[doc(alias = "MAGIC_NO_CHECK_TOKENS")]
        const NO_CHECK_TOKENS   = self::libmagic::MAGIC_NO_CHECK_TOKENS;

        /// Don't check text encodings
        #[doc(alias = "MAGIC_NO_CHECK_ENCODING")]
        const NO_CHECK_ENCODING = self::libmagic::MAGIC_NO_CHECK_ENCODING;

        /// Don't examine JSON files
        #[doc(alias = "MAGIC_NO_CHECK_JSON")]
        const NO_CHECK_JSON     = self::libmagic::MAGIC_NO_CHECK_JSON;

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

fn db_filenames<P: AsRef<Path>>(filenames: &[P]) -> Result<Option<CString>, MagicError> {
    match filenames.len() {
        0 => Ok(None),
        // this is not the most efficient nor correct for Windows, but consistent with previous behaviour
        _ => Ok(Some(
            CString::new(
                filenames
                    .iter()
                    .map(|f| f.as_ref().to_string_lossy().into_owned())
                    .collect::<Vec<String>>()
                    .join(":"),
            )
            .map_err(|_| MagicError::InvalidDatabaseFilePath)?,
        )),
    }
}

/// FFI error while calling `libmagic`
// This is a newtype wrapper to avoid making `ffi::LibmagicError` fields public
#[derive(Error, Debug)]
#[error("`libmagic` error: {0:?}")]
pub struct FfiError(#[from] ffi::LibmagicError);

/// The error type used in this crate
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum MagicError {
    #[error(transparent)]
    Libmagic(#[from] FfiError),
    #[error("`libmagic` flag {0:?} is not supported on this system")]
    LibmagicFlagUnsupported(CookieFlags),
    #[error("invalid database file path")]
    InvalidDatabaseFilePath,
    #[error("unknown error")]
    Unknown,
}

impl From<self::ffi::LibmagicError> for MagicError {
    fn from(libmagic_error: self::ffi::LibmagicError) -> Self {
        FfiError::from(libmagic_error).into()
    }
}

/// Configuration of which `CookieFlags` and magic databases to use
#[derive(Debug)]
#[doc(alias = "magic_t")]
#[doc(alias = "magic_set")]
pub struct Cookie {
    cookie: self::libmagic::magic_t,
}

impl Drop for Cookie {
    /// Closes the magic database and deallocates any resources used
    #[doc(alias = "magic_close")]
    fn drop(&mut self) {
        self::ffi::close(self.cookie);
    }
}

impl Cookie {
    /// Returns a textual description of the contents of the `filename`
    #[doc(alias = "magic_file")]
    pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, MagicError> {
        let c_string = CString::new(filename.as_ref().to_string_lossy().into_owned()).unwrap();
        match self::ffi::file(self.cookie, c_string.as_c_str()) {
            Ok(res) => Ok(res.to_string_lossy().to_string()),
            Err(err) => Err(err.into()),
        }
    }

    /// Returns a textual description of the contents of the `buffer`
    #[doc(alias = "magic_buffer")]
    pub fn buffer(&self, buffer: &[u8]) -> Result<String, MagicError> {
        match self::ffi::buffer(self.cookie, buffer) {
            Ok(res) => Ok(res.to_string_lossy().to_string()),
            Err(err) => Err(err.into()),
        }
    }

    /// Sets the flags to use
    ///
    /// Overwrites any previously set flags, e.g. those from `load()`.
    #[doc(alias = "magic_setflags")]
    pub fn set_flags(&self, flags: self::CookieFlags) -> Result<(), MagicError> {
        let ret = self::ffi::setflags(self.cookie, flags.bits());
        match ret {
            // according to `libmagic` man page this is the only flag that could be unsupported
            Err(_) => Err(MagicError::LibmagicFlagUnsupported(
                CookieFlags::PRESERVE_ATIME,
            )),
            Ok(_) => Ok(()),
        }
    }

    // TODO: check, compile, list and load mostly do the same, refactor!
    // TODO: ^ also needs to implement multiple databases, possibly waiting for the Path reform

    /// Check the validity of entries in the database `filenames`
    #[doc(alias = "magic_check")]
    pub fn check<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let db_filenames = db_filenames(filenames)?;

        match self::ffi::check(self.cookie, db_filenames.as_deref()) {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    /// Compiles the given database `filenames` for faster access
    ///
    /// The compiled files created are named from the `basename` of each file argument with '.mgc' appended to it.
    #[doc(alias = "magic_compile")]
    pub fn compile<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let db_filenames = db_filenames(filenames)?;

        match self::ffi::compile(self.cookie, db_filenames.as_deref()) {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    /// Dumps all magic entries in the given database `filenames` in a human readable format
    #[doc(alias = "magic_list")]
    pub fn list<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let db_filenames = db_filenames(filenames)?;

        match self::ffi::list(self.cookie, db_filenames.as_deref()) {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    /// Loads the given database `filenames` for further queries
    ///
    /// Adds ".mgc" to the database filenames as appropriate.
    ///
    /// Calling `Cookie::load` or [`Cookie::load_buffers`] replaces the previously loaded database/s.
    ///
    /// # Examples
    /// ```rust
    /// # fn main() -> Result<(), magic::MagicError> {
    /// let cookie = magic::Cookie::open(Default::default())?;
    ///
    /// // Load the default database
    /// cookie.load::<&str>(&[])?;
    ///
    /// // Load databases from files
    /// cookie.load(&vec!["data/tests/db-images-png", "data/tests/db-python"])?;
    /// # Ok(())
    /// # }
    #[doc(alias = "magic_load")]
    pub fn load<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), MagicError> {
        let db_filenames = db_filenames(filenames)?;

        match self::ffi::load(self.cookie, db_filenames.as_deref()) {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    /// Loads the given compiled databases for further queries
    ///
    /// Databases need to be compiled with a compatible `libmagic` version.
    ///
    /// This function can be used in environments where `libmagic` does
    /// not have direct access to the filesystem, but can access the magic
    /// database via shared memory or other IPC means.
    ///
    /// Calling `Cookie::load_buffers` or [`Cookie::load`] replaces the previously loaded database/s.
    #[doc(alias = "magic_load_buffers")]
    pub fn load_buffers(&self, buffers: &[&[u8]]) -> Result<(), MagicError> {
        match self::ffi::load_buffers(self.cookie, buffers) {
            Err(err) => Err(err.into()),
            Ok(_) => Ok(()),
        }
    }

    /// Creates a new configuration, `flags` specify how other functions should behave
    ///
    /// This does not `load()` any databases yet.
    #[doc(alias = "magic_open")]
    pub fn open(flags: self::CookieFlags) -> Result<Cookie, MagicError> {
        let cookie;
        unsafe {
            cookie = self::libmagic::magic_open((flags | self::CookieFlags::ERROR).bits());
        }
        if cookie.is_null() {
            Err(self::ffi::LibmagicError::Open {
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
