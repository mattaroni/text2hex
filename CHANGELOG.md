# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-05-18

### Changed
- Switch from printing the hexadecimal bytes all at once to printing them in
  chunks; this should make the program more memory efficient.

## [0.2.0] - 2025-05-14

### Added
- "Help" option (`-h`/`--help`) to the binary: prints a useful "help" message,
  explaining what the binary does and what command-line arguments it takes.
- "Version" option (`-V`/`--version`) to the binary: prints the version number
  of the downloaded binary.

### Changed
- Switch from using `std::env` to [clap](https://docs.rs/clap/latest/clap/)
  for handling command-line arguments.

## [0.1.1] - 2025-05-14

### Fixed
- Patched an issue where characters in the `00`-`0f` byte range were rendered
  in hexadecimal without the leading `0` digit. For example, the line feed
  character `U+000a` would be represented as `a` instead of `0a`. This is now
  fixed.

## [0.1.0] - 2025-05-13

### Added
- Basic functionality.
- Essential documentation (readme, license, changelog, & cargo file).
- Availability on [GitHub](https://github.com/mattaroni/text2hex).

[0.2.1]: https://github.com/mattaroni/text2hex/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/mattaroni/text2hex/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/mattaroni/text2hex/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/mattaroni/text2hex/releases/tag/v0.1.0
