# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2017-07-06

### Added

- Support for global logging: a `set_global_logger!` macro was added, and the
  `$logger` argument is now optional in all the logging macros.

- Support for individually disabling log levels: the `Logger` trait gained
  several `*_enabled` methods.

### Changed

- [breaking-change] The `Logger::log` method now takes `self` by reference.

## v0.1.0 - 2017-06-03

- Initial release

[Unreleased]: https://github.com/japaric/stlog/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/stlog/compare/v0.1.0...v0.2.0
