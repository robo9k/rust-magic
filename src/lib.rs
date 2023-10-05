// SPDX-FileCopyrightText: © The `magic` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # About
//!
//! This crate provides bindings for the `libmagic` C library, which recognizes the
//! type of data contained in a file (or buffer).
//!
//! You might be familiar with `libmagic`'s command-line-interface; [`file`](https://www.darwinsys.com/file/):
//!
//! ```shell
//! $ file data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//! ```
//!
//! ## `libmagic`
//!
//! Understanding how the `libmagic` C library and thus this Rust crate works requires a bit of glossary.
//!
//! `libmagic` at its core can analyze a file or buffer and return a mostly unstructured text that describes the analysis result.
//! There are built-in tests for special cases such as symlinks and compressed files
//! and there are magic databases with signatures which can be supplied by the user for the generic cases
//! ("if those bytes look like this, it's a PNG image file").
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
//! // open a new configuration with flags
//! let cookie = magic::Cookie::open(magic::cookie::Flags::ERROR)?;
//!
//! // load a specific database
//! // (so exact test text assertion below works regardless of the system's default database version)
//! let database = ["data/tests/db-images-png"].try_into()?;
//! // you can instead load the default database
//! //let database = Default::default();
//!
//! let cookie = cookie.load(&database)?;
//!
//! // analyze a test file
//! let file_to_analyze = "data/tests/rust-logo-128x128-blk.png";
//! let expected_analysis_result = "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced";
//! assert_eq!(cookie.file(file_to_analyze)?, expected_analysis_result);
//! # Ok(())
//! # }
//! ```
//!
//! See further examples in [`examples/`](https://github.com/robo9k/rust-magic/tree/main/examples).
//!
//! # MIME example
//!
//! Return a MIME type with "charset" encoding parameter:
//!
//! ```shell
//! $ file --mime data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: image/png; charset=binary
//! ```
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use std::convert::TryInto;
//! // open a new configuration with flags for mime type and encoding
//! let flags = magic::cookie::Flags::MIME_TYPE | magic::cookie::Flags::MIME_ENCODING;
//! let cookie = magic::Cookie::open(flags)?;
//!
//! // load a specific database
//! let database = ["data/tests/db-images-png"].try_into()?;
//! let cookie = cookie.load(&database)?;
//!
//! // analyze a test file
//! let file_to_analyze = "data/tests/rust-logo-128x128-blk.png";
//! let expected_analysis_result = "image/png; charset=binary";
//! assert_eq!(cookie.file(file_to_analyze)?, expected_analysis_result);
//! # Ok(())
//! # }
//! ```
//!
//! See [`magic::cookie::Flags::MIME`](crate::cookie::Flags::MIME).
//!
//! # Filename extensions example
//!
//! Return slash-separated filename extensions (the ".png" in "example.png")
//! from file contents (the input filename is not used for detection):
//!
//! ```shell
//! $ file --extension data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: png
//! ```
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use std::convert::TryInto;
//! // open a new configuration with flags for filename extension
//! let flags = magic::cookie::Flags::EXTENSION;
//! let cookie = magic::Cookie::open(flags)?;
//!
//! // load a specific database
//! let database = ["data/tests/db-images-png"].try_into()?;
//! let cookie = cookie.load(&database)?;
//!
//! // analyze a test file
//! let file_to_analyze = "data/tests/rust-logo-128x128-blk.png";
//! let expected_analysis_result = "png";
//! assert_eq!(cookie.file(file_to_analyze)?, expected_analysis_result);
//! # Ok(())
//! # }
//! ```
//!
//! See [`magic::cookie::Flags::EXTENSION`](crate::cookie::Flags::EXTENSION).
//!
//! # Further reading
//!
//! * [`Cookie::open()`][Cookie::open]
//! * cookie [`Flags`](crate::cookie::Flags), in particular:
//!     * [`Flags::ERROR`](crate::cookie::Flags::ERROR)
//!     * [`Flags::MIME`](crate::cookie::Flags::MIME)
//!     * [`Flags::EXTENSION`](crate::cookie::Flags::EXTENSION)
//!     * [`Flags::CONTINUE`](crate::cookie::Flags::CONTINUE)
//!     * [`Flags::NO_CHECK_BUILTIN`](crate::cookie::Flags::NO_CHECK_BUILTIN)
//! * [`Cookie::load()`](Cookie::load), [`Cookie::load_buffers()`](Cookie::load_buffers)
//! * [`Cookie::file()`](Cookie::file), [`Cookie::buffer()`](Cookie::buffer)
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

/// Functionality for [`Cookie`]
pub mod cookie {
    use std::convert::TryFrom;
    use std::ffi::CString;
    use std::path::Path;

    use magic_sys as libmagic;

    bitflags::bitflags! {
        /// Configuration bits for [`Cookie`]
        ///
        /// A bitflags instance is a combined set of individual flags.
        /// `cookie::Flags` are configuration bits for `Cookie` instances that specify how the cookie should behave.
        ///
        /// `cookie::Flags` influence several functions, e.g. [`Cookie::file()`](Cookie::file)
        /// but also [`Cookie::load()`](Cookie::load).
        ///
        /// Flags are initially set when a new cookie is created with [`Cookie::open()`](Cookie::open)
        /// and can be overwritten lateron with [`Cookie::set_flags()`](Cookie::set_flags).
        ///
        /// Flags of particular interest:
        /// - [`ERROR`](Flags::ERROR)
        /// - [`MIME`](Flags::MIME)
        /// - [`EXTENSION`](Flags::EXTENSION)
        /// - [`CONTINUE`](Flags::CONTINUE)
        /// - [`NO_CHECK_BUILTIN`](Flags::NO_CHECK_BUILTIN)
        ///
        /// # Examples
        ///
        /// ```
        /// // default flags
        /// // `: Flags` type annotation is only needed for this example
        /// // if you pass it to Cookie::open() etc., Rust will figure it out
        /// let flags: magic::cookie::Flags = Default::default();
        ///
        /// // custom flags combination
        /// let flags = magic::cookie::Flags::COMPRESS | magic::cookie::Flags::DEVICES;
        /// ```
        ///
        /// # Errors
        ///
        /// Some flags might not be supported on all platforms, i.e.
        /// - [`Cookie::open()`](Cookie::open) might return a [`cookie::OpenError`](OpenError)
        /// - [`Cookie::set_flags()`](Cookie::set_flags) might return a [`cookie::SetFlagsError`](SetFlagsError)
        ///
        /// # Misc
        ///
        /// NOTE: The flag descriptions are mostly copied from `man libmagic 3`.
        ///
        /// `MAGIC_NONE` is the default, meaning "No special handling" and has no constant.
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
            /// This is equivalent to the `file` CLI option `--debug`.
            ///
            /// NOTE: Those messages are printed by `libmagic` itself, no this Rust crate.
            #[doc(alias = "MAGIC_DEBUG")]
            #[doc(alias = "--debug")]
            const DEBUG             = libmagic::MAGIC_DEBUG;

            /// If the file queried is a symlink, follow it
            ///
            /// This is equivalent to the `file` CLI option `--dereference`.
            #[doc(alias = "MAGIC_SYMLINK")]
            #[doc(alias = "--dereference")]
            const SYMLINK           = libmagic::MAGIC_SYMLINK;

            /// If the file is compressed, unpack it and look at the contents
            ///
            /// This is equivalent to the `file` CLI option `--uncompress`.
            #[doc(alias = "MAGIC_COMPRESS")]
            #[doc(alias = "--uncompress")]
            const COMPRESS          = libmagic::MAGIC_COMPRESS;

            /// If the file is a block or character special device, then open the device and try to look in its contents
            ///
            /// This is equivalent to the `file` CLI option `--special-files`.
            #[doc(alias = "MAGIC_DEVICES")]
            #[doc(alias = "--special-files")]
            const DEVICES           = libmagic::MAGIC_DEVICES;

            /// Return a MIME type string, instead of a textual description
            ///
            /// See also: [`Flags::MIME`]
            ///
            /// This is equivalent to the `file` CLI option `--mime-type`.
            ///
            /// NOTE: `libmagic` uses non-standard MIME types for at least some built-in checks,
            /// e.g. `inode/*` (also see [`Flags::SYMLINK`], [`Flags::DEVICES`]):
            /// ```shell
            /// $ file --mime-type /proc/self/exe
            /// /proc/self/exe: inode/symlink
            ///
            /// $ file --mime-type /dev/sda
            /// /dev/sda: inode/blockdevice
            /// ```
            #[doc(alias = "MAGIC_MIME_TYPE")]
            #[doc(alias = "--mime-type")]
            const MIME_TYPE         = libmagic::MAGIC_MIME_TYPE;

            /// Return all matches, not just the first
            ///
            /// This is equivalent to the `file` CLI option `--keep-going`.
            #[doc(alias = "MAGIC_CONTINUE")]
            #[doc(alias = "--keep-going")]
            const CONTINUE          = libmagic::MAGIC_CONTINUE;

            /// Check the magic database for consistency and print warnings to `stderr`
            ///
            /// NOTE: Those warnings are printed by `libmagic` itself, no this Rust crate.
            #[doc(alias = "MAGIC_CHECK")]
            const CHECK             = libmagic::MAGIC_CHECK;

            /// On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed
            ///
            /// This is equivalent to the `file` CLI option `--preserve-date`.
            #[doc(alias = "MAGIC_PRESERVE_ATIME")]
            #[doc(alias = "--preserve-date")]
            const PRESERVE_ATIME    = libmagic::MAGIC_PRESERVE_ATIME;

            /// Don't translate unprintable characters to a `\\ooo` octal representation
            ///
            /// This is equivalent to the `file` CLI option `--raw`.
            #[doc(alias = "MAGIC_RAW")]
            #[doc(alias = "--raw")]
            const RAW               = libmagic::MAGIC_RAW;

            /// Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer
            #[doc(alias = "MAGIC_ERROR")]
            const ERROR             = libmagic::MAGIC_ERROR;

            /// Return a MIME encoding, instead of a textual description
            ///
            /// See also: [`Flags::MIME`]
            ///
            /// This is equivalent to the `file` CLI option `--mime-encoding`.
            ///
            /// NOTE: `libmagic` uses non-standard MIME `charset` values, e.g. for binary files:
            /// ```shell
            /// $ file --mime-encoding /proc/self/exe
            /// binary
            /// ```
            #[doc(alias = "MAGIC_MIME_ENCODING")]
            #[doc(alias = "--mime-encoding")]
            const MIME_ENCODING     = libmagic::MAGIC_MIME_ENCODING;

            /// A shorthand for `MIME_TYPE | MIME_ENCODING`
            ///
            /// See also: [`Flags::MIME_TYPE`], [`Flags::MIME_ENCODING`]
            ///
            /// This is equivalent to the `file` CLI option `--mime`.
            ///
            /// NOTE: `libmagic` returns a parseable MIME type with a `charset` field:
            /// ```shell
            /// $ file --mime /proc/self/exe
            /// /proc/self/exe: inode/symlink; charset=binary
            /// ```
            #[doc(alias = "MAGIC_MIME")]
            #[doc(alias = "--mime")]
            const MIME              = Self::MIME_TYPE.bits()
                                    | Self::MIME_ENCODING.bits();

            /// Return the Apple creator and type
            ///
            /// This is equivalent to the `file` CLI option `--apple`.
            #[doc(alias = "MAGIC_APPLE")]
            #[doc(alias = "--apple")]
            const APPLE             = libmagic::MAGIC_APPLE;

            /// Return a slash-separated list of extensions for this file type
            ///
            /// This is equivalent to the `file` CLI option `--extension`.
            ///
            /// NOTE: `libmagic` returns a list with one or more extensions without a leading "." (dot):
            /// ```shell
            /// $ file --extension example.jpg
            /// example.jpg: jpeg/jpg/jpe/jfif
            ///
            /// $ file --extension /proc/self/exe
            /// /proc/self/exe: ???
            /// ```
            #[doc(alias = "MAGIC_EXTENSION")]
            #[doc(alias = "--extension")]
            const EXTENSION         = libmagic::MAGIC_EXTENSION;

            /// Don't report on compression, only report about the uncompressed data
            ///
            /// This is equivalent to the `file` CLI option `--uncompress-noreport`.
            #[doc(alias = "MAGIC_COMPRESS_TRANSP")]
            #[doc(alias = "--uncompress-noreport")]
            const COMPRESS_TRANSP   = libmagic::MAGIC_COMPRESS_TRANSP;

            /// A shorthand for `EXTENSION | MIME | APPLE`
            #[doc(alias = "MAGIC_NODESC")]
            const NODESC            = Self::EXTENSION.bits()
                                    | Self::MIME.bits()
                                    | Self::APPLE.bits();

            /// Don't look inside compressed files
            ///
            /// This is equivalent to the `file` CLI option `--exclude compress`.
            #[doc(alias = "MAGIC_NO_CHECK_COMPRESS")]
            #[doc(alias = "--exclude compress")]
            const NO_CHECK_COMPRESS = libmagic::MAGIC_NO_CHECK_COMPRESS;

            /// Don't examine tar files
            ///
            /// This is equivalent to the `file` CLI option `--exclude tar`.
            #[doc(alias = "MAGIC_NO_CHECK_TAR")]
            #[doc(alias = "--exclude tar")]
            const NO_CHECK_TAR      = libmagic::MAGIC_NO_CHECK_TAR;

            /// Don't consult magic files
            ///
            /// This is equivalent to the `file` CLI option `--exclude soft`.
            #[doc(alias = "MAGIC_NO_CHECK_SOFT")]
            #[doc(alias = "--exclude soft")]
            const NO_CHECK_SOFT     = libmagic::MAGIC_NO_CHECK_SOFT;

            /// Check for EMX application type (only on EMX)
            ///
            /// This is equivalent to the `file` CLI option `--exclude apptype`.
            #[doc(alias = "MAGIC_NO_CHECK_APPTYPE")]
            #[doc(alias = "--exclude apptype")]
            const NO_CHECK_APPTYPE  = libmagic::MAGIC_NO_CHECK_APPTYPE;

            /// Don't print ELF details
            ///
            /// This is equivalent to the `file` CLI option `--exclude elf`.
            #[doc(alias = "MAGIC_NO_CHECK_ELF")]
            #[doc(alias = "--exclude elf")]
            const NO_CHECK_ELF      = libmagic::MAGIC_NO_CHECK_ELF;

            /// Don't check for various types of text files
            ///
            /// This is equivalent to the `file` CLI option `--exclude text`.
            #[doc(alias = "MAGIC_NO_CHECK_TEXT")]
            #[doc(alias = "--exclude text")]
            const NO_CHECK_TEXT     = libmagic::MAGIC_NO_CHECK_TEXT;

            /// Don't get extra information on MS Composite Document Files
            ///
            /// This is equivalent to the `file` CLI option `--exclude cdf`.
            #[doc(alias = "MAGIC_NO_CHECK_CDF")]
            #[doc(alias = "--exclude cdf")]
            const NO_CHECK_CDF      = libmagic::MAGIC_NO_CHECK_CDF;

            /// Don't examine CSV files
            ///
            /// This is equivalent to the `file` CLI option `--exclude csv`.
            #[doc(alias = "MAGIC_NO_CHECK_CSV")]
            #[doc(alias = "--exclude csv")]
            const NO_CHECK_CSV      = libmagic::MAGIC_NO_CHECK_CSV;

            /// Don't look for known tokens inside ascii files
            ///
            /// This is equivalent to the `file` CLI option `--exclude tokens`.
            #[doc(alias = "MAGIC_NO_CHECK_TOKENS")]
            #[doc(alias = "--exclude tokens")]
            const NO_CHECK_TOKENS   = libmagic::MAGIC_NO_CHECK_TOKENS;

            /// Don't check text encodings
            ///
            /// This is equivalent to the `file` CLI option `--exclude encoding`.
            #[doc(alias = "MAGIC_NO_CHECK_ENCODING")]
            #[doc(alias = "--exclude encoding")]
            const NO_CHECK_ENCODING = libmagic::MAGIC_NO_CHECK_ENCODING;

            /// Don't examine JSON files
            ///
            /// This is equivalent to the `file` CLI option `--exclude json`.
            #[doc(alias = "MAGIC_NO_CHECK_JSON")]
            #[doc(alias = "--exclude json")]
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
    ///
    /// This is returned from [`DatabasePaths::new()`](DatabasePaths::new)
    #[derive(thiserror::Error, Debug)]
    #[error("invalid database files path")]
    pub struct InvalidDatabasePathError {}

    /// Magic database file paths
    ///
    /// `libmagic` requires database file paths for certain operations on a [`Cookie`] that must:
    /// - be a valid C string
    /// - not contain ":" (colon), since that is used to separate multiple file paths (on all platforms)
    ///
    /// Those operations are [`Cookie::load()`](Cookie::load), [`Cookie::compile()`](Cookie::compile), [`Cookie::check()`](Cookie::check), [`Cookie::list()`](Cookie::list).\
    /// [`Cookie::file()`](Cookie::file) does not take database file paths but the single file to inspect instead.
    ///
    /// The default unnamed database can be constructed with [`Default::default()`](DatabasePaths::default).  
    /// Explicit paths can be constructed manually with [`new()`](DatabasePaths::new) or by fallible conversion from an array, slice or Vec
    /// containing something convertible as [`std::path::Path`], or a single something.
    ///
    /// Note that this only ensures the paths themselves are valid.
    /// Operating on those database file paths can still fail,
    /// for example if they refer to files that do not exist, can not be opened or do not have the required format.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::TryInto;
    /// # use magic::cookie::DatabasePaths;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // `: DatabasePaths` type annotation is only needed for these examples
    /// // if you pass it to Cookie::load() etc., Rust will figure it out
    ///
    /// // construct default unnamed database paths
    /// let database: DatabasePaths = Default::default();
    ///
    /// // construct from single path
    /// let database: DatabasePaths = "first-directory/first-database".try_into()?;
    /// let database: DatabasePaths =
    ///     std::path::Path::new("second-directory/second-database").try_into()?;
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
        /// Create a new database paths instance
        ///
        /// Using one of the `TryFrom` implementations is recommended instead, see [`DatabasePaths`] examples.
        ///
        /// Empty `paths` returns [`Default::default()`](DatabasePaths::default).
        ///
        /// # Errors
        ///
        /// If the `paths` contain a ":" (colon), a [`cookie::InvalidDatabasePathError`](InvalidDatabasePathError) will be returned.
        ///
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
        /// Returns the path for the default unnamed database/s
        ///
        /// Note that the default database/s can be overwritten by setting the "MAGIC" environment variable
        /// to a colon-separated text of database file paths:
        /// ```shell
        /// $ export MAGIC='data/tests/db-python:data/tests/db-images-png-precompiled.mgc'
        /// $ # file-ish uses `DatabasePaths::default()`
        /// $ cargo run --example file-ish -- data/tests/rust-logo-128x128-blk.png
        /// ```
        /// This is a feature of `libmagic` itself, not of this Rust crate.
        ///
        /// Note that the `file` CLI (which uses `libmagic`) prints the location of its default database with:
        /// ```shell
        /// $ file --version
        /// file-5.38
        /// magic file from /etc/magic:/usr/share/misc/magic
        ///
        /// $ export MAGIC='data/tests/db-python:data/tests/db-images-png-precompiled.mgc'
        /// $ file --version
        /// file-5.39
        /// magic file from data/tests/db-python:data/tests/db-images-png-precompiled.mgc
        /// ```
        fn default() -> Self {
            Self { filenames: None }
        }
    }

    impl<P: AsRef<std::path::Path>, const N: usize> TryFrom<[P; N]> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
        fn try_from(value: [P; N]) -> Result<Self, <Self as TryFrom<[P; N]>>::Error> {
            Self::new(value)
        }
    }

    impl<P: AsRef<std::path::Path>> TryFrom<Vec<P>> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
        fn try_from(value: Vec<P>) -> Result<Self, <Self as TryFrom<Vec<P>>>::Error> {
            Self::new(value)
        }
    }

    impl<P: AsRef<std::path::Path>> TryFrom<&'_ [P]> for DatabasePaths {
        type Error = InvalidDatabasePathError;

        /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
        fn try_from(value: &[P]) -> Result<Self, <Self as TryFrom<&[P]>>::Error> {
            Self::new(value)
        }
    }

    macro_rules! databasepaths_try_from_impl {
        ($t:ty) => {
            impl TryFrom<$t> for DatabasePaths {
                type Error = InvalidDatabasePathError;

                /// Invokes [`DatabasePaths::new()`](DatabasePaths::new)
                fn try_from(value: $t) -> Result<Self, <Self as TryFrom<$t>>::Error> {
                    DatabasePaths::new(std::iter::once(value))
                }
            }
        };
    }

    // missing for now are:
    // - Cow<'_, OsStr>
    // - std::path::Component<'_>
    // - std::path::Components<'_>
    // - std::path::Iter<'_>
    databasepaths_try_from_impl!(&str);
    databasepaths_try_from_impl!(&std::ffi::OsStr);
    databasepaths_try_from_impl!(std::ffi::OsString);
    databasepaths_try_from_impl!(&std::path::Path);
    databasepaths_try_from_impl!(std::path::PathBuf);
    databasepaths_try_from_impl!(String);

    /// Error within several [`Cookie`] functions
    ///
    /// Most functions on a [`Cookie`] can return an error from `libmagic`,
    /// which unfortunately is not very structured.
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

    /// Combined configuration of [`Flags`], parameters and databases
    ///
    /// A "cookie" is `libmagic` lingo for a combined configuration of
    /// - [`cookie::Flags`](crate::cookie::Flags)
    /// - parameters (not implemented yet)
    /// - loaded datbases, e.g. [`cookie::DatabasePaths`](crate::cookie::DatabasePaths)
    ///
    /// A cookie advances through 2 states: opened, then loaded.
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // create new cookie in initial opened state with given flags
    /// let cookie = magic::Cookie::open(magic::cookie::Flags::default())?;
    ///
    /// // advance cookie into loaded state
    /// let cookie = cookie.load(&magic::cookie::DatabasePaths::default())?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// In either state, you can use operations that do not require
    /// already loaded magic databases:
    /// - [`Cookie::load()`](Cookie::load), [`Cookie::load_buffers()`](Cookie::load_buffers) to load databases and transition into the loaded state
    /// - [`Cookie::set_flags()`](Cookie::set_flags) to overwrite the initial flags given in [`Cookie::open()`](Cookie::open)
    /// - [`Cookie::compile()`](Cookie::compile), [`Cookie::check()`](Cookie::check), [`Cookie::list()`](Cookie::list) to operate on magic database files
    ///
    /// Once in the loaded state, you can perform magic "queries":
    /// - [`Cookie::file()`](Cookie::file), [`Cookie::buffer()`](Cookie::buffer)
    #[derive(Debug)]
    #[doc(alias = "magic_t")]
    #[doc(alias = "magic_set")]
    pub struct Cookie<S: State> {
        cookie: crate::ffi::Cookie,
        marker: std::marker::PhantomData<S>,
    }

    /// Error within [`Cookie::load()`](Cookie::load) or [`Cookie::load_buffers()`](Cookie::load_buffers)
    ///
    /// This is like [`cookie:Error`](Error) but also has the cookie in its original state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::TryInto;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let cookie = magic::Cookie::open(Default::default())?;
    /// let database = "data/tests/db-images-png".try_into()?;
    /// // try to load an existing database, consuming and returning early
    /// let cookie = cookie.load(&database)?;
    ///
    /// let database = "doesntexist.mgc".try_into()?;
    /// // load a database that does not exist
    /// let cookie = match cookie.load(&database) {
    ///     Err(err) => {
    ///         println!("whoopsie: {:?}", err);
    ///         // recover the loaded cookie without dropping it
    ///         err.cookie()
    ///     },
    ///     Ok(cookie) => cookie,
    /// };
    ///
    /// let database = "data/tests/db-python".try_into()?;
    /// // try to load another existing database
    /// let cookie = cookie.load(&database)?;
    /// # Ok(())
    /// # }
    /// ```
    #[derive(thiserror::Error, Debug)]
    #[error("magic cookie error in `libmagic` function {}", .function)]
    pub struct LoadError<S: State> {
        function: &'static str,
        //#[backtrace]
        source: crate::ffi::CookieError,
        cookie: Cookie<S>,
    }

    impl<S: State> LoadError<S> {
        /// Returns the cookie in its original state
        pub fn cookie(self) -> Cookie<S> {
            self.cookie
        }
    }

    impl<S: State> Drop for Cookie<S> {
        /// Closes the loaded magic database files and deallocates any resources used
        #[doc(alias = "magic_close")]
        fn drop(&mut self) {
            crate::ffi::close(&mut self.cookie);
        }
    }

    /// Operations that are valid in the `Open` state
    ///
    /// A new cookie created with [`Cookie::open`](Cookie::open) does not have any databases [loaded](Cookie::load).
    impl Cookie<Open> {
        /// Creates a new configuration cookie, `flags` specify how other operations on this cookie should behave
        ///
        /// This does not [`load()`](Cookie::load) any databases yet.
        ///
        /// The `flags` can be changed lateron with [`set_flags()`](Cookie::set_flags).
        ///
        /// # Examples
        ///
        /// ```
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// // open a new cookie with default flags
        /// let cookie = magic::Cookie::open(Default::default())?;
        ///
        /// // open a new cookie with custom flags
        /// let flags = magic::cookie::Flags::COMPRESS | magic::cookie::Flags::DEVICES;
        /// let cookie = magic::Cookie::open(flags)?;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error allocating a new cookie, a [`cookie::OpenError`](OpenError) will be returned.
        ///
        /// If the given `flags` are unsupported on the current platform, a [`cookie::OpenError`](OpenError) will be returned.
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
        /// Returns a textual description of the contents of the file `filename`
        ///
        /// Requires to [`load()`](Cookie::load) databases before calling.
        ///
        /// # Examples
        ///
        /// ```
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// // open a new cookie with default flags and database
        /// let cookie = magic::Cookie::open(Default::default())?.load(&Default::default())?;
        ///
        /// let file_description = cookie.file("data/tests/rust-logo-128x128-blk.png");
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::Error`](Error) will be returned.
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
        /// Requires to [`load()`](Cookie::load) databases before calling.
        ///
        /// # Examples
        ///
        /// ```
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// // open a new cookie with default flags and database
        /// let cookie = magic::Cookie::open(Default::default())?.load(&Default::default())?;
        ///
        /// let buffer = b"%PDF-\xE2\x80\xA6";
        /// let buffer_description = cookie.buffer(buffer);
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::Error`](Error) will be returned.
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
        /// Calling `load()` or [`load_buffers()`](Cookie::load_buffers) replaces the previously loaded database/s.
        ///
        /// This is equivalent to the using the `file` CLI:
        /// ```shell
        /// $ file --magic-file 'data/tests/db-images-png:data/tests/db-python' --version
        /// file-5.39
        /// magic file from data/tests/db-images-png:data/tests/db-python
        /// ```
        ///
        /// # Examples
        /// ```rust
        /// # use std::convert::TryInto;
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// // open a new cookie with default flags
        /// let cookie = magic::Cookie::open(Default::default())?;
        ///
        /// // load the default unnamed database
        /// let database = Default::default();
        /// let cookie = cookie.load(&database)?;
        ///
        /// // load databases from files
        /// let databases = ["data/tests/db-images-png", "data/tests/db-python"].try_into()?;
        /// let cookie = cookie.load(&databases)?;
        ///
        /// // load precompiled database from file
        /// let database = "data/tests/db-images-png-precompiled.mgc".try_into()?;
        /// let cookie = cookie.load(&database)?;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::LoadError`](LoadError) will be returned,
        /// which contains the cookie in its original state.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_load")]
        #[doc(alias = "--magic-file")]
        pub fn load(self, filenames: &DatabasePaths) -> Result<Cookie<Load>, LoadError<S>> {
            match crate::ffi::load(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(LoadError {
                    function: "magic_load",
                    source: err,
                    cookie: self,
                }),
                Ok(_) => {
                    let mut cookie = std::mem::ManuallyDrop::new(self);

                    let cookie = Cookie {
                        cookie: crate::ffi::Cookie::new(&mut cookie.cookie),
                        marker: std::marker::PhantomData,
                    };
                    Ok(cookie)
                }
            }
        }

        /// Loads the given compiled databases `buffers` for further queries
        ///
        /// Databases need to be compiled with a compatible `libmagic` version.
        ///
        /// This function can be used in environments where `libmagic` does
        /// not have direct access to the filesystem, but can access the magic
        /// database via shared memory or other IPC means.
        ///
        /// Calling `load_buffers()` or [`load()`](Cookie::load) replaces the previously loaded database/s.
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::LoadError`](LoadError) will be returned,
        /// which contains the cookie in its original state.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_load_buffers")]
        pub fn load_buffers(self, buffers: &[&[u8]]) -> Result<Cookie<Load>, LoadError<S>> {
            match crate::ffi::load_buffers(&self.cookie, buffers) {
                Err(err) => Err(LoadError {
                    function: "magic_load_buffers",
                    source: err,
                    cookie: self,
                }),
                Ok(_) => {
                    let mut cookie = std::mem::ManuallyDrop::new(self);

                    let cookie = Cookie {
                        cookie: crate::ffi::Cookie::new(&mut cookie.cookie),
                        marker: std::marker::PhantomData,
                    };
                    Ok(cookie)
                }
            }
        }

        /// Sets the `flags` to use for this configuration
        ///
        /// Overwrites any previously set flags, e.g. those from [`load()`](Cookie::load).
        ///
        /// # Examples
        /// ```rust
        /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
        /// // open a new cookie with initial default flags
        /// let cookie = magic::Cookie::open(Default::default())?;
        ///
        /// // overwrite the initial flags
        /// let flags = magic::cookie::Flags::COMPRESS | magic::cookie::Flags::DEVICES;
        /// cookie.set_flags(flags)?;
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// # Errors
        ///
        /// If the given `flags` are unsupported on the current platform, an [`cookie::SetFlagsError`](SetFlagsError) will be returned.
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

        /// Compiles the given database files `filenames` for faster access
        ///
        /// The compiled files created are named from the `basename` of each file argument with ".mgc" appended to it.
        ///
        /// This is equivalent to the following `file` CLI command:
        /// ```shell
        /// $ file --compile --magic-file data/tests/db-images-png:data/tests/db-python
        /// ```
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::Error`](Error) will be returned.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_compile")]
        #[doc(alias = "--compile")]
        pub fn compile(&self, filenames: &DatabasePaths) -> Result<(), Error> {
            match crate::ffi::compile(&self.cookie, filenames.filenames.as_deref()) {
                Err(err) => Err(Error {
                    function: "magic_compile",
                    source: err,
                }),
                Ok(_) => Ok(()),
            }
        }

        /// Checks the validity of entries in the database files `filenames`
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::Error`](Error) will be returned.
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

        /// Dumps all magic entries in the given database files `filenames` in a human readable format
        ///
        /// This is equivalent to the following `file` CLI command:
        /// ```shell
        /// $ file --checking-printout --magic-file data/tests/db-images-png:data/tests/db-python
        /// ```
        ///
        /// # Errors
        ///
        /// If there was an `libmagic` internal error, a [`cookie::Error`](Error) will be returned.
        ///
        /// # Panics
        ///
        /// Panics if `libmagic` violates its API contract, e.g. by not setting the last error or returning undefined data.
        #[doc(alias = "magic_list")]
        #[doc(alias = "--checking-printout")]
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

    /// Error within [`Cookie::open()`](Cookie::open)
    ///
    /// Note that a similar [`cookie::SetFlagsError`](SetFlagsError) can also occur
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

    /// Error within [`Cookie::set_flags()`](Cookie::set_flags)
    ///
    /// Note that a similar [`cookie::OpenError`](OpenError) can also occur
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
