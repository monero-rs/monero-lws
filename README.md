[![Build Status](https://img.shields.io/github/actions/workflow/status/monero-rs/monero-lws/build.yml?branch=main)](https://github.com/monero-rs/monero-lws/actions/workflows/build.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/monero-lws.svg)](https://crates.io/crates/monero-lws)
[![Documentation](https://docs.rs/monero-lws/badge.svg)](https://docs.rs/monero-lws)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.56.1-blue)](https://blog.rust-lang.org/2021/11/01/Rust-1.56.1.html)

# Monero Light Wallet Server RPC Client

A Rust RPC client for the [monero light wallet server](https://github.com/vtnerd/monero-lws).

## Tests

To run tests against monero light wallet server you can use the Docker stack in `tests/`. Use `docker-compose up` and then run `cargo test`.

## Releases and Changelog

See [CHANGELOG.md](CHANGELOG.md) and [RELEASING.md](RELEASING.md).

## License

See [LICENSE](LICENSE).
