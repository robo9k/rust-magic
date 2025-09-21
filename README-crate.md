
[//]: # (This is the README for the `magic` crate only)

[//]: # (The whole project has docs in https://github.com/robo9k/rust-magic )

High-level bindings for `libmagic`

# About

This crate provides bindings for the [`libmagic` C library]((https://www.darwinsys.com/file/)),
which recognizes the type of data contained in a file (or buffer) and can give you
a textual description, a MIME type and the usual file extensions.

# Usage

```rust
// only for Rust Edition 2018, see https://doc.rust-lang.org/edition-guide/rust-2021/prelude.html
use std::convert::TryInto;

fn file_example() -> Result<(), Box<dyn std::error::Error>> {
    // Open a new configuration with flags
    let cookie = magic::Cookie::open(magic::cookie::Flags::ERROR)?;

    // Load a specific database
    // (so exact test text assertion below works regardless of the system's default database version)
    let database = ["data/tests/db-images-png"].try_into()?;
    // You can instead load the default database
    //let database = Default::default();

    let cookie = cookie.load(&database)?;

    let file = "data/tests/rust-logo-128x128-blk.png";

    // Analyze the file
    assert_eq!(cookie.file(file)?, "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

    Ok(())
}
```

Check the [crate rustdoc](https://docs.rs/magic) for more details.

# Repository

The project's repository is [github.com/robo9k/rust-magic](https://github.com/robo9k/rust-magic)

It contains the latest in-development version of the `magic` crate (might not be published to `crates.io` yet),  
more [examples](https://github.com/robo9k/rust-magic/tree/main/examples) how to use the `magic` crate  
as well as [issues](https://github.com/robo9k/rust-magic/issues)
and [discussions](https://github.com/robo9k/rust-magic/discussions).

# MSRV

The Minimum Supported Rust Version (MSRV) is Rust 1.64 or higher.

This version might be changed in the future, but it will be done with a crate version bump.

# Requirements

By default, compiling the `magic` crate will (via the [`magic-sys` crate](https://crates.io/crates/magic-sys))
search your system library paths for a shared library version of `libmagic` to link against.  
For this to work, you need to install the development version of `libmagic` in a standard location:
```shell
$ # On Debian based Linux systems:
$ sudo apt-get install libmagic1 libmagic-dev

$ # On macOS:
$ brew install libmagic

$ # On Windows:
$ cargo install cargo-vcpkg
$ cargo vcpkg build
```

If you're cross-compiling, or need more control over which library is selected,
see [how to build `magic-sys`](https://crates.io/crates/magic-sys#building).
