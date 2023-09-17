
<div align="center">

## jwt-tui
A terminal user interface for JSON Web Tokens (JWTs) that allows you to decode, verify, and generate JWTs.

[![License][lisence-badge]][license]
[![Crates.io][jwt-tui-crate-badge]][jwt-tui-crate-url]

[![CI][ci-badge]][ci-url]
[![CD][cd-badge]][cd-url]

</div>

## Installation
The program can be installed from source or from [crates.io]. it is recommended to install from crates.io as it is easier and it's a stable release.

> **Note:** `jwt-tui` in the `master` is not stable and is still in development. If you want to use a stable version, use the latest release from [crates.io].

### From Source
The following commands will clone the repository and install the binary to `~/.cargo/bin`, which should be in your `PATH` to run the program from anywhere.

> See [Cargo's documentation](cargo-install) for more information about installing Rust programs.

```bash
git clone https://github.com/theawiteb/jwt-tui.git
cd jwt-tui
cargo install --path .
```

### From Crates.io
```bash
cargo install jwt-tui
```

## Usage
To run the program, simply type its name in the terminal, `jwt-tui`.

```bash
jwt-tui
```
## Showcase
The following are some videos showcasing the program's features.

### Decoding _**WIP**_
[![]()]()

### Encoding data (Generating JWTs) _**WIP**_
[![]()]()

### Verifying JWTs _**WIP**_
[![]()]()


## Goals
The following are the goals of the project:
- Decoding JWTs.
- Encoding data (generating JWTs).
- Verifying JWTs.
- Support for multiple algorithms.


## Changelog

You can find the changelog [here](changelog). It contains information about the changes in each version of the program.

> **Note:** Our changelog follows the [Keep a Changelog](keepachangelog) format.

## Contributing
Contributions are welcome, and they are greatly appreciated! Please see [CONTRIBUTING.md](contributing) for more information.



## License
This project is licensed under the [MIT License](license).

<!-- Links -->

[crates.io]: https://crates.io/crates/jwt-tui
[changelog]: https://github.com/theawiteb/jwt-tui/blob/master/CHANGELOG.md
[license]: https://github.com/theawiteb/jwt-tui/blob/master/LICENSE
[keepachangelog]: https://keepachangelog.com/en/1.0.0/
[cargo-install]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
[contributing]: https://github.com/theawiteb/jwt-tui/blob/master/CONTRIBUTING.md

<!-- Badges -->
[lisence-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[jwt-tui-crate-badge]: https://img.shields.io/crates/v/jwt-tui.svg
[jwt-tui-crate-url]: https://crates.io/crates/jwt-tui
[ci-url]: https://github.com/theawiteb/jwt-tui/actions/workflows/ci.yml
[ci-badge]: https://github.com/theawiteb/jwt-tui/actions/workflows/ci.yml/badge.svg
[cd-url]: https://github.com/theawiteb/jwt-tui/actions/workflows/cd.yml
[cd-badge]: https://github.com/theawiteb/jwt-tui/actions/workflows/cd.yml/badge.svg