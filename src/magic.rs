extern crate libc;

use libc::{c_char, c_int, size_t};
use std::path::Path;
use std::string;

enum Magic {}

pub struct MagicFlag {
    flag: c_int
}

impl BitOr<MagicFlag, MagicFlag> for MagicFlag {
    fn bitor(&self, other: &MagicFlag) -> MagicFlag {
        MagicFlag{flag: self.flag | other.flag}
    }
}

/// No flags
pub static MAGIC_NONE: MagicFlag = MagicFlag{flag: 0x000000};

/// Turn on debugging
pub static MAGIC_DEBUG: MagicFlag = MagicFlag{flag: 0x000001};

/// Follow symlinks
pub static MAGIC_SYMLINK: MagicFlag = MagicFlag{flag: 0x000002};

/// Check inside compressed files
pub static MAGIC_COMPRESS: MagicFlag = MagicFlag{flag: 0x000004};

/// Look at the contents of devices
pub static MAGIC_DEVICES: MagicFlag = MagicFlag{flag: 0x000008};

/// Return the MIME type
pub static MAGIC_MIME_TYPE: MagicFlag = MagicFlag{flag: 0x000010};

/// Return all matches
pub static MAGIC_CONTINUE: MagicFlag = MagicFlag{flag: 0x000020};

/// Print warnings to stderr
pub static MAGIC_CHECK: MagicFlag = MagicFlag{flag: 0x000040};

/// Restore access time on exit
pub static MAGIC_PRESERVE_ATIME: MagicFlag = MagicFlag{flag: 0x000080};

/// Don't translate unprintable chars
pub static MAGIC_RAW: MagicFlag = MagicFlag{flag: 0x000100};

/// Handle ENOENT etc as real errors
pub static MAGIC_ERROR: MagicFlag = MagicFlag{flag: 0x000200};

/// Return the MIME encoding
pub static MAGIC_MIME_ENCODING: MagicFlag = MagicFlag{flag: 0x000400};

/// `MAGIC_MIME_TYPE` and `MAGIC_MIME_ENCODING`
pub static MAGIC_MIME: MagicFlag = MagicFlag{flag: 0x000410};

/// Return the Apple creator and type
pub static MAGIC_APPLE: MagicFlag = MagicFlag{flag: 0x000800};

/// Don't check for compressed files
pub static MAGIC_NO_CHECK_COMPRESS: MagicFlag = MagicFlag{flag: 0x001000};

/// Don't check for tar files
pub static MAGIC_NO_CHECK_TAR: MagicFlag = MagicFlag{flag: 0x002000};

/// Don't check magic entries
pub static MAGIC_NO_CHECK_SOFT: MagicFlag = MagicFlag{flag: 0x004000};

/// Don't check application type
pub static MAGIC_NO_CHECK_APPTYPE: MagicFlag = MagicFlag{flag: 0x008000};

/// Don't check for elf details
pub static MAGIC_NO_CHECK_ELF: MagicFlag = MagicFlag{flag: 0x010000};

/// Don't check for text files
pub static MAGIC_NO_CHECK_TEXT: MagicFlag = MagicFlag{flag: 0x020000};

/// Don't check for cdf files
pub static MAGIC_NO_CHECK_CDF: MagicFlag = MagicFlag{flag: 0x040000};

/// Don't check tokens
pub static MAGIC_NO_CHECK_TOKENS: MagicFlag = MagicFlag{flag: 0x100000};

/// Don't check text encodings
pub static MAGIC_NO_CHECK_ENCODING: MagicFlag = MagicFlag{flag: 0x200000};

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

    pub fn setflags(&self, flags: MagicFlag) {
        unsafe {
            magic_setflags(self.cookie, flags.flag);
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

    pub fn open(flags: MagicFlag) -> Option<Cookie> {
        unsafe {
            let cookie = magic_open(flags.flag | MAGIC_ERROR.flag);
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
        assert_eq!(cookie.buffer(s).unwrap().as_slice(), "text/x-python");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(MAGIC_NONE | MAGIC_ERROR).unwrap();
        assert!(cookie.load(&Path::new("/usr/share/misc/magic")));

        let ret = cookie.file(&Path::new("non-existent_file.txt"));
        assert_eq!(ret, None);
        assert_eq!(cookie.error().unwrap().as_slice(), "cannot stat `non-existent_file.txt' (No such file or directory)");
    }
}
