# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) as described in [The Cargo Book](https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field).

## [Unreleased]

## [0.2.0] - 2026-07-19

### Added

- Added support for LWS version 0.3 ([#35](https://github.com/monero-rs/monero-lws/pull/35)).
- Added builder for `LwsRpcClient` ([#38](https://github.com/monero-rs/monero-lws/pull/38)).

### Changed

- Changed MSRV to 1.85, rust edition to 2024.
- Updated `reqwest` to version `0.12`.
- Updated `monero` to version `0.22`.

### Removed

- Removed `LwsRpcClient::new` method (replaced by the builder).

## [0.1.0] - 2022-07-26

### Added

- `LwsRpcClient` able to `login` to a Monero Light Wallet Server and execute commands.

[unreleased]: https://github.com/monero-rs/monero-lws/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/monero-rs/monero-lws/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/monero-rs/monero-lws/compare/8e11482b10b7d1e3c153330f848be59bcca0bdf4...v0.1.0
