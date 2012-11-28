extern mod std;

use libc::c_int;
use libc::size_t;
use libc::c_char;
use libc::c_void;
use ptr::is_null;
use str::as_c_str;

export open;

enum Magic {}

enum MagicFlag {
  /// No flags
  MAGIC_NONE              = 0x000000,
  /// Turn on debugging
  MAGIC_DEBUG             = 0x000001,
  /// Follow symlinks
  MAGIC_SYMLINK           = 0x000002,
  /// Check inside compressed files
  MAGIC_COMPRESS          = 0x000004,
  /// Look at the contents of devices
  MAGIC_DEVICES           = 0x000008,
  /// Return the MIME type
  MAGIC_MIME_TYPE         = 0x000010,
  /// Return all matches
  MAGIC_CONTINUE          = 0x000020,
  /// Print warnings to stderr
  MAGIC_CHECK             = 0x000040,
  /// Restore access time on exit
  MAGIC_PRESERVE_ATIME    = 0x000080,
  /// Don't translate unprintable chars
  MAGIC_RAW               = 0x000100,
  /// Handle ENOENT etc as real errors
  MAGIC_ERROR             = 0x000200,
  /// Return the MIME encoding
  MAGIC_MIME_ENCODING     = 0x000400,
  /// `MAGIC_MIME_TYPE` and `MAGIC_MIME_ENCODING`
  MAGIC_MIME              = 0x000410,
  /// Return the Apple creator and type
  MAGIC_APPLE             = 0x000800,
  /// Don't check for compressed files
  MAGIC_NO_CHECK_COMPRESS = 0x001000,
  /// Don't check for tar files
  MAGIC_NO_CHECK_TAR      = 0x002000,
  /// Don't check magic entries
  MAGIC_NO_CHECK_SOFT     = 0x004000,
  /// Don't check application type
  MAGIC_NO_CHECK_APPTYPE  = 0x008000,
  /// Don't check for elf details
  MAGIC_NO_CHECK_ELF      = 0x010000,
  /// Don't check for text files
  MAGIC_NO_CHECK_TEXT     = 0x020000,
  /// Don't check for cdf files
  MAGIC_NO_CHECK_CDF      = 0x040000,
  /// Don't check tokens
  MAGIC_NO_CHECK_TOKENS   = 0x100000,
  /// Don't check text encodings
  MAGIC_NO_CHECK_ENCODING = 0x200000,
}

fn combine_flags(flags: &[MagicFlag]) -> c_int {
  vec::foldl(0 as c_int, flags, { |a: c_int, b: &MagicFlag| a | (*b as c_int) })
}

extern mod magic {
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

use magic::*;

struct Cookie {
  priv cookie: *Magic,

  drop {
    magic_close(self.cookie);
  }
}

impl Cookie {
  fn file(&self, filename: &str) -> Option<~str> unsafe {
    let cookie = self.cookie;
    let text = as_c_str(filename, { |filename| magic_file(cookie, filename) });

    if is_null(text) {
      None
    } else {
      Some(str::raw::from_c_str(text))
    }
  }

  fn buffer(&self, buffer: &[u8]) -> Option<~str> unsafe {
    let buffer_len = vec::len(buffer) as size_t;
    let pbuffer = vec::raw::to_ptr(buffer);
    let text = magic_buffer(self.cookie, pbuffer, buffer_len);
    if is_null(text) {
      None
    } else {
      Some(str::raw::from_c_str(text))
    }
  }

  fn error() -> Option<~str> unsafe {
    let text = magic_error(self.cookie);
    if is_null(text) {
      None
    } else {
      Some(str::raw::from_c_str(text))
    }
  }

  fn setflags(&self, flags: &[MagicFlag]) {
    magic_setflags(self.cookie, combine_flags(flags));
  }

  fn check(&self, filename: &str) -> bool {
    let cookie = self.cookie;
    as_c_str(filename, { |filename| magic_check(cookie, filename) }) == 0
  }

  fn compile(&self, filename: &str) -> bool {
    let cookie = self.cookie;
    as_c_str(filename, { |filename| magic_compile(cookie, filename) }) == 0
  }

  fn list(&self, filename: &str) -> bool {
    let cookie = self.cookie;
    as_c_str(filename, { |filename| magic_list(cookie, filename) }) == 0
  }

  fn load(&self, filename: &str) -> bool {
    let cookie = self.cookie;
    as_c_str(filename, { |filename| magic_load(cookie, filename) }) == 0
  }
}

fn open(flags: &[MagicFlag]) -> Option<Cookie> {
  let cookie = magic_open(combine_flags(flags));
  if is_null(cookie) {
    None
  } else {
    Some(Cookie { cookie: cookie })
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn file() {
    let cookie = option::unwrap(open([MAGIC_NONE]));
    assert(cookie.load("/usr/share/file/magic"));

    assert(option::unwrap(cookie.file("rust-logo-128x128-blk.png")) ==
           ~"PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

    cookie.setflags([MAGIC_MIME_TYPE]);
    assert(option::unwrap(cookie.file("rust-logo-128x128-blk.png")) ==
           ~"image/png");

    cookie.setflags([MAGIC_MIME_TYPE, MAGIC_MIME_ENCODING]);
    assert(option::unwrap(cookie.file("rust-logo-128x128-blk.png")) ==
           ~"image/png; charset=binary");
  }

  #[test]
  fn buffer() {
    let cookie = option::unwrap(open([MAGIC_NONE]));
    assert(cookie.load("/usr/share/file/magic"));

    let s = ~"#!/usr/bin/env python3\nprint('Hello, world!')";
    let text = option::unwrap(str::as_bytes(&s, |bytes| {
      cookie.buffer(*bytes)
    }));
    assert(text == ~"Python script, ASCII text executable");

    cookie.setflags([MAGIC_MIME_TYPE]);
    let text = option::unwrap(str::as_bytes(&s, |bytes| {
      cookie.buffer(*bytes)
    }));
    assert(text == ~"text/x-python");
  }

  #[test]
  fn file_error() {
    let cookie = option::unwrap(open([MAGIC_NONE]));
    assert(cookie.load("/usr/share/file/magic"));

    let ret = cookie.file("non-existent_file.txt");
    assert(ret.is_none());
    assert(option::unwrap(cookie.error()) ==
           ~"cannot open `non-existent_file.txt' (No such file or directory)");
  }
}
