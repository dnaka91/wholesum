<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

## [0.2.4] - 2022-08-03

### Changed

- Updated all dependencies.
- Added crate categories and keywords for crates.io.

## [0.2.3] - 2022-05-26

### Changed

- Update dependencies to their latest versions.

## [0.2.2] - 2022-01-21

### Changed

- Create and sign checksums for release artifacts.

## [0.2.1] - 2022-01-21

### Changed

- Strip the pre-compiled binaries for ARM architecture.

### Fixed

- Adjust the `include` setting of `Cargo.toml` to generate a proper package for <https://crates.io>.

## [0.2.0] - 2022-01-20

### Added

- New subcommand to verify hash files, like `checksum.sha256` but with a slightly different format
  that prefixes the hash algorithm to each entry.

### Changed

- Switch from `structopt` to `clap` (3.0).
- Update all dependencies to the latest version for improved performance and security.

## [0.1.0]

### Added

- Initial release.

[Unreleased]: https://github.com/dnaka91/wholesum/compare/v0.2.4...HEAD
[0.2.4]: https://github.com/dnaka91/wholesum/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/dnaka91/wholesum/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/dnaka91/wholesum/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/dnaka91/wholesum/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/dnaka91/wholesum/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dnaka91/wholesum/releases/tag/v0.1.0
