rust-magic [![Build Status](https://travis-ci.org/robo9k/rust-magic.svg?branch=master)](https://travis-ci.org/robo9k/rust-magic)
==========

[libmagic](http://darwinsys.com/file/) bindings for [Rust](http://www.rust-lang.org/).

---

`libmagic(3)` is the backend of the `file(1)` command, which classifies files, e.g.:

```sh
$ file assets/rust-logo-128x128-blk.png
assets/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

This project provides `libmagic` Rust bindings (NOT the `file` command from the example).
Documentation is [rust-magic](http://rust-ci.org/robo9k/rust-magic/doc/magic/) on Rust CI.
Licensed under the MIT license (see `LICENSE`).


# Usage

Create a new cargo project (or edit your existing one):

```sh
$ cargo new --bin magic-usage && cd magic-usage/
$ $EDITOR Cargo.toml
$ $EDITOR src/main.rs
```

Add a dependency to your `Cargo.toml` (see [cargo doc](http://doc.crates.io/guide.html#adding-dependencies)):

```toml
[dependencies.magic]
version = "0.3.0"
```

Then use the `magic` crate like this (in your `src/main.rs`):

```rust
extern crate magic;
use magic::{Cookie, flags};

fn main() {
    let cookie = Cookie::open(flags::NONE).ok().unwrap();
    cookie.load(&Path::new("/usr/share/misc/magic"));
    println!("It's a kind of magic: {}", cookie.file(&Path::new("assets/rust-logo-128x128-blk.png")).ok().unwrap());
}
```

And for this example, build and run it:

```sh
$ cargo run
     Running `target/magic-usage`
It's a kind of magic: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```
