# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- having `#use no_prelude;` at the start of a quote_use invocation disables the prelude
- added `quote_use_no_prelude` variants for all `quote_use` macros

### Removed
- removed `namespace_idents` feature, just use `__variable_name` instead
- removed `prelude_*` features, by default all preludes are included now.

## [0.7.2] - 2023-09-03
### Fixed
- `syn` and `quote` crate to be present for working

## [0.7.1] - 2023-05-21
- Removed `proc-macro-error` dependency

## [0.7.0]
- **Breaking Change**: `$'lifetime` is supported with `namespace_idents` feature
- Updated `syn` to version 2

## [0.6.0]
### Changed
- Reduced required features of `syn`
- **Breaking Change**: `# use #var path::Value` is no longer supported, use `# use #var::path::Value` instead

[unreleased]: https://github.com/ModProg/quote-use/compare/v0.7.2...HEAD
[0.7.2]: https://github.com/ModProg/quote-use/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/ModProg/quote-use/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/ModProg/quote-use/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/ModProg/quote-use/compare/v0.5.1...v0.6.0
