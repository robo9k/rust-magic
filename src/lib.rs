// SPDX-FileCopyrightText: © The `magic` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # About
//!
//! This crate provides bindings for the `libmagic` C library, which recognizes the
//! type of data contained in a file (or buffer).
//!
//! You might be familiar with `libmagic`'s CLI; [`file`](https://www.darwinsys.com/file/):
//!
//! ```shell
//! $ file data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//! ```
//!
//! ## `libmagic`
//!
//! Understanding how the `libmagic` C library and thus this crate works requires a bit of glossary.
//!
//! `libmagic` at its core can analyze a file or buffer and return a mostly unstructured text that describes the analysis result.
//! There are built-in tests for special cases such as symlinks and compressed files
//! and there are magic databases with signatures which can be supplied by the user for the generic cases.
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
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use std::convert::TryInto;
//! // Open a new configuration with flags
//! let cookie = magic::Cookie::open(magic::cookie::Flags::ERROR)?;
//!
//! // Load a specific database
//! // (so exact test text assertion below works regardless of the system's default database version)
//! let database = &["data/tests/db-images-png"].try_into()?;
//! // You can instead load the default database
//! //let database = &Default::default();
//!
//! let cookie = cookie.load(database)?;
//!
//! // Analyze a test file
//! let file_to_analyze = "data/tests/rust-logo-128x128-blk.png";
//! let expected_analysis_result = "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced";
//! assert_eq!(cookie.file(file_to_analyze)?, expected_analysis_result);
//! # Ok(())
//! # }
//! ```
//!
//! See further examples in [`examples/`](https://github.com/robo9k/rust-magic/tree/main/examples).
//!
//! # Further reading
//!
//! * [`Cookie::open`]
//! * cookie [`Flags`](crate::cookie::Flags), in particular:
//!     * [`Flags::ERROR`](crate::cookie::Flags::ERROR)
//!     * [`Flags::NO_CHECK_BUILTIN`](crate::cookie::Flags::NO_CHECK_BUILTIN)
//!     * [`Flags::MIME`](crate::cookie::Flags::MIME)
//!     * [`Flags::EXTENSION`](crate::cookie::Flags::EXTENSION)
//! * [`Cookie::load`], [`Cookie::load_buffers`]
//! * [`Cookie::file`], [`Cookie::buffer`]
//!
//! Note that while some `libmagic` functions return somewhat structured text, e.g. MIME types and file extensions,
//! the `magic` crate does not attempt to parse them into Rust data types since the format is not guaranteed by the C FFI API.
//!
//! Check the [crate README](https://crates.io/crates/magic) for required dependencies and MSRV.
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
//! This Rust project / crate is not affiliated with the original `file` / `libmagic` C project.
//!
//! # Use cases
//!
//! `libmagic` can help to identify unknown content. It does this by looking at byte patterns, among other things.
//! This does not guarantee that e.g. a file which is detected as a PNG image is indeed a valid PNG image.
//!
//! Maybe you just want a mapping from file name extensions to MIME types instead, e.g. ".png" ↔ "image/png"?
//! In this case you do not even need to look at file contents and could use e.g. the [`mime_guess` crate](https://crates.io/crates/mime_guess).
//!
//! Maybe you want to be certain that a file is valid for a kown format, e.g. a PNG image?
//! In this case you should use a parser for that format specifically, e.g. the [`image` crate](https://crates.io/crates/image).
//!
//! Maybe you want to know if a file contains other, malicious content?
//! In this case you should use an anti-virus software, e.g. [ClamAV](https://www.clamav.net/), [Virus Total](https://www.virustotal.com/).

#![deny(unsafe_code)]

mod ffi;

/// Returns the version of the `libmagic` C library as reported by itself.
///
/// # Examples
/// A version of "5.41" is returned as `541`.
#[doc(alias = "magic_version")]
pub fn libmagic_version() -> libc::c_int {
    crate::ffi::version()
}

// Functionality for [`Cookie`]
pub mod cookie {
    use std::convert::TryFrom;
    use std::ffi::CString;
    use std::path::Path;

    use magic_sys as libmagic;

    bitflags::bitflags! {
        /// Bitmask flags that specify how `Cookie` functions should behave
        ///
        /// NOTE: The descriptions are taken from `man libmagic 3`.
        ///
        /// `MAGIC_NONE` is the default, meaning "No special handling".
        /// ```
        /// let default_flags: magic::cookie::Flags = Default::default();
        /// assert_eq!(default_flags, magic::cookie::Flags::empty());
        /// ```
        #[derive(std::default::Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
        pub struct Flags: libc::c_int {
            // MAGIC_NONE is 0/default, see https://docs.rs/bitflags/latest/bitflags/#zero-bit-flags

            // Define unnamed flag for all other bits https://docs.rs/bitflags/latest/bitflags/#externally-defined-flags
            const _                 = !0;

            /// Print debugging messages to `stderr`
            ///
            /// NOTE: Those messages are printed by `libmagic` itself, no this Rust crate.
            #[doc(alias = "MAGIC_DEBUG")]
            const DEBUG             = libmagic::MAGIC_DEBUG;

            /// If the file queried is a symlink, follow it
            #[doc(alias = "MAGIC_SYMLINK")]
            const SYMLINK           = libmagic::MAGIC_SYMLINK;

            /// If the file is compressed, unpack it and look at the contents
            #[doc(alias = "MAGIC_COMPRESS")]
            const COMPRESS          = libmagic::MAGIC_COMPRESS;

            /// If the file is a block or character special device, then open the device and try to look in its contents
            #[doc(alias = "MAGIC_DEVICES")]
            const DEVICES           = libmagic::MAGIC_DEVICES;

            /// Return a MIME type string, instead of a textual description
            ///
            /// See also: [`Flags::MIME`]
            ///
            /// NOTE: `libmagic` uses non-standard MIME types for at least some built-in checks,
            /// e.g. `inode/*` (also see [`Flags::SYMLINK`], [`Flags::DEVICES`]):
            /// ```shell
            /// $ file --mime-type /proc/self/exe
            /// /proc/self/exe: inode/symlink
            ///
            /// $file --mime-type /dev/sda
            /// /dev/sda: inode/blockdevice
            /// ```
            #[doc(alias = "MAGIC_MIME_TYPE")]
            const MIME_TYPE         = libmagic::MAGIC_MIME_TYPE;

            /// Return all matches, not just the first
            #[doc(alias = "MAGIC_CONTINUE")]
            const CONTINUE          = libmagic::MAGIC_CONTINUE;

            /// Check the magic database for consistency and print warnings to `stderr`
            ///
            /// NOTE: Those warnings are printed by `libmagic` itself, no this Rust crate.
            #[doc(alias = "MAGIC_CHECK")]
            const CHECK             = libmagic::MAGIC_CHECK;

            /// On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed
            #[doc(alias = "MAGIC_PRESERVE_ATIME")]
            const PRESERVE_ATIME    = libmagic::MAGIC_PRESERVE_ATIME;

            /// Don't translate unprintable characters to a `\\ooo` octal representation
            #[doc(alias = "MAGIC_RAW")]
            const RAW               = libmagic::MAGIC_RAW;

            /// Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer
            #[doc(alias = "MAGIC_ERROR")]
            const ERROR             = libmagic::MAGIC_ERROR;

            /// Return a MIME encoding, instead of a textual description
            ///
            /// See also: [`Flags::MIME`]
            ///
            /// NOTE: `libmagic` uses non-standard MIME `charset` values, e.g. for binary files:
            /// ```shell
            /// $ file --mime-encoding /proc/self/exe
            /// binary
            /// ```
            #[doc(alias = "MAGIC_MIME_ENCODING")]
            const MIME_ENCODING     = libmagic::MAGIC_MIME_ENCODING;

            /// A shorthand for `MIME_TYPE | MIME_ENCODING`
            ///
            /// See also: [`Flags::MIME_TYPE`], [`Flags::MIME_ENCODING`]
            ///
            /// NOTE: `libmagic` returns a parseable MIME type with a `charset` field:
            /// ```shell
            /// $ file --mime /proc/self/exe
            /// /proc/self/exe: inode/symlink; charset=binary
            /// ```
            #[doc(alias = "MAGIC_MIME")]
            const MIME              = Self::MIME_TYPE.bits()
                                    | Self::MIME_ENCODING.bits();

            /// Return the Apple creator and type
            #[doc(alias = "MAGIC_APPLE")]
            const APPLE             = libmagic::MAGIC_APPLE;

            /// Return a slash-separated list of extensions for this file type
            ///
            /// NOTE: `libmagic` returns a list with one or more extensions without a leading "." dot:
            /// ```shell
            /// $ file --extension example.jpg
            /// example.jpg: jpeg/jpg/jpe/jfif
            ///
            /// $ file --extension /proc/self/exe
            /// /proc/self/exe: ???
            /// ```
            #[doc(alias = "MAGIC_EXTENSION")]
            const EXTENSION         = libmagic::MAGIC_EXTENSION;

            /// Don't report on compression, only report about the uncompressed data
            #[doc(alias = "MAGIC_COMPRESS_TRANSP")]
            const COMPRESS_TRANSP   = libmagic::MAGIC_COMPRESS_TRANSP;

            /// A shorthand for `EXTENSION | MIME | APPLE`
            #[doc(alias = "MAGIC_NODESC")]
            const NODESC            = Self::EXTENSION.bits()
                                    | Self::MIME.bits()
                                    | Self::APPLE.bits();

            /// Don't look inside compressed files
            #[doc(alias = "MAGIC_NO_CHECK_COMPRESS")]
            const NO_CHECK_COMPRESS = libmagic::MAGIC_NO_CHECK_COMPRESS;

            /// Don't examine tar files
            #[doc(alias = "MAGIC_NO_CHECK_TAR")]
            const NO_CHECK_TAR      = libmagic::MAGIC_NO_CHECK_TAR;

            /// Don't consult magic files
            #[doc(alias = "MAGIC_NO_CHECK_SOFT")]
            const NO_CHECK_SOFT     = libmagic::MAGIC_NO_CHECK_SOFT;

            /// Check for EMX application type (only on EMX)
            #[doc(alias = "MAGIC_NO_CHECK_APPTYPE")]
            const NO_CHECK_APPTYPE  = libmagic::MAGIC_NO_CHECK_APPTYPE;

            /// Don't print ELF details
            #[doc(alias = "MAGIC_NO_CHECK_ELF")]
            const NO_CHECK_ELF      = libmagic::MAGIC_NO_CHECK_ELF;

            /// Don't check for various types of text files
            #[doc(alias = "MAGIC_NO_CHECK_TEXT")]
            const NO_CHECK_TEXT     = libmagic::MAGIC_NO_CHECK_TEXT;

            /// Don't get extra information on MS Composite Document Files
            #[doc(alias = "MAGIC_NO_CHECK_CDF")]
            const NO_CHECK_CDF      = libmagic::MAGIC_NO_CHECK_CDF;

            /// Don't examine CSV files
            #[doc(alias = "MAGIC_NO_CHECK_CSV")]
            const NO_CHECK_CSV      = libmagic::MAGIC_NO_CHECK_CSV;

            /// Don't look for known tokens inside ascii files
            #[doc(alias = "MAGIC_NO_CHECK_TOKENS")]
            const NO_CHECK_TOKENS   = libmagic::MAGIC_NO_CHECK_TOKENS;

            /// Don't check text encodings
            #[doc(alias = "MAGIC_NO_CHECK_ENCODING")]
            const NO_CHECK_ENCODING = libmagic::MAGIC_NO_CHECK_ENCODING;

            /// Don't examine JSON files
            #[doc(alias = "MAGIC_NO_CHECK_JSON")]
            const NO_CHECK_JSON     = libmagic::MAGIC_NO_CHECK_JSON;

            /// No built-in tests; only consult the magic file
            #[doc(alias = "MAGIC_NO_CHECK_BUILTIN")]
            const NO_CHECK_BUILTIN  = Self::NO_CHECK_COMPRESS.bits()
                                    | Self::NO_CHECK_TAR.bits()
                                    | Self::NO_CHECK_APPTYPE.bits()
                                    | Self::NO_CHECK_ELF.bits()
                                    | Self::NO_CHECK_TEXT.bits()
                                    | Self::NO_CHECK_CSV.bits()
                                    | Self::NO_CHECK_CDF.bits()
                                    | Self::NO_CHECK_TOKENS.bits()
                                    | Self::NO_CHECK_ENCODING.bits()
                                    | Self::NO_CHECK_JSON.bits();
        }
    }

    impl std::fmt::Display for Flags {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            bitflags::parser::to_writer(self, f)
        }
    }

    /// Invalid [`DatabasePaths`]
    #[derive(thiserror::Error, Debug)]
    #[error("invalid database files path")]
    pub struct InvalidDatabasePathError {}

    /// Database file paths
    ///
    /// `libmagic` requires database file paths for certain operations that must:
    /// - be a valid C string
    /// - not contain ":", since that is used to separate multiple file paths
    ///
    /// The default unnamed database can be constructed with [`Default::default`](DatabasePaths::default).  
    /// Can be constructed manually with [new()](DatabasePaths::new) or by fallible conversion from an array, slice or Vec
    /// containing something convertible as [`std::path::Path`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::TryInto;
    /// # use magic::cookie::DatabasePaths;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // `: DatabasePaths` type annotation is only needed for these examples
    /// // if you pass it to Cookie::load() Rust will figure it out
    ///
    /// // construct default unnamed database
    /// let database: DatabasePaths = Default::default();
    ///
    /// // construct from multiple paths in array
    /// let array: [&'static str; 2] = [
    ///     "first-directory/first-database",
    ///     "second-directory/second-database",
    /// ];
    /// let database: DatabasePaths = array.try_into()?;
    ///
    /// // construct from multiple paths in slice
    /// let database: DatabasePaths = [
    ///     "first-directory/first-database".as_ref(),
    ///     std::path::Path::new("second-directory/second-database"),
    /// ]
    /// .try_into()?;
    ///
    /// // construct from multiple paths in Vec
    /// let database: DatabasePaths = vec![
    ///     std::ffi::OsStr::new("first-directory/first-database"),
    ///     "second-directory/second-database".as_ref(),
    /// ]
    /// .try_into()?;
    /// # Ok(())
    /// # }
    /// ```
    pub struct DatabasePaths {
        filenames: Option<CString>,
    }

    const DATABASE_FILENAME_SEPARATOR: &str = ":";

    impl DatabasePaths {
        pub fn new<I, P>(paths: I) -> Result<Self, InvalidDatabasePathError>
        where
            I: IntoIterator<Item = P>,
            P: AsRef<Path>,
        {
            // this is not the most efficient nor correct for Windows, but consistent with previous behaviour

            let filename = paths
                .into_iter()
                .map(|f| f.as_ref().to_string_lossy().into_owned())
                .collect::<Vec<String>>()
                .join(DATABASE_FILENAME_SEPARATOR);

            Ok(Self {
                filenames: match filename.is_empty() {
                    true => None,
                    _ => Some(CString::new(filename).map_err(|_| InvalidDatabasePathError {})?),
                },
            })
        }
    }

    impl Default for DatabasePaths {
        /// Returns the default unnamed database
        fn default() -> Self {
            Self { filenames: None }
        }
    }

    impl<P: AsRef<std::path::Path>, const N: usize> TryFrom<[P; N]> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        fn try_from(value: [P; N]) -> Result<Self, <Self as TryFrom<[P; N]>>::Error> {
            Self::new(value)
        }
    }

    impl<P: AsRef<std::path::Path>> TryFrom<Vec<P>> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        fn try_from(value: Vec<P>) -> Result<Self, <Self as TryFrom<Vec<P>>>::Error> {
            Self::new(value)
        }
    }

    impl<P: AsRef<std::path::Path>> TryFrom<&'_ [P]> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        fn try_from(value: &[P]) -> Result<Self, <Self as TryFrom<&[P]>>::Error> {
            Self::new(value)
        }
    }

    /// Error within several [`Cookie`] functions
    #[derive(thiserror::Error, Debug)]
    #[error("magic cookie error in `libmagic` function {}", .function)]
    pub struct Error {
        function: &'static str,
        //#[backtrace]
        source: crate::ffi::CookieError,
    }

    #[doc(hidden)]
    #[derive(Debug)]
    pub enum Open {}

    #[doc(hidden)]
    #[derive(Debug)]
    pub enum Load {}

    mod private {
        pub trait Sealed {}

        impl Sealed for super::Open {}
        impl Sealed for super::Load {}
    }

    #[doc(hidden)]
    pub trait State: private::Sealed {}

    impl State for Open {}
    impl State for Load {}

    /// Configuration of which `Flags` and magic databases to use
    #[derive(Debug)]
    #[doc(alias = "magic_t")]
    #[doc(alias = "magic_set")]
    pub struct Cookie<S: State> {
        cookie: crate::ffi::MagicHandle,
        marker: std::marker::PhantomData<S>,
    }

    impl<S: State> Drop for Cookie<S> {
        /// Closes the magic database and deallocates any resources used
        #[doc(alias = "magic_close")]
        fn drop(&mut self) {
            crate::ffi::close(&self.cookie);
        }
    }

    /// Operations that are valid in the `Open` state
    ///
    /// A new cookie created with [`Cookie::open`](Cookie::open) does not have any databases [loaded](Cookie::load).
    impl Cookie<Open> {
        /// Creates a new configuration, `flags` specify how other functions should behave
        ///
        /// This does not [`load()`](Cookie::load) any databases yet.
        #[doc(alias = "magic_open")]
        pub fn open(flags: Flags) -> Result<Cookie<Open>, OpenError> {
            match crate::ffi::open(flags.bits()) {
                Err(err) => Err(OpenError {
                    flags,
                    kind: match err.errno().kind() {
                        std::io::ErrorKind::InvalidInput => OpenErrorKind::UnsupportedFlags,
                        _ => OpenErrorKind::Errno,
                    },
                    source: err,
                }),
                Ok(cookie) => {
                    let cookie = Cookie {
                        cookie,
                        marker: std::marker::PhantomData,
                    };
                    Ok(cookie)
                }
            }
        }
    }

    /// Operations that are valid in the `Load` state
    ///
    /// An opened cookie with [loaded](Cookie::load) databases can inspect [files](Cookie::file) and [buffers](Cookie::buffer).
    impl Cookie<Load> {
        /// Returns a textual description of the contents of the `filename`
        ///
        /// Requires [loaded](Cookie::load) databases.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error.
        #[doc(alias = "magic_file")]
        pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, Error> {
            let c_string = CString::new(filename.as_ref().to_string_lossy().into_owned()).unwrap();
            match crate::ffi::file(&self.cookie, c_string.as_c_str()) {
                Ok(res) => Ok(res.to_string_lossy().to_string()),
                Err(err) => Err(Error {
                    function: "magic_file",
                    source: err,
                }),
            }
        }

        /// Returns a textual description of the contents of the `buffer`
        ///
        /// Requires [loaded](Cookie::load) databases.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error.
        #[doc(alias = "magic_buffer")]
        pub fn buffer(&self, buffer: &[u8]) -> Result<String, Error> {
            match crate::ffi::buffer(&self.cookie, buffer) {
                Ok(res) => Ok(res.to_string_lossy().to_string()),
                Err(err) => Err(Error {
                    function: "magic_buffer",
                    source: err,
                }),
            }
        }
    }

    /// Operations that are valid in any state
    impl<S: State> Cookie<S> {
        /// Loads the given database `filenames` for further queries
        ///
        /// Adds ".mgc" to the database filenames as appropriate.
        ///
        /// Calling `Cookie::load` or [`Cookie::load_buffers`] replaces the previously loaded database/s.
        ///
        /// # Examples
        /// ```rust
        /// # use std::convert::TryInto;
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// let cookie = magic::Cookie::open(Default::default())?;
        ///
        /// // Load the default unnamed database
        /// let database = &Default::default();
        /// let cookie = cookie.load(database)?;
        ///
        /// // Load databases from files
        /// let database = &["data/tests/db-images-png", "data/tests/db-python"].try_into()?;
        /// let cookie = cookie.load(database)?;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_load")]
        pub fn load(self, filenames: &DatabasePaths) -> Result<Cookie<Load>, Error> {
            match crate::ffi::load(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_load",
                    source: err,
                }),
                Ok(_) => {
                    let cookie = Cookie {
                        cookie: crate::ffi::MagicHandle(self.cookie.0),
                        marker: std::marker::PhantomData,
                    };
                    std::mem::forget(self);
                    Ok(cookie)
                }
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
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_load_buffers")]
        pub fn load_buffers(self, buffers: &[&[u8]]) -> Result<Cookie<Load>, Error> {
            match crate::ffi::load_buffers(&self.cookie, buffers) {
                Err(err) => Err(Error {
                    function: "magic_load_buffers",
                    source: err,
                }),
                Ok(_) => {
                    let cookie = Cookie {
                        cookie: crate::ffi::MagicHandle(self.cookie.0),
                        marker: std::marker::PhantomData,
                    };
                    std::mem::forget(self);
                    Ok(cookie)
                }
            }
        }

        /// Sets the flags to use
        ///
        /// Overwrites any previously set flags, e.g. those from [`load()`](Cookie::load).
        #[doc(alias = "magic_setflags")]
        pub fn set_flags(&self, flags: Flags) -> Result<(), SetFlagsError> {
            let ret = crate::ffi::setflags(&self.cookie, flags.bits());
            match ret {
                // according to `libmagic` man page this is the only flag that could be unsupported
                Err(err) => Err(SetFlagsError {
                    flags: Flags::PRESERVE_ATIME,
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        // TODO: check, compile, list and load mostly do the same, refactor!

        /// Compiles the given database `filenames` for faster access
        ///
        /// The compiled files created are named from the `basename` of each file argument with '.mgc' appended to it.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_compile")]
        pub fn compile(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::ffi::compile(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_compile",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        /// Check the validity of entries in the database `filenames`
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_check")]
        pub fn check(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::ffi::check(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_check",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        /// Dumps all magic entries in the given database `filenames` in a human readable format
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_list")]
        pub fn list(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::ffi::list(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_list",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }
    }

    /// Error within [`Cookie::open`](Cookie::open)
    #[derive(thiserror::Error, Debug)]
    #[error("could not open magic cookie: {}",
match .kind {
    OpenErrorKind::UnsupportedFlags => format!("unsupported flags {}", .flags),
    OpenErrorKind::Errno => "other error".to_string(),
}
)]
    pub struct OpenError {
        flags: Flags,
        kind: OpenErrorKind,
        //#[backtrace]
        source: crate::ffi::OpenError,
    }

    /// Kind of [`OpenError`]
    #[derive(Debug)]
    enum OpenErrorKind {
        /// Unsupported flags given
        UnsupportedFlags,
        /// Other kind
        Errno,
    }

    /// Error within [`Cookie::set_flags`](Cookie::set_flags)
    #[derive(thiserror::Error, Debug)]
    #[error("could not set magic cookie flags {}", .flags)]
    pub struct SetFlagsError {
        flags: Flags,
        //#[backtrace]
        source: crate::ffi::SetFlagsError,
    }
} // mod cookie

pub use crate::cookie::Cookie;

#[cfg(test)]
mod tests {
    use super::cookie::Flags;
    use super::Cookie;
    use std::convert::TryInto;

    // Using relative paths to test files should be fine, since cargo doc
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#inputs-to-the-build-script
    // states that cwd == CARGO_MANIFEST_DIR

    #[test]
    fn file() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        let databases = &["data/tests/db-images-png"].try_into().unwrap();
        let cookie = cookie.load(databases).unwrap();

        let path = "data/tests/rust-logo-128x128-blk.png";

        assert_eq!(
            cookie.file(path).ok().unwrap(),
            "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
        );

        cookie.set_flags(Flags::MIME_TYPE).unwrap();
        assert_eq!(cookie.file(path).ok().unwrap(), "image/png");

        cookie
            .set_flags(Flags::MIME_TYPE | Flags::MIME_ENCODING)
            .unwrap();
        assert_eq!(cookie.file(path).ok().unwrap(), "image/png; charset=binary");
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        let databases = &["data/tests/db-python"].try_into().unwrap();
        let cookie = cookie.load(databases).unwrap();

        let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
        assert_eq!(
            cookie.buffer(s).ok().unwrap(),
            "Python script, ASCII text executable"
        );

        cookie.set_flags(Flags::MIME_TYPE).unwrap();
        assert_eq!(cookie.buffer(s).ok().unwrap(), "text/x-python");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        let cookie = cookie.load(&Default::default()).unwrap();

        let ret = cookie.file("non-existent_file.txt");
        assert!(ret.is_err());
    }

    #[test]
    fn load_default() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        assert!(cookie.load(&Default::default()).is_ok());
    }

    #[test]
    fn load_one() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        let databases = &["data/tests/db-images-png"].try_into().unwrap();
        assert!(cookie.load(databases).is_ok());
    }

    #[test]
    fn load_multiple() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        let databases = &["data/tests/db-images-png", "data/tests/db-python"]
            .try_into()
            .unwrap();
        assert!(cookie.load(databases).is_ok());
    }

    // TODO:
    //static_assertions::assert_impl_all!(Cookie<S>: std::fmt::Debug);

    #[test]
    fn load_buffers_file() {
        let cookie = Cookie::open(Flags::ERROR).unwrap();
        // file --compile --magic-file data/tests/db-images-png
        let magic_database = std::fs::read("data/tests/db-images-png-precompiled.mgc").unwrap();
        let buffers = vec![magic_database.as_slice()];
        let cookie = cookie.load_buffers(&buffers).unwrap();

        let path = "data/tests/rust-logo-128x128-blk.png";
        assert_eq!(
            cookie.file(path).ok().unwrap(),
            "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced"
        );
    }

    #[test]
    fn libmagic_version() {
        let version = super::libmagic_version();

        assert!(version > 500);
    }
}

#[cfg(doctest)]
#[doc=include_str!("../README-crate.md")]
mod readme {}
