rust-magic [![linux build status](https://github.com/robo9k/rust-magic/actions/workflows/linux.yml/badge.svg)](https://github.com/robo9k/rust-magic/actions/workflows/linux.yml) [![Documentation](https://docs.rs/magic/badge.svg)](https://docs.rs/magic)
==========
[libmagic](https://www.darwinsys.com/file/) bindings for [Rust](https://www.rust-lang.org/).


# Usage

Create a new Cargo project (or edit your existing one):

```sh
$ cargo new --bin magic-usage && cd magic-usage/
$ $EDITOR Cargo.toml
```

Add a dependency to your `Cargo.toml` (see [Cargo doc](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-cratesio)):

```toml
[dependencies]
magic = "0.13"
```

Then use the [`magic` crate](https://crates.io/crates/magic) according to [its documentation](https://docs.rs/magic/#usage-example).

# MSRV

The Minimum Supported Rust Version (MSRV) is Rust 1.48 or higher.

This version might be changed in the future, but it will be done with a crate version bump.

# Requirements

By default compiling `rust-magic` will search your system library paths for a version of `libmagic.so`. If you're cross-compiling, or need more control over which library is selected, see [how to build `rust-magic-sys`](https://github.com/robo9k/rust-magic-sys#building).

# License

This project is licensed under the MIT license (see [`LICENSE`](https://github.com/robo9k/rust-magic/blob/master/LICENSE)).

The `magic-sys` crate being used is licensed under the MIT license as well (see [`LICENSE-MIT`](https://github.com/robo9k/rust-magic-sys/blob/main/LICENSE-MIT)).

The `file`/`libmagic` project is licensed under a modified BSD license (see [`COPYING`](https://github.com/file/file/blob/master/COPYING)).
This crate contains partial test-data from its magic databases (`rust-magic/data/tests/db-images-png` is from `file/magic/Magdir/images`, `rust-magic/data/tests/db-python` is from `file/magic/Magdir/python`).
