LICENSE
=======


This project is licensed under either of
 * Apache License, Version 2.0
 ([LICENSES/Apache-2.0.txt](LICENSES/Apache-2.0.txt) or https://opensource.org/licenses/Apache-2.0)
 * MIT license
 ([LICENSES/MIT.txt](LICENSES/MIT.txt) or https://opensource.org/licenses/MIT)

at your option.  
The machine-readable version of this is `SPDX-License-Identifier: Apache-2.0 OR MIT`.


REUSE [![REUSE status](https://api.reuse.software/badge/github.com/robo9k/rust-magic)](https://api.reuse.software/info/github.com/robo9k/rust-magic)
-----

This project is compliant with the [REUSE guidelines](https://reuse.software/) for licensing software and other files.


Other
-----

The following is a human-readable version of parts of [.reuse/dep5](.reuse/dep5).

The `file` / `libmagic` project is licensed under a modified BSD license (see their [`COPYING`](https://github.com/file/file/blob/master/COPYING) file).

This project contains partial test-data from the `file` project:
- [`rust-magic/data/tests/db-images-png`](data/tests/db-images-png) is from [`file/magic/Magdir/images`](https://github.com/file/file/blob/master/magic/Magdir/images)
- [`rust-magic/data/tests/db-python`](data/tests/db-python) is from [`file/magic/Magdir/python`](https://github.com/file/file/blob/master/magic/Magdir/python)

This project contains test-data from the Rust Foundation:
- [rust-magic/data/tests/rust-logo-128x128-blk.png](data/tests/rust-logo-128x128-blk.png) see [#12](https://github.com/robo9k/rust-magic/issues/12)

If you only use this project in form of the [`magic` crate](https://crates.io/crates/magic) you can ignore those other details since the binary `.crate` distribution does not include the aforementioned test-data files.


Contribution
------------

See [CONTRIBUTING.md](CONTRIBUTING.md).
