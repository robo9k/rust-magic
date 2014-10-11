rust-magic [![Build Status](https://travis-ci.org/robo9k/rust-magic.svg?branch=master)](https://travis-ci.org/robo9k/rust-magic)
==========

[libmagic](http://darwinsys.com/file/) bindings for [Rust](http://www.rust-lang.org/).

---

`libmagic(3)` is the backend of the `file(1)` command, which classifies files, e.g.:

```sh
$ file assets/rust-logo-128x128-blk.png
assets/rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
```

This project provides `libmagic` Rust bindings (NOT the `file` command from the example). Documentation is [rust-magic](http://rust-ci.org/robo9k/rust-magic/doc/magic/) on Rust CI and `$ man 3 libmagic`.
