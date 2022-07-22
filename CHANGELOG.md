# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased: 0.2.0]

### Added
- The following files: `CHANGELOG.md`, `README.md`, and `rustfmt.toml`.
- `Cargo.toml`: *readme* and *repository* metadata.
- `Cargo.toml`: added `regex`, `lazy_static`, and `log` dependencies.
- All objects as defined in *ISO 32000-1:2008*, 7 "Syntax", 7.3 "Objects".
- Utilities: `margins`, `position`, and `mm` (base unit).
- Filters: `ASCII85Decode` and `ASCIIHexDecode` encoders.
- Repository icon.
- GitHub and GitLab CI.

### Changed
- `Cargo.toml`: updated the version metadata to `0.2.0`.

### Fixed
- `Cargo.toml`: typo in the description metadata.

## [0.1.0] 2022-02-05

### Added
- GNU GPLv3 licence.
- `Cargo.toml`: authors, description and licence metadata.

### Changed
- Updated `.gitignore` to ignore the `/.idea/` directory.
