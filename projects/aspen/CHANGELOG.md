# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.10 - 2026-07-29
- Added keyword flag to aspen fmt
- Added automatic release builds.

## [1.0.0] - 2026-07-28

### Added
- Initial CHANGELOG.md for project tracking

### Changed
- Moved from standalone repository into the vb6 monorepo (`projects/aspen`)
- Updated to `vb6parse` v1.1.0 via workspace dependency
- Inherited workspace metadata (edition 2024, license MIT, repository)

### Stable
- `check` subcommand is now stable — recursively finds `.vbp` project files and
  validates them, including forms, modules, classes, sub-project references, and
  non-English file detection.

## [0.4.1] - 2024

### Fixed
- Various bug fixes and improvements

## [0.4.0] - 2024

### Added
- Initial release of Aspen, a VB6 analysis tool in the spirit of `cargo check`
- `check` subcommand for validating VB6 project files
- Recursive project discovery in directories
- Support for checking forms, modules, classes, and references
- Sub-project reference validation
- Non-English file detection
- Parallel processing support via rayon
