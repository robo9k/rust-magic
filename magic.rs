#[link(name = "magic",
       vers = "0.1.0",
       uuid = "201abfd3-0d07-41ab-a6c5-9eb94b318383",
       url = "https://github.com/thestinger/rust-magic")];

#[comment = "libmagic bindings"];
#[license = "MIT"];
#[crate_type = "lib"];

use std::libc::{c_char, c_int, size_t};
use std::ptr::is_null;
use std::str::as_c_str;
use std::{str, vec};

enum Magic {}

pub struct MagicFlag {
    priv flag: c_int
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

#[link_args = "-lmagic"]
extern "C" {
    fn magic_open(flags: c_int) -> *Magic;
    fn magic_close(cookie: *Magic);
    fn magic_error(cookie: *Magic) -> *c_char;
    fn magic_errno(cookie: *Magic) -> c_int;
    fn magic_descriptor(cookie: *Magic, fd: c_int) -> *c_char;
    fn magic_file(cookie: *Magic, filename: *c_char) -> *c_char;
    fn magic_buffer(cookie: *Magic, buffer: *u8, length: size_t) -> *c_char;
    fn magic_setflags(cookie: *Magic, flags: c_int) -> c_int;
    fn magic_check(cookie: *Magic, filename: *c_char) -> c_int;
    fn magic_compile(cookie: *Magic, filename: *c_char) -> c_int;
    fn magic_list(cookie: *Magic, filename: *c_char) -> c_int;
    fn magic_load(cookie: *Magic, filename: *c_char) -> c_int;
}

pub struct Cookie {
    priv cookie: *Magic,
}

impl Drop for Cookie {
    fn finalize(&self) { unsafe { magic_close(self.cookie) } }
}

impl Cookie {
    pub fn file(&self, filename: &str) -> Option<~str> {
        unsafe {
            let cookie = self.cookie;
            let s = as_c_str(filename, |filename| magic_file(cookie, filename));
            if is_null(s) { None } else { Some(str::raw::from_c_str(s)) }
        }
    }

    pub fn buffer(&self, buffer: &[u8]) -> Option<~str> {
        unsafe {
            let buffer_len = buffer.len() as size_t;
            let pbuffer = vec::raw::to_ptr(buffer);
            let s = magic_buffer(self.cookie, pbuffer, buffer_len);
            if is_null(s) { None } else { Some(str::raw::from_c_str(s)) }
        }
    }

    pub fn error(&self) -> Option<~str> {
        unsafe {
            let s = magic_error(self.cookie);
            if is_null(s) { None } else { Some(str::raw::from_c_str(s)) }
        }
    }

    pub fn setflags(&self, flags: MagicFlag) {
        unsafe {
            magic_setflags(self.cookie, flags.flag);
        }
    }

    pub fn check(&self, filename: &str) -> bool {
        unsafe {
            let cookie = self.cookie;
            as_c_str(filename, |filename| magic_check(cookie, filename)) == 0
        }
    }

    pub fn compile(&self, filename: &str) -> bool {
        unsafe {
            let cookie = self.cookie;
            as_c_str(filename, |filename| magic_compile(cookie, filename)) == 0
        }
    }

    pub fn list(&self, filename: &str) -> bool {
        unsafe {
            let cookie = self.cookie;
            as_c_str(filename, |filename| magic_list(cookie, filename)) == 0
        }
    }

    pub fn load(&self, filename: &str) -> bool {
        unsafe {
            let cookie = self.cookie;
            as_c_str(filename, |filename| magic_load(cookie, filename)) == 0
        }
    }

    pub fn open(flags: MagicFlag) -> Option<Cookie> {
        unsafe {
            let cookie = magic_open(flags.flag);
            if is_null(cookie) { None } else { Some(Cookie{cookie: cookie,}) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn file() {
        let cookie = Cookie::open(MAGIC_NONE).unwrap();
        assert!(cookie.load("/usr/share/file/misc/magic.mgc"));

        assert!(cookie.file("rust-logo-128x128-blk.png").unwrap() ==
                ~"PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

        cookie.setflags(MAGIC_MIME_TYPE);
        assert!(cookie.file("rust-logo-128x128-blk.png").unwrap() ==
                ~"image/png");

        cookie.setflags(MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING);
        assert!(cookie.file("rust-logo-128x128-blk.png").unwrap() ==
                ~"image/png; charset=binary");
    }

    #[test]
    fn buffer() {
        let cookie = Cookie::open(MAGIC_NONE).unwrap();
        assert!(cookie.load("/usr/share/file/misc/magic.mgc"));

        let s = ~"#!/usr/bin/env python3\nprint('Hello, world!')";
        assert!(str::as_bytes(&s, |bytes| {
          cookie.buffer(*bytes)
        }).unwrap() == ~"Python script, ASCII text executable");

        cookie.setflags(MAGIC_MIME_TYPE);
        assert!(str::as_bytes(&s, |bytes| {
          cookie.buffer(*bytes)
        }).unwrap() == ~"text/x-python");
    }

    #[test]
    fn file_error() {
        let cookie = Cookie::open(MAGIC_NONE).unwrap();
        assert!(cookie.load("/usr/share/file/misc/magic.mgc"));

        let ret = cookie.file("non-existent_file.txt");
        assert!(ret.is_none());
        assert!(cookie.error().unwrap() ==
                ~"cannot open `non-existent_file.txt' (No such file or directory)");
    }
}
