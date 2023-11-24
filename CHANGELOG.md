# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.16.3](https://github.com/robo9k/rust-magic/compare/v0.16.2...v0.16.3) - 2023-11-24

### Other
- *(deps)* Bump github/codeql-action from 2.22.5 to 2.22.8
- *(deps)* Bump taiki-e/install-action from 2.21.10 to 2.21.18
- *(deps)* Bump step-security/harden-runner from 2.6.0 to 2.6.1
- *(deps)* Bump libc from 0.2.149 to 0.2.150
- *(deps)* Bump taiki-e/install-action from 2.21.7 to 2.21.10
- *(deps)* Bump MarcoIeni/release-plz-action from 0.5.25 to 0.5.26
- *(deps)* Bump actions/create-github-app-token from 1.5.0 to 1.5.1
- *(deps)* Bump github/codeql-action from 2.22.4 to 2.22.5
- *(deps)* Bump taiki-e/install-action from 2.20.17 to 2.21.7
- *(deps)* Bump ossf/scorecard-action from 2.3.0 to 2.3.1
- *(deps)* Bump Swatinem/rust-cache from 2.7.0 to 2.7.1
- *(deps)* Bump taiki-e/install-action from 2.20.11 to 2.20.17
- *(deps)* Bump github/codeql-action from 2.22.3 to 2.22.4
- *(deps)* Bump taiki-e/install-action from 2.20.10 to 2.20.11
- Use regular `release-plz-action` version again
- *(deps)* Bump bitflags from 2.4.0 to 2.4.1
- *(deps)* Use precise version for `bitflags`
- *(deps)* Bump thiserror from 1.0.49 to 1.0.50
- *(deps)* Bump taiki-e/install-action from 2.20.3 to 2.20.10
- *(deps)* Bump actions/checkout from 4.1.0 to 4.1.1
- *(deps)* Bump libc from 0.2.148 to 0.2.149
- *(deps)* Bump github/codeql-action from 2.21.9 to 2.22.3
- *(deps)* Bump actions/create-github-app-token from 1.3.0 to 1.5.0
- *(deps)* Bump taiki-e/install-action from 2.19.4 to 2.20.3
- *(deps)* Bump ossf/scorecard-action from 2.2.0 to 2.3.0
- *(deps)* Bump actions/create-github-app-token from 1.2.2 to 1.3.0
- *(deps)* Bump taiki-e/install-action from 2.19.2 to 2.19.4
- *(deps)* Bump step-security/harden-runner from 2.5.1 to 2.6.0

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

