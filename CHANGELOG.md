# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-05

### Added

- Interactive console menu with language selection (Russian / English)
- Automatic detection of video files next to the binary
  (`mp4`, `mov`, `avi`, `mkv`, `webm`, `m4v`, `flv`, `wmv`)
- Four ASCII character sets to choose from:
  - `0 1`
  - `: ; 0 1`
  - `*`
  - all together (`0 1 : ; *`)
- Output format selection: **MP4** (H.264) and **GIF** (palette-optimized)
- Frame extraction via `ffmpeg` with per-frame brightness → ASCII conversion
- Custom bitmap rendering of ASCII frames using an embedded TTF font
  (`Datekt-Black`) — no external font dependencies at runtime
- Automatic `ffmpeg` download on first run (sidecar), fully portable afterwards
- Progress bars for each stage: frame extraction, ASCII rendering, video encoding
- CLI flags:
  - `vidpix --version` — print version
  - `vidpix --help` — print help text (localized)
- Embedded build metadata: git short hash and target triple in `--help` output
- Unit tests for charset mapping and ASCII conversion (12 tests)
- GitHub Actions release workflow producing binaries for:
  - Windows (x86_64-pc-windows-msvc)
  - Linux (x86_64-unknown-linux-gnu)
  - macOS Intel (x86_64-apple-darwin)
  - macOS Apple Silicon (aarch64-apple-darwin)

### Notes

- This is the first public release. API and CLI may change before `1.0.0`.
- Requires Windows 10+, Linux, or macOS 10.15+.
- First run requires internet access to download `ffmpeg` (~80 MB).

[Unreleased]: https://github.com/datekt/vidpix/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/datekt/vidpix/releases/tag/v0.1.0