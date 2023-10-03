# CONTRIBUTING

Thank you for contributing to the `rust-magic` project, which contains the `magic` Rust crate! ♥️

## Maintenance

Please note that the project is only maintained passively by @robo9k \
This is sometimes also called ["casual maintenance intended"](https://casuallymaintained.tech/).

This means that it might take longer for the maintainers to look at your contributions, e.g.
pull requests that require much code review might not be merged
and issues that require much research/work might remain unresolved.

That being said, the project welcomes your contributions and will try to
prioritize security issues (see [Security](#Security))
and bug reports.

## Discussions

Please use [discussions](https://github.com/robo9k/rust-magic/discussions) to ask and answer questions.

The maintainers will try and answer questions, but you can answer other users' questions, too.

## Issues

Please use issues to create bug reports, suggest features or submit a custom issue.\
The [issue templates](https://github.com/robo9k/rust-magic/issues/new/choose) will guide you towards the expected format.

Please take a look at [existing issues](https://github.com/robo9k/rust-magic/issues) before
creating new ones.

## Pull requests

Please use [pull requests](https://github.com/robo9k/rust-magic/pulls) to propose concrete changes. Before doing so,
use discussions or issues to align your implementation ideas with the project maintainers.

## Code

The project is pretty standard for Rust and GitHub.
It uses `cargo`, `crates.io`, GitHub Actions and GitHub Pages.

Check the [crate README](README-crate.md) for
the Minimum Supported Rust Version and
requirements of the `magic-sys` crate / the `libmagic` C library.\
Note that the `magic-sys` crate is a related but separate sister project, see [robo9k/rust-magic-sys](https://github.com/robo9k/rust-magic-sys).

When developing code, please use `cargo clippy` and `cargo fmt`.

New code should also come with new documentation (`cargo doc`, readme) and tests (`cargo test`, GitHub Actions).\
Changed code should accordingly result in changed documentation and tests.\
There are no hard rules on good tests, test coverage or what makes good documentation,
but try to adhere to the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/documentation.html).
If you are unsure, just ask.

## Commits

Commit messages should follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) and
use the additional types `build:`, `chore:`, `ci:`, `docs:`, `style:`, `refactor:`, `perf:`, `test:`.\
Ideally use breaking change messages according to [Semantic Versioning](https://semver.org/).\
You to not have to provide a changelog entry
(we use the [keep a changelog](https://keepachangelog.com/en/1.1.0/) format)
but it helps if you provide a short summary in your pull request.

Also see [Signoff](#Signoff) as part of the DCO.

There are other conventions/rules, but the general idea is that continous integration will run for your
pull request and fail if something is not in order (`cargo deny`, linear Git history to name a few).\
Don't worry trying to get something perfect on the first try - you can always ask the maintainers for
help and force-push fixes to your pull request branch. We're all no wizards 🧙

## Donations

The project does _not_ accept donations, monetary or otherwise.

## License

This project is dual-licensed under the MIT and the Apache-2.0 licenses, see [LICENSE](LICENSE.md).

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

Also see the [GitHub Terms of Service](https://docs.github.com/en/site-policy/github-terms/github-terms-of-service#6-contributions-under-repository-license).

## Developer Certificate of Origin

As per https://developercertificate.org/ Version 1.1:

```
By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

### Signoff

Please use [`git commit --signoff`](https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---signoff).

## Security

For security related contributions, please follow the policy in [SECURITY](SECURITY.md).
