//! Internal Foreign Function Interface module for `magic_sys` / `libmagic`
//!
//! Contains `unsafe` as a medium level binding.

#![allow(unsafe_code)]

extern crate magic_sys as libmagic;

pub(crate) fn close(cookie: self::libmagic::magic_t) {
    unsafe { self::libmagic::magic_close(cookie) }
}
