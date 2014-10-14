extern crate libc;

use libc::size_t;
use std::path::Path;
use std::string;
use std::ptr;

/// Bitmask flags which control `libmagic` behaviour
#[unstable]
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


mod ffi {
    use libc::{c_char, c_int, size_t};

    pub enum Magic {}

    #[allow(dead_code)]
    #[link(name = "magic")]
    extern "C" {
        pub fn magic_open(flags: c_int) -> *const Magic;
        pub fn magic_close(cookie: *const Magic);
        pub fn magic_error(cookie: *const Magic) -> *const c_char;
        pub fn magic_errno(cookie: *const Magic) -> *const c_int;
        pub fn magic_descriptor(cookie: *const Magic, fd: c_int) -> *const c_char;
        pub fn magic_file(cookie: *const Magic, filename: *const c_char) -> *const c_char;
        pub fn magic_buffer(cookie: *const Magic, buffer: *const u8, length: size_t) -> *const c_char;
        pub fn magic_setflags(cookie: *const Magic, flags: c_int) -> c_int;
        pub fn magic_check(cookie: *const Magic, filename: *const c_char) -> c_int;
        pub fn magic_compile(cookie: *const Magic, filename: *const c_char) -> c_int;
        pub fn magic_list(cookie: *const Magic, filename: *const c_char) -> c_int;
        pub fn magic_load(cookie: *const Magic, filename: *const c_char) -> c_int;
    }
}


#[experimental]
pub struct MagicError;


#[unstable]
pub struct Cookie {
    cookie: *const self::ffi::Magic,
}

#[unstable]
impl Drop for Cookie {
    fn drop(&mut self) { unsafe { self::ffi::magic_close(self.cookie) } }
}

#[experimental]
impl Cookie {
    pub fn file(&self, filename: &Path) -> Option<String> {
        unsafe {
            let cookie = self.cookie;
            let s = filename.with_c_str(|filename| self::ffi::magic_file(cookie, filename));
            if s.is_null() { None } else { Some(string::raw::from_buf(s as *const u8)) }
        }
    }

    pub fn buffer(&self, buffer: &[u8]) -> Option<String> {
        unsafe {
            let buffer_len = buffer.len() as size_t;
            let pbuffer = buffer.as_ptr();
            let s = self::ffi::magic_buffer(self.cookie, pbuffer, buffer_len);
            if s.is_null() { None } else { Some(string::raw::from_buf(s as *const u8)) }
        }
    }

    pub fn error(&self) -> Option<String> {
        unsafe {
            let s = self::ffi::magic_error(self.cookie);
            if s.is_null() { None } else { Some(string::raw::from_buf(s as *const u8)) }
        }
    }

    pub fn setflags(&self, flags: self::flags::CookieFlags) {
        unsafe {
            self::ffi::magic_setflags(self.cookie, flags.bits());
        }
    }

    pub fn check(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| self::ffi::magic_check(cookie, filename)) == 0
        }
    }

    pub fn compile(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| self::ffi::magic_compile(cookie, filename)) == 0
        }
    }

    pub fn list(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| self::ffi::magic_list(cookie, filename)) == 0
        }
    }

    pub fn load(&self, filename: &Path) -> Result<(), MagicError> {
        unsafe {
            let cookie = self.cookie;
            let ret = filename.with_c_str(|filename| self::ffi::magic_load(cookie, filename));
            if 0 == ret { Ok(()) } else { Err(self::MagicError) }
        }
    }

    pub fn load_default(&self) -> bool {
        unsafe {
            let cookie = self.cookie;
            self::ffi::magic_load(cookie, ptr::null()) == 0
        }
    }

    pub fn open(flags: self::flags::CookieFlags) -> Option<Cookie> {
        unsafe {
            let cookie = self::ffi::magic_open((flags | self::flags::ERROR).bits());
            if cookie.is_null() { None } else { Some(Cookie{cookie: cookie,}) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cookie;
	use super::flags;

    #[test]
    fn file() {
        let cookie = Cookie::open(flags::NONE).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")).is_ok());

        let path = Path::new("assets/rust-logo-128x128-blk.png");

        assert_eq!(cookie.file(&path).unwrap().as_slice(), "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

        cookie.setflags(flags::MIME_TYPE);
        assert_eq!(cookie.file(&path).unwrap().as_slice(), "image/png");

        cookie.setflags(flags::MIME_TYPE | flags::MIME_ENCODING);
        assert_eq!(cookie.file(&path).unwrap().as_slice(), "image/png; charset=binary");
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(flags::NONE).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")).is_ok());

        let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
        assert_eq!(cookie.buffer(s).unwrap().as_slice(), "a python script, ASCII text executable");

        cookie.setflags(flags::MIME_TYPE);
        assert_eq!(cookie.buffer(s).unwrap().as_slice(), "text/plain");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(flags::NONE | flags::ERROR).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")).is_ok());

        let ret = cookie.file(&Path::new("non-existent_file.txt"));
        assert_eq!(ret, None);
        assert_eq!(cookie.error().unwrap().as_slice(), "cannot stat `non-existent_file.txt' (No such file or directory)");
    }

    #[test]
    fn load_default() {
        let cookie = Cookie::open(flags::NONE).unwrap();
        assert!(cookie.load_default());
    }
}
