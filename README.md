rust-magic [![Build Status](https://travis-ci.org/robo9k/rust-magic.svg?branch=master)](https://travis-ci.org/robo9k/rust-magic)
==========
[libmagic](http://darwinsys.com/file/) bindings for [Rust](http://www.rust-lang.org/).


# Usage

Create a new Cargo project (or edit your existing one):

```sh
$ cargo new --bin magic-usage && cd magic-usage/
$ $EDITOR Cargo.toml
```

Add a dependency to your `Cargo.toml` (see [Cargo doc](http://doc.crates.io/guide.html#adding-dependencies)):

```toml
[dependencies]
magic = "0.*"
```

Then use the [`magic` crate](https://crates.io/crates/magic) according to [its documentation](https://robo9k.github.io/rust-magic/magic/#usage-example).


# License

This project is licensed under the MIT license (see [`LICENSE`](https://github.com/robo9k/rust-magic/blob/master/LICENSE)).

The `magic-sys` crate being used is licensed under the MIT license as well (see [`LICENSE`](https://github.com/robo9k/rust-magic-sys/blob/master/LICENSE)).

The `file`/`libmagic` project is licensed under a modified BSD license (see [`COPYING`](https://github.com/file/file/blob/master/COPYING)).
This crate contains partial test-data from its magic databases (`rust-magic/data/tests/db-images-png` is from `file/magic/Magdir/images`, `rust-magic/data/tests/db-python` is from `file/magic/Magdir/python`).
