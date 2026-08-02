# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


### [0.1.1]
- Updated to latest version of vb6parse which better handles malformed top level elements.

## [0.1.0] - 2026-08-01

### Added
- Initial public release of `vb6format` as a standalone formatting crate.
- Public formatting API with `fmt_source` and `fmt_cst` entry points.
- `FmtSettings` configuration with support for:
  - `indent_size`
  - `keyword_case` (`upper`, `lower`, `camel`, `first`)
  - `blank_lines_around_directives`
  - `blank_lines_inside_directives`
  - `blank_lines_around_top_level`
- CST-based formatter pipeline built from rewrite and pass stages.
- Formatting passes for comments, continuations, control flow, compiler directives,
  blank-line deduplication, indentation, keyword casing, line endings,
  single-line `If`, top-level spacing, and `Type`/`Enum` blocks.
- Test coverage for directive spacing, top-level spacing behavior, keyword casing,
  line continuations, and control-flow indentation.

### Changed
- Replaced line-by-line post-processing with CST-driven formatting passes.
- Reorganized formatter internals into dedicated modules and context-based flow to
  simplify adding new rewrite rules.
- Updated crate metadata and documentation for standalone crate usage.

### Fixed
- Corrected top-level spacing behavior around declarations such as `Option Explicit`.
- Removed invalid nested procedure test coverage that does not reflect VB6 syntax.