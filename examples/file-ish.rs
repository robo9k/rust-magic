// SPDX-FileCopyrightText: © The `magic` Rust crate authors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Minimalist `file(1)` clone
//!
//! Prints the `libmagic` description of the file given as a command line argument:
//! ```shell
//! $ file data/tests/rust-logo-128x128-blk.png
//! data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//!
//! $ cargo run --example file-ish -- data/tests/rust-logo-128x128-blk.png
//! PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//! ```

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // open a new configuration with flags
    let cookie = magic::Cookie::open(magic::cookie::Flags::ERROR)?;

    // load the system's default database
    let database = &Default::default();
    let cookie = cookie.load(database)?;

    let file = std::env::args_os()
        .nth(1)
        .expect("One command line argument");

    // analyze the file
    println!("{}", cookie.file(file)?);

    Ok(())
}
