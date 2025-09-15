# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.3](https://github.com/robo9k/rust-magic/compare/v0.16.2...v0.16.3) - 2025-09-15

### Other

- Link docs to alternative pure Rust crate `file_type`
- *(deps)* Bump `thiserror` from 1.0.49 to 1.0.61
- *(deps)* Bump `libc` from 0.2.148 to 0.2.155
- *(deps)* Bump `bitflags` from 2 to 2.5.0

## [0.16.2](https://github.com/robo9k/rust-magic/compare/v0.16.1...v0.16.2) - 2023-10-05

### Added
- Implement `TryFrom` for a few more database path types
- Allow recovering cookie from state transition errors

## [0.16.1](https://github.com/robo9k/rust-magic/compare/v0.16.0...v0.16.1) - 2023-10-03

### Other
- Add a whole lot of rustdoc

## [0.16.0](https://github.com/robo9k/rust-magic/compare/v0.15.1...v0.16.0) - 2023-09-30

### Changed
- [**breaking**] Move everything `Cookie` into mod `cookie`
- [**breaking**] Use typestate for opened/loaded `Cookie`
- [**breaking**] Introduce `DatabasePaths` and `InvalidDatabasePathError`
- [**breaking**] Introduce `OpenError` for `Cookie::open`
- [**breaking**] Introduce `SetFlagsError` for `Cookie::set_flags`
- [**breaking**] Replace `MagicError` with `cookie::Error`
- [**breaking**] Replace internal `ApiViolation` error for `libmagic` with `panic!`

### Other
- Split project/crate README
- *(deps)* Bump thiserror from 1.0.40 to 1.0.49
- *(deps)* Bump libc from 0.2.141 to 0.2.148

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

