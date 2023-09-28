# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/robo9k/rust-magic/compare/v0.15.1...v0.16.0) - 2023-09-28

### Added
- [**breaking**] Replace `errno` with `std::io::Error`

### Fixed
- fix compilation for master
- fix compile for incoming
- fix usage of extern
- fix tests
- fix non-camel-case-types warning

### Other
- Prepare changelog for release
- release
- Refactor workflow names and events
- Fix clippy findings in rustdoc doctests
- Run `cargo-rustdoc-clippy` as part of lint workflow
- Fix clippy findings in tests
- Run tests on nightly with `cargo-careful`
- Configure `release-plz` to add "release" labels
- Configure `release-plz` to use a GitHub App instead of PAT
- Use `mold` linker
- Fix docs workflow SHA for pull requests
- *(deps)* Update `proc-macro2` dependency for nightly builds
- Build docs on pull requests but do not deploy them
- Add `Cargo.lock`
- Run build workflow on push also
- Add codecov badge to readme
- Upload coverage to `codecov.io`
- Report test coverage with `cargo-llvm-cov`
- Fix CI build workflow badge in readme
- Prepare changelog for release
- release
- *(deps)* [**breaking**] Upgrade to `bitflags` v2
- *(deps)* Bump actions/checkout from 4.0.0 to 4.1.0
- Fix Dependabot configuration for conventional commits
- Add Dependabot configuration
- Add `SECURITY.md` policy
- Always run `cargo deny` workflow
- Update build workflow
- Update lint workflow
- Fix docs workflow some more
- Fix docs workflow
- Update docs workflow
- Update CI workflows
- Add `step-security/harden-runner` to all CI steps
- Update OpenSSF scorecard setup
- Enforce conventional commits
- Fix duplicate version in CHANGELOG
- Fix duplicate version in CHANGELOG
- Cleanup old release configuration
- Mention SPDX license identifier
- Backfill empty CHANGELOG for old releases
- Use `release-plz` GitHub Action
- Update crate version
- Add affiliation disclaimer
- Fix MSRV and documentation
- Add use cases section to README
- Update precompiled test magic database
- Update MSRV for dependencies
- Improve documentation
- Add Rust code example to README, runnable `file`-ish example ([#58](https://github.com/robo9k/rust-magic/pull/58))
- Add `magic::libmagic_version()` function
- Remove `magic::version()` function
- Relicense as `MIT OR Apache-2.0`
- Update to idiomatic 2018 edition
- Update crate version
- Update `magic-sys` dependency
- Fix permissions for right workflow job
- Explicitly allow Unicode license
- Update GitHub workflows to use pinned dependencies and tokens with least permissions, refs [#50](https://github.com/robo9k/rust-magic/pull/50)
- Add OpenSSF scorecard badge to README
- Add OpenSSF scorecard (badge)
- Add OpenSSF best practices badge to README
- Fix docs CI workflow
- Add docs CI workflow that publishes rustdoc using GitHub Pages
- Exclude unnecessary license files from packaged crate
- Add copyright text to `LICENSE`
- Add unmodified `LICENSES/MIT.txt` as `LICENSE` file for GitHub
- Add GitHub CI workflow for REUSE compliance
- Make project REUSE-compliant
- Add SPDX copy right and license tags to Rust source files
- Remove impossible error
- Remove unwrap and panic in FFI error handling
- Refactor FFI error API
- Implement FFI wrapper for `magic_open()`
- Implement FFI wrapper for `magic_load_buffers()`
- Implement FFI wrapper for `magic_load()`
- Implement FFI wrapper for `magic_list()`
- Implement FFI wrapper for `magic_compile()`
- Implement FFI wrapper for `magic_check()`
- Implement FFI wrapper for `magic_setflags()`
- Implement FFI wrapper for `magic_buffer()`
- Implement FFI wrapper for `magic_file()`
- Add separate error type for FFI module
- Add private FFI module
- Use `extern crate magic_sys as libmagic` to free up "ffi" name
- Merge pull request [#39](https://github.com/robo9k/rust-magic/pull/39) from robo9k/clippy-msrv
- Add clippy configuration file for MSRV
- Update crate version
- Add more comprehensive crate rustdoc
- Remove `Cookie.error`
- Make `Cookie.set_flags` return `Result`
- Implement more comprehensive errors
- Use multiple doc alias attributes to actually support MSRV 1.48
- Add doc aliases for FFI names
- Bump MSRV to 1.48
- Add `#[must_use]` attribute
- Use different precompiled magic database for test
- Add `Cookie.load_buffers()`
- Implement `std::fmt::Debug` for `Cookie` struct
- Update crate version
- Update `CookieFlags` to `libmagic` v5.38 API
- Remove `CookieFlags::NONE` in favor of `impl Default`
- Use constants from `magic-sys` for `CookieFlags`
- Use alpha version of `magic-sys` dependency
- Update crate version
- Update URLs
- Overhaul `Cargo.toml` crate data
- Update all dependencies to their latest stable versions
- Apply `cargo +stable clippy`
- Apply `cargo +stable fmt`
- Run `cargo clippy` and `cargo fmt` linters for CI
- Add GitHub Actions workflow for `cargo deny`
- Add initial `deny.toml` configuration for `cargo deny`
- Determine and document minimum supported rust version
- Replace Travis with GitHub Actions CI
- Fix expected panic message for unimplemented!()
- Update README for robo9k/rust-magic-sys[#11](https://github.com/robo9k/rust-magic/pull/11)
- Bump version: 0.12.1 → 0.12.2
- Add rustdoc for Cookie
- Improve example code
- Add short rustdoc to MagicError and Cookie
- Add rustdoc for Default impl
- Use descriptions from `man libmagic` for CookieFlags
- Explicitly implement Default for CookieFlags using `NONE`
- Fix clippy identity_op warning
- Bump version: 0.12.0 → 0.12.1
- Use docs.rs to host rustdoc instead of Github Pages
- Point repository in Cargo.toml at 'webview'
- Remove homepage from Cargo.toml
- Add Travis CI badge to Cargo.toml
- Add categories to Cargo.toml
- Bump version: 0.11.0 → 0.12.0
- Fix Path to * c_char conversation once again
- Bump version: 0.10.0 → 0.11.0
- Update dependencies
- Use container-based infrastructure and caching on Travis
- Bump version: 0.9.0 → 0.10.0
- Mention required rustc version in README, fixes [#15](https://github.com/robo9k/rust-magic/pull/15)
- Run Travis for rust nightly and stable
- Update authors/contributors
- Remove feature(convert) which was required pre [#16](https://github.com/robo9k/rust-magic/pull/16)
- Use CString::new instead of to_cstring()
- Bump version: 0.8.0 → 0.9.0
- Use `AsRef<Path>` wherever a `Path` was previously used.
- Bump version: 0.7.0 → 0.8.0
- Use nightly Rust for Travis, refs [#13](https://github.com/robo9k/rust-magic/pull/13)
- Update tests, refs [#13](https://github.com/robo9k/rust-magic/pull/13)
- Update dependencies, refs [#13](https://github.com/robo9k/rust-magic/pull/13)
- Remove stability attributes, refs [#13](https://github.com/robo9k/rust-magic/pull/13)
- Update FFI C string code, refs [#13](https://github.com/robo9k/rust-magic/pull/13)
- Update magic_sys dependency, refs [#13](https://github.com/robo9k/rust-magic/pull/13)
- Cleanup feature gates
- Update regex dependency
- Do use `sudo -H` for `pip install`
- Do not use sudo for `pip install`
- Fix copy & paste error
- Update to work with `Path` RFC
- Bump version: 0.6.4 → 0.7.0
- Replace `experimental` with `unstable` attributes
- Silence unstable feature warnings
- Derive `Debug` instead of `Show`
- Update regex dependency
- Unclutter README a bit
- Remove duplicate content from README
- Add rustdoc on crate level
- Fix dependencies version snippet in README
- Fix path to test PNG file in README examples
- Clarify usage of test-data from `file`
- Add links to LICENSE files in README
- Improve `version()` implementation
- Bump version: 0.6.3 → 0.6.4
- Add a .bumpversion.cfg
- Add `impl Display for MagicError`
- Update links to rustdoc
- Use GitHub Pages for rustdoc
- Use bitflags! from crates.io
- Move `extern crate regex` into test module
- Move `regex` to `dev-dependencies`
- Use `regex` library from crates.io
- Bump version
- Fixes for RFC 494
- Bump version
- Add `use std::c_str::ToCStr;` after prelude stabilization
- Replace renamed `deriving` with `derive`
- Fix crates.io dependencies example formatting in README
- Bump version to publish on crates.io
- Disable test for unimplemented functionality
- Add TODO notes regarding functions using multiple database files
- Fix Cargo.toml format for crates.io dependencies
- Use `&[Path]` for all functions with multiple database files, fixes [#9](https://github.com/robo9k/rust-magic/pull/9)
- Use a `&[Path]` for `Cookie::load()`, fixes [#5](https://github.com/robo9k/rust-magic/pull/5)
- Add comment about relative paths to test files
- Improve `version()` implementation and test
- Bump version to publish crate
- Use custom magic databases for tests, fixes [#7](https://github.com/robo9k/rust-magic/pull/7)
- Fix cargo packaging excludes, move test data files, fixes [#3](https://github.com/robo9k/rust-magic/pull/3)
- Add `impl Error for MagicError`
- Add link to crates.io package to README
- Add a `version()` function to return crate version, refs [#4](https://github.com/robo9k/rust-magic/pull/4)
- Add MIT license and mention it in README, fixes [#1](https://github.com/robo9k/rust-magic/pull/1)
- Split `magic-sys` into its own repository, fixes [#2](https://github.com/robo9k/rust-magic/pull/2)
- Add a `Usage` section to README
- Specify `license`, fix `repository`, extend `authors`, bump `version` in cargo metadata
- Add  and  to cargo metadata, bump version of
- Add required metadata to publish `magic-sys` to crates.io
- Add .gitignore for magic-sys
- Add and use magic-sys package
- Replace deprecated string::raw::from_buf() with String::from_raw_buf
- Add package metadata and bump version
- Return a desc String in MagicError
- Change Cookie::open() to return Result
- Change Cookie::file() and :buffer() to return Result
- Rename fn to Cookie::set_flags() and return bool
- Change remaining Cookie database functions to return Result
- Change Cookie::load() to return Result
- Fix indentation
- Add Cookie::load_default()
- Add explanation of libmagic and link to documentation to README
- Add stability attributes
- Fix s/static/const in bitflags! macro usage
- Move FFI code into its own module
- Use self:: prefix when referring to flags module
- Drop MAGIC_ prefix from CookieFlags
- Move CookieFlags into their own flags module
- Rename bitflags to more specific `CookieFlags`
- Remove version() -> magic_version(), as it requires libmagic >= 5.13
- Rename src/magic.rs to src/lib.rs following Cargo conventions
- Trim down .gitignore no  defaults
- Copy doc from magic.h verbatim for MAGIC_* flags
- Make all Travis scripts verbose
- Add fn version() -> magic_version()
- Fix using tabs instead of spaces for Flags
- Use bitflags! macro instead of self-made MagicFlags struct
- Fix MIME comparison string in buffer() test
- Fix comparison string in buffer() test
- apt-get libmagic-dev for Travis build
- apt-get libmagic1 for Travis build
- Fix build to work with Travis language:rust instead of rust/cargo PPAs
- Fix RUSTCI_TOKEN
- Extend Travis config to run tests and upload doc to Rust CI
- Add Travis badge and links to README
- Revert r2d6bc04 for Travis
- Add initial README.md
- Add Travis CI config
- Fix unit tests after r9935524
- Replace str::raw::from_c_str() with string::raw::from_buf()
- Replace [[lib]] with [lib] to fix Cargo warning
- Remove bare raw pointers
- Bytes to 'b'
- Added Cargo file and compiles again.
- use `Path` for file names
- replace MagicFlag enum with a struct
- make the impl public
- fail_unless! -> assert!
- remove static from static method
- port to rustpkg
- update to new impl syntax
- port to the current rust master
- use a single .rc file
- more cleanup
- use 4 space indents
- rm extra braces
- extern mod -> extern "C" mod
- clean up use statements
- c_void is unused
- switch from deprecated drop syntax to Drop trait
- cleanup
- replace option::unwrap with the unwrap method
- make open a static method
- use explicit self everywhere
- switch export -> pub
- add MagicFlag enum
- make error method safe
- cleanup
- add check, compile and list methods
- add setflags method
- add an error method
- initial commit

## [0.15.1](https://github.com/robo9k/rust-magic/compare/v0.15.0...v0.15.1) - 2023-09-28

### Other
- Fix clippy findings in rustdoc doctests
- Fix clippy findings in tests

## [0.15.0](https://github.com/robo9k/rust-magic/compare/v0.14.0...v0.15.0) - 2023-09-26

### Changed
- *(deps)* [**breaking**] Replace `errno` with `std::io::Error`
- *(deps)* [**breaking**] Upgrade to `bitflags` v2

## 0.14.0 - 2023-09-16

TBD

## 0.13.0 - 2022-08-18

TBD

## 0.13.0-alpha.3 - 2021-11-07

TBD

## v0.13.0-alpha.2 - 2021-11-06

TBD

## 0.13.0-alpha.1 - 2021-10-26

TBD

## 0.12.2 - 2017-04-12

TBD

## 0.12.1 - 2017-04-11

TBD

## 0.12.0 - 2016-10-18

TBD

## 0.11.0 - 2016-07-01

TBD

## 0.10.0 - 2016-06-11

TBD

## 0.9.0 - 2016-01-30

TBD

## 0.8.0 - 2015-07-11

TBD

## 0.7.0 - 2015-02-03

TBD

## 0.6.4 - 2015-01-24

TBD

## 0.6.3 - 2015-01-19

TBD

## 0.6.2 - 2015-01-09

TBD

## 0.6.1 - 2015-01-05

TBD

## 0.6.0 - 2014-12-22

TBD

## 0.5.2 - 2014-12-21

TBD

## 0.5.1 - 2014-12-21

TBD

## 0.5.0 - 2014-12-21

TBD

## 0.4.0 - 2014-12-21

TBD

## 0.3.0 - 2014-12-19

TBD

## 0.2.0 - 2014-11-20

TBD

## 0.1.0 - 2014-11-20

TBD

