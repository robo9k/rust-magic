extern crate libc;

use libc::{c_char, c_int, size_t};
use std::path::Path;
use std::string;

enum Magic {}

bitflags! {
    flags Flags: c_int {
        #[doc = "No flags"]
        static MAGIC_NONE              = 0x000000,

        #[doc = "Turn on debugging"]
        static MAGIC_DEBUG             = 0x000001,

        #[doc = "Follow symlinks"]
        static MAGIC_SYMLINK           = 0x000002,

        #[doc = "Check inside compressed files"]
        static MAGIC_COMPRESS          = 0x000004,

        #[doc = "Look at the contents of devices"]
        static MAGIC_DEVICES           = 0x000008,

        #[doc = "Return the MIME type"]
        static MAGIC_MIME_TYPE         = 0x000010,

        #[doc = "Return all matches"]
        static MAGIC_CONTINUE          = 0x000020,

        #[doc = "Print warnings to stderr"]
        static MAGIC_CHECK             = 0x000040,

        #[doc = "Restore access time on exit"]
        static MAGIC_PRESERVE_ATIME    = 0x000080,

        #[doc = "Don't translate unprintable chars"]
        static MAGIC_RAW               = 0x000100,

        #[doc = "Handle ENOENT etc as real errors"]
        static MAGIC_ERROR             = 0x000200,

        #[doc = "Return the MIME encoding"]
        static MAGIC_MIME_ENCODING     = 0x000400,

        #[doc = "Return the MIME type and encoding"]
        static MAGIC_MIME              = MAGIC_MIME_TYPE.bits
                                       | MAGIC_MIME_ENCODING.bits,

        #[doc = "Return the Apple creator and type"]
        static MAGIC_APPLE             = 0x000800,

        #[doc = "Don't check for compressed files"]
        static MAGIC_NO_CHECK_COMPRESS = 0x001000,

        #[doc = "Don't check for tar files"]
        static MAGIC_NO_CHECK_TAR      = 0x002000,

        #[doc = "Don't check magic entries"]
        static MAGIC_NO_CHECK_SOFT     = 0x004000,

        #[doc = "Don't check application type"]
        static MAGIC_NO_CHECK_APPTYPE  = 0x008000,

        #[doc = "Don't check for elf details"]
        static MAGIC_NO_CHECK_ELF      = 0x010000,

        #[doc = "Don't check for text files"]
        static MAGIC_NO_CHECK_TEXT     = 0x020000,

        #[doc = "Don't check for cdf files"]
        static MAGIC_NO_CHECK_CDF      = 0x040000,

        #[doc = "Don't check tokens"]
        static MAGIC_NO_CHECK_TOKENS   = 0x100000,

        #[doc = "Don't check text encodings"]
        static MAGIC_NO_CHECK_ENCODING = 0x200000,

        #[doc = "No built-in tests; only consult the magic file"]
        static MAGIC_NO_CHECK_BUILTIN  = MAGIC_NO_CHECK_COMPRESS.bits
                                       | MAGIC_NO_CHECK_TAR.bits
                                       | MAGIC_NO_CHECK_APPTYPE.bits
                                       | MAGIC_NO_CHECK_ELF.bits
                                       | MAGIC_NO_CHECK_TEXT.bits
                                       | MAGIC_NO_CHECK_CDF.bits
                                       | MAGIC_NO_CHECK_TOKENS.bits
                                       | MAGIC_NO_CHECK_ENCODING.bits
                                       | 0,
	}
}

#[allow(dead_code)]
#[link(name = "magic")]
extern "C" {
    fn magic_open(flags: c_int) -> *const Magic;
    fn magic_close(cookie: *const Magic);
    fn magic_error(cookie: *const Magic) -> *const c_char;
    fn magic_errno(cookie: *const Magic) -> *const c_int;
    fn magic_descriptor(cookie: *const Magic, fd: c_int) -> *const c_char;
    fn magic_file(cookie: *const Magic, filename: *const c_char) -> *const c_char;
    fn magic_buffer(cookie: *const Magic, buffer: *const u8, length: size_t) -> *const c_char;
    fn magic_setflags(cookie: *const Magic, flags: c_int) -> c_int;
    fn magic_check(cookie: *const Magic, filename: *const c_char) -> c_int;
    fn magic_compile(cookie: *const Magic, filename: *const c_char) -> c_int;
    fn magic_list(cookie: *const Magic, filename: *const c_char) -> c_int;
    fn magic_load(cookie: *const Magic, filename: *const c_char) -> c_int;
    fn magic_version() -> c_int;
}

pub fn version() -> c_int {
    unsafe {
        magic_version()
    }
}

pub struct Cookie {
    cookie: *const Magic,
}

impl Drop for Cookie {
    fn drop(&mut self) { unsafe { magic_close(self.cookie) } }
}

impl Cookie {
    pub fn file(&self, filename: &Path) -> Option<String> {
        unsafe {
            let cookie = self.cookie;
            let s = filename.with_c_str(|filename| magic_file(cookie, filename));
            if s.is_null() { None } else { Some(string::raw::from_buf(s as *const u8)) }
        }
    }

    pub fn buffer(&self, buffer: &[u8]) -> Option<String> {
        unsafe {
            let buffer_len = buffer.len() as size_t;
            let pbuffer = buffer.as_ptr();
            let s = magic_buffer(self.cookie, pbuffer, buffer_len);
            if s.is_null() { None } else { Some(string::raw::from_buf(s as *const u8)) }
        }
    }

    pub fn error(&self) -> Option<String> {
        unsafe {
            let s = magic_error(self.cookie);
            if s.is_null() { None } else { Some(string::raw::from_buf(s as *const u8)) }
        }
    }

    pub fn setflags(&self, flags: Flags) {
        unsafe {
            magic_setflags(self.cookie, flags.bits);
        }
    }

    pub fn check(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| magic_check(cookie, filename)) == 0
        }
    }

    pub fn compile(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| magic_compile(cookie, filename)) == 0
        }
    }

    pub fn list(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| magic_list(cookie, filename)) == 0
        }
    }

    pub fn load(&self, filename: &Path) -> bool {
        unsafe {
            let cookie = self.cookie;
            filename.with_c_str(|filename| magic_load(cookie, filename)) == 0
        }
    }

    pub fn open(flags: Flags) -> Option<Cookie> {
        unsafe {
            let cookie = magic_open(flags.bits | MAGIC_ERROR.bits);
            if cookie.is_null() { None } else { Some(Cookie{cookie: cookie,}) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cookie, MAGIC_NONE, MAGIC_MIME_TYPE, MAGIC_MIME_ENCODING, MAGIC_ERROR};

    #[test]
    fn file() {
        let cookie = Cookie::open(MAGIC_NONE).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")));

        let path = Path::new("assets/rust-logo-128x128-blk.png");

        assert_eq!(cookie.file(&path).unwrap().as_slice(), "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

        cookie.setflags(MAGIC_MIME_TYPE);
        assert_eq!(cookie.file(&path).unwrap().as_slice(), "image/png");

        cookie.setflags(MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING);
        assert_eq!(cookie.file(&path).unwrap().as_slice(), "image/png; charset=binary");
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(MAGIC_NONE).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")));

        let s = b"#!/usr/bin/env python\nprint('Hello, world!')";
        assert_eq!(cookie.buffer(s).unwrap().as_slice(), "a python script, ASCII text executable");

        cookie.setflags(MAGIC_MIME_TYPE);
        assert_eq!(cookie.buffer(s).unwrap().as_slice(), "text/plain");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(MAGIC_NONE | MAGIC_ERROR).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")));

        let ret = cookie.file(&Path::new("non-existent_file.txt"));
        assert_eq!(ret, None);
        assert_eq!(cookie.error().unwrap().as_slice(), "cannot stat `non-existent_file.txt' (No such file or directory)");
    }

    #[test]
    fn version() {
        let version = ::version();
		assert!(version >= 513);
    }
}
