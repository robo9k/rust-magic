rust-magic [![Build Status](https://travis-ci.org/robo9k/rust-magic.svg?branch=master)](https://travis-ci.org/robo9k/rust-magic)
==========

[libmagic](http://darwinsys.com/file/) bindings for [Rust](http://www.rust-lang.org/).

---

`libmagic(3)` is the backend of the `file(1)` command, which classifies files, e.g.:

```sh
$ file data/tests/rust-logo-128x128-blk.png
data/tests/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

This project provides `libmagic` Rust bindings (NOT the `file` command from the example).
Documentation is [rust-magic](https://robo9k.github.io/rust-magic/magic/) on GitHub Pages.


# Usage

Create a new cargo project (or edit your existing one):

```sh
$ cargo new --bin magic-usage && cd magic-usage/
$ $EDITOR Cargo.toml
$ $EDITOR src/main.rs
```

Add a dependency to your `Cargo.toml` (see [cargo doc](http://doc.crates.io/guide.html#adding-dependencies)):

```toml
[dependencies]
magic = "0.6.4"
```

Then use the [`magic` crate](https://crates.io/crates/magic) like this (in your `src/main.rs`):

```rust
extern crate magic;
use magic::{Cookie, flags};

fn main() {
    let cookie = Cookie::open(flags::NONE).ok().unwrap();
    cookie.load(&Path::new("/usr/share/misc/magic"));
    println!("It's a kind of magic: {}", cookie.file(&Path::new("data/tests/rust-logo-128x128-blk.png")).ok().unwrap());
}
```

And for this example, build and run it:

```sh
$ cargo run
     Running `target/magic-usage`
It's a kind of magic: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

# License

This project is licensed under the MIT license (see [`LICENSE`](https://github.com/robo9k/rust-magic/blob/master/LICENSE)).

The `magic-sys` crate being used is licensed under the MIT license as well (see [`LICENSE`](https://github.com/robo9k/rust-magic-sys/blob/master/LICENSE)).

The `file`/`libmagic` project is licensed under a modified BSD license (see [`COPYING`](https://github.com/file/file/blob/master/COPYING)).
This crate contains partial test-data from its magic databases (`rust-magic/data/tests/db-images-png` is from `file/magic/Magdir/images`, `rust-magic/data/tests/db-python` is from `file/magic/Magdir/python`).
