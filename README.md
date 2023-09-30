
[//]: # (This is the README for the whole project / Git repo)

[//]: # (The crate has a separate README-crate.md )

rust-magic [![build status](https://github.com/robo9k/rust-magic/actions/workflows/build.yml/badge.svg)](https://github.com/robo9k/rust-magic/actions/workflows/linux.yml) [![Documentation](https://docs.rs/magic/badge.svg)](https://docs.rs/magic) [![REUSE status](https://api.reuse.software/badge/github.com/robo9k/rust-magic)](https://api.reuse.software/info/github.com/robo9k/rust-magic) [![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/5709/badge)](https://bestpractices.coreinfrastructure.org/projects/5709) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/robo9k/rust-magic/badge)](https://securityscorecards.dev/viewer/?uri=github.com/robo9k/rust-magic) [![codecov](https://codecov.io/gh/robo9k/rust-magic/graph/badge.svg?token=YnazJQdLXI)](https://codecov.io/gh/robo9k/rust-magic) 
==========

[`libmagic`](https://www.darwinsys.com/file/) bindings for the [Rust programming language](https://www.rust-lang.org/).

`libmagic` recognizes the type of data contained in a file (or buffer) and can give you
a textual description, a MIME type and the usual file extensions.

# Usage

This project's [crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) is
published on the [`crates.io` Rust package registry](https://crates.io/): the [`magic` crate](https://crates.io/crates/magic)

In your Rust project, use [`cargo add`](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html#cargo-add)
to [specify dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html):

```shell
$ cargo add magic
```

To install the latest in-development version instead:
```shell
$ cargo add --git https://github.com/robo9k/rust-magic
```

You might be familiar with `libmagic`'s CLI; `file`:
```shell
$ file data/tests/rust-logo-128x128-blk.png
data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

You can implement something similar in Rust with the `magic` crate, see [crate README](README-crate.md):

```shell
$ cargo run --example file-ish -- data/tests/rust-logo-128x128-blk.png
PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

For more details, check the `magic` [rustdoc](https://doc.rust-lang.org/rustdoc/index.html): [robo9k.github.io/rust-magic/magic](https://robo9k.github.io/rust-magic/magic/index.html)

# Requirements

For the `magic` crate requirements, see [crate README](README-crate.md).

For developing the `rust-magic` project, see [CONTRIBUTING](CONTRIBUTING.md).

# License

This project is licensed under either of
 * Apache License, Version 2.0
 ([LICENSES/Apache-2.0.txt](LICENSES/Apache-2.0.txt) or https://opensource.org/licenses/Apache-2.0)
 * MIT license
 ([LICENSES/MIT.txt](LICENSES/MIT.txt) or https://opensource.org/licenses/MIT)

at your option.

For further details, see [LICENSE](LICENSE.md).

# Security

See [SECURITY](SECURITY.md).

# Contribution

See [CONTRIBUTING](CONTRIBUTING.md).
