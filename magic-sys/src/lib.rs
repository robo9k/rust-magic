extern crate libc;
use libc::{c_char, c_int, size_t};

pub enum Magic {}

impl Copy for Magic {}

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
