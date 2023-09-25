# Security policy

If you have found a potential security vulnerability in the [`magic` Rust crate](https://crates.io/crates/magic) or elsewhere in this [`robo9k/rust-magic`](https://github.com/robo9k/rust-magic) repository, please follow coordinated disclosure:
- do report it _privately_ on GitHub, i.e. click "Report a vulnerability" on the [repo's "Security" tab](https://github.com/robo9k/rust-magic/security)
- do *NOT* report it _publically_, e.g. do not post it in the public issues or discussions of the project, not on social media nor your own blog just yet

GitHub has some general info about [privately reporting a security vulnerability](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability), which does apply to this `robo9k/rust-magic` repo.

Note that the whole project is only passively-maintained, so fast response times can not be guaranteed and instead are made on best-effort. Use your own judgement if you consider the project to be entirely unresponsive.

If your report is deemed valid (i.e. not "working as intended", a regular bug, or a feature request instead), a public GitHub Security Advisory (GHSA) will be created and you will be credited for the finding. You can also privately collaborate on GitHub for development of a fix. After the project's GHSA publication, feel free to post your own cross-referenced advisory on social media or your own blog. There will be no other embargoed notifications.

There are no security bug bounties or other forms of rewards other than the attribution of the GitHub advisory.

No other forms of reporting security issues are provided (e.g. no email contact).

## Supported versions

Only the latest semver compatible version of the `magic` crate is supported. Note that [`cargo` treats `v0.x` development versions differently](https://doc.rust-lang.org/cargo/reference/semver.html#change-categories).
Other versions will not recieve security upates. All affected versions will be yanked.

## Reports

First of all, ensure that the security issue is in fact in this project. Note that the project depends on the [`magic-sys` Rust crate](https://crates.io/crates/magic-sys) (less likely to contain security issues) and the [`libmagic` C library](https://www.darwinsys.com/file/) (more likely to contain security issues).  
This project itself might also have non-code security issues, such as in its supply-chain (dependencies, continous integration etc.).

Like regular bug reports, please include as much information as possible. For example
- which [versions of the `magic` crate](https://crates.io/crates/magic/versions) are affected?
- which revisions of the [`robo9k/rust-magic`](https://github.com/robo9k/rust-magic) repo are affected?
- in which lines of code, configuration or documentation is the vulnerability located?
- how specifically does this vulnerability create a security risk?
- if we leave the code as it is, what could an attacker ultimately do?
- how can we replicate the issue?
do you have a working proof-of-concept that you would be willing to show us?
- what would we have to do to fix the issue?

Thank you for your contribution.
