# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.0](https://github.com/robo9k/rust-magic/compare/v0.15.1...v0.16.0) - 2023-09-30

### Added
- [**breaking**] Introduce `DatabasePaths` for valid filename inputs
- [**breaking**] Use typestate for opened/loaded cookies
- Improve new error display output
- [**breaking**] Replace `MagicError` error `CookieError` and `CookieDatabaseError`
- [**breaking**] Replace internal `ApiViolation` error for `libmagic` with `panic!`
- [**breaking**] Introduce `magic::CookieSetFlagsError`
- [**breaking**] Introduce `magic::CookieOpenError`

### Other
- Split project/crate README
- [**breaking**] Move everything `Cookie` into mod `cookie`
- *(deps)* Bump thiserror from 1.0.40 to 1.0.49
- *(deps)* Bump libc from 0.2.141 to 0.2.148
- *(deps)* Bump taiki-e/install-action from 2.18.16 to 2.19.1
- *(deps)* Bump github/codeql-action from 2.21.8 to 2.21.9
- *(deps)* Bump EmbarkStudios/cargo-deny-action from 1.5.4 to 1.5.5

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

