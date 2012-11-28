extern mod std;

use libc::c_int;
use libc::size_t;
use libc::c_char;
use libc::c_void;
use ptr::is_null;

export open;

enum magic {}

extern mod magic {
  fn magic_open(flags: c_int) -> *magic;
  fn magic_close(cookie: *magic);
  fn magic_error(cookie: *magic) -> *c_char;
  fn magic_errno(cookie: *magic) -> c_int;
  fn magic_descriptor(cookie: *magic, fd: c_int) -> *c_char;
  fn magic_file(cookie: *magic, filename: *c_char) -> *c_char;
  fn magic_buffer(cookie: *magic, buffer: *u8, length: size_t) -> *c_char;
  fn magic_setflags(cookie: *magic, flags: c_int) -> c_int;
  fn magic_check(cookie: *magic, filename: *c_char) -> c_int;
  fn magic_compile(cookie: *magic, filename: *c_char) -> c_int;
  fn magic_list(cookie: *magic, filename: *c_char) -> c_int;
  fn magic_load(cookie: *magic, filename: *c_char) -> c_int;
}

use magic::*;

struct Cookie {
  priv cookie: *magic,

  drop {
    magic_close(self.cookie);
  }
}

impl Cookie {
  fn load(&self, filename: &str) -> bool unsafe {
    let cookie = self.cookie;
    let ret = str::as_c_str(filename, {
      |filename| magic_load(cookie, filename)
    });
    ret == 0
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

  fn file(&self, filename: &str) -> Option<~str> unsafe {
    let cookie = self.cookie;
    let text = str::as_c_str(filename, {
       |filename| magic_file(cookie, filename)
    });

    if is_null(text) {
      None
    } else {
      Some(str::raw::from_c_str(text))
    }
  }

  fn error() -> ~str unsafe {
    str::raw::from_c_str(magic_error(self.cookie))
  }

  fn setflags(&self, flags: int) {
    magic_setflags(self.cookie, flags as c_int);
  }
}

fn open(flags: int) -> Option<Cookie> {
  let cookie = magic_open(flags as c_int);
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
    let cookie = option::unwrap(open(0));
    assert(cookie.load("/usr/share/file/magic"));

    assert(option::unwrap(cookie.file("rust-logo-128x128-blk.png")) == 
           ~"PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");
  }
  #[test]
  fn buffer() {
    let cookie = option::unwrap(open(0));
    assert(cookie.load("/usr/share/file/magic"));

    let s = ~"#!/usr/bin/env python3\nprint('Hello, world!')";
    let text = option::unwrap(str::as_bytes(&s, |bytes| {
      cookie.buffer(*bytes)
    }));
    assert(text == ~"Python script, ASCII text executable");
  }
  #[test]
  fn file_error() {
    let cookie = option::unwrap(open(0));
    assert(cookie.load("/usr/share/file/magic"));

    let ret = cookie.file("non-existent_file.txt");
    assert(ret.is_none());
    assert(cookie.error() == ~"cannot open `non-existent_file.txt' (No such file or directory)");
  }
}
