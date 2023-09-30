rust-magic [![build status](https://github.com/robo9k/rust-magic/actions/workflows/build.yml/badge.svg)](https://github.com/robo9k/rust-magic/actions/workflows/linux.yml) [![Documentation](https://docs.rs/magic/badge.svg)](https://docs.rs/magic) [![REUSE status](https://api.reuse.software/badge/github.com/robo9k/rust-magic)](https://api.reuse.software/info/github.com/robo9k/rust-magic) [![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/5709/badge)](https://bestpractices.coreinfrastructure.org/projects/5709) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/robo9k/rust-magic/badge)](https://securityscorecards.dev/viewer/?uri=github.com/robo9k/rust-magic) [![codecov](https://codecov.io/gh/robo9k/rust-magic/graph/badge.svg?token=YnazJQdLXI)](https://codecov.io/gh/robo9k/rust-magic) 
==========
[libmagic](https://www.darwinsys.com/file/) bindings for [Rust](https://www.rust-lang.org/).


# Usage

This [`magic` crate](https://crates.io/crates/magic) is published on the `crates.io` Rust package registry.

Use [`cargo add`](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html#cargo-add) to [specify dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html):

```shell
$ cargo add magic
```

You might be familiar with `libmagic`'s CLI; `file`:
```shell
$ file data/tests/rust-logo-128x128-blk.png
data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

You can implement something similar in Rust with the `magic` crate (see [examples/file-ish.rs](examples/file-ish.rs)):
```rust
// only for Rust Edition 2018, see https://doc.rust-lang.org/edition-guide/rust-2021/prelude.html
use std::convert::TryInto;

fn file_example() -> Result<(), Box<dyn std::error::Error>> {
    // Open a new configuration with flags
    let cookie = magic::Cookie::open(magic::CookieFlags::ERROR)?;

    // Load a specific database
    // (so exact test text assertion below works regardless of the system's default database version)
    let database = &["data/tests/db-images-png"].try_into()?;
    // You can instead load the default database
    //let database = &Default::default();

    let cookie = cookie.load(database)?;

    let file = "data/tests/rust-logo-128x128-blk.png";

    // Analyze the file
    assert_eq!(cookie.file(file)?, "PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced");

    Ok(())
}
```
```shell
$ cargo run --example file-ish -- data/tests/rust-logo-128x128-blk.png
PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

Read the [`magic` rustdoc](https://docs.rs/magic/#usage-example) for further examples and info.

# MSRV

The Minimum Supported Rust Version (MSRV) is Rust 1.56 or higher.

This version might be changed in the future, but it will be done with a crate version bump.

# Requirements

By default, compiling the `magic` crate will search your system library paths for a shared library version of `libmagic` to link against. For this to work, you need to install the development version of `libmagic` in a standard location:
```shell
$ # On Debian based Linux systems:
$ sudo apt-get install libmagic1 libmagic-dev

$ # On MacOs:
$ brew install libmagic

$ # On Windows:
$ cargo install cargo-vcpkg
$ cargo vcpkg build
```

If you're cross-compiling, or need more control over which library is selected, see [how to build `magic-sys`](https://github.com/robo9k/rust-magic-sys#building).

# License

This project is licensed under either of
 * Apache License, Version 2.0
 ([LICENSES/Apache-2.0.txt](LICENSES/Apache-2.0.txt) or https://opensource.org/licenses/Apache-2.0)
 * MIT license
 ([LICENSES/MIT.txt](LICENSES/MIT.txt) or https://opensource.org/licenses/MIT)

at your option.

For further details, see [LICENSE.md](LICENSE.md).

# Security

See [SECURITY.md](SECURITY.md).

# Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).
