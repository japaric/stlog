# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.2] - 2018-09-23

### Changed

- Reduced the number of symbols required to keep track of the log levels. This
  is technically a breaking change but versions v0.3.0 and v0.3.1 of stlog and
  versions v0.2.0 and v0.2.1 have been yanked.

### Fixed

- Fixed the expansion of the `warn!` macro when used with the global logger.

## [v0.3.1] - 2018-09-23 - YANKED

### Added

- Add documentation to the `stlog-macros` crate.

### Changed

- The documentation link in the README and in the crate metadata

- Render README on crates.io

## [v0.3.0] - 2018-09-23 - YANKED

### Added

- A `GlobalLog` trait has been added. It's the interface of a global logger.

- A `global_logger` attribute has been added. It's used to declare the global
  logger.

- Several Cargo features have been added. These can be used to select a
  maximum allowed log level.

### Changed

- [breaking-change] The `Logger` trait has been renamed to `Log`.

- [breaking-change] The signature of the `Log::log` method has changed; it now
  takes `&mut self`.

- This crate now compiles on 1.30-beta.

### Removed

- [breaking-change] The `set_global_logger!` macro has been removed in favor of
  the `global_logger` attribute.

- [breaking-change] The static methods of the `Log` trait that were used to
  individually enable / disable log levels has been removed in favor of Cargo
  features.

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

[Unreleased]: https://github.com/japaric/stlog/compare/v0.3.2...HEAD
[v0.3.2]: https://github.com/japaric/stlog/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/japaric/stlog/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/stlog/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/stlog/compare/v0.1.0...v0.2.0
