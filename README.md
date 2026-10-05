![VIDPIX demo](assets/images/vidpix-test.gif)

# VIDPIX

Turn any video into ASCII animation — MP4 or GIF, right in your terminal.

VIDPIX is a single portable binary that converts a video file into a stylized animation made entirely out of characters like 0 1 : ; *. Drop it next to your videos, run it, pick a few options — get a ready-to-share .mp4 or .gif in seconds. No installers, no Python, no ffmpeg setup. Just one file.

![VIDPIX banner](assets/images/vidpix-banner.png)

[![Release](https://img.shields.io/github/v/release/datekt/vidpix?style=flat-square)](https://github.com/datekt/vidpix/releases)
[![License](https://img.shields.io/github/license/datekt/vidpix?style=flat-square)](https://github.com/datekt/vidpix/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)

[Русская версия](README.ru.md) · [Changelog](CHANGELOG.md)

## Features

- One binary, three platforms — Windows, Linux, macOS (Intel + Apple Silicon)
- Portable by design — works from any folder, scans next to itself
- Fully offline after first run — downloads ffmpeg once, then never again
- Two output formats — MP4 (H.264) and GIF (optimized palette)
- Four character sets — 0 1, : ; 0 1, *, or all together
- Bilingual UI — English and Russian out of the box
- Custom font rendering — crisp ASCII frames with an embedded typeface
- Zero runtime dependencies on the target machine

## Quick start

### 1. Download

Grab the archive for your OS from the [latest release](https://github.com/datekt/vidpix/releases):

| Platform | File |
|---|---|
| Windows (x64) | vidpix-windows-x86_64.zip |
| Linux (x64) | vidpix-linux-x86_64.tar.gz |
| macOS (Intel) | vidpix-macos-x86_64.tar.gz |
| macOS (Apple Silicon) | vidpix-macos-arm64.tar.gz |

### 2. Put it next to your videos

    MyVideos/
      vidpix.exe
      clip.mp4
      concert.mov

### 3. Run it

    vidpix

### 4. Answer the menu

    Language? / Язык?
    > Русский
      English
      Cancel / Отмена

    Привет! Какое видео конвентируем?
    > clip.mp4
      concert.mov
      Отмена

    Отличный выбор! Из каких символов хотите анимацию?
    > 0 и 1
      : ; 0 1
      *
      Все вместе (0 1 : ; *)
      Отмена

    В каком формате сохранить?
    > MP4 (видео)
      GIF (анимация)
      Отмена

### 5. Get your result

    clip_vidpix.mp4

That's it.

## Supported input

| Container | Extension |
|---|---|
| MPEG-4 | .mp4, .m4v |
| QuickTime | .mov |
| Matroska | .mkv |
| WebM | .webm |
| AVI | .avi |
| Flash Video | .flv |
| Windows Media | .wmv |

Any codec that ffmpeg can decode will work.

## CLI

    vidpix              launch the interactive menu
    vidpix --help       print help text
    vidpix --version    print version

The interactive menu is the main path — there is nothing to remember.

## How it works

1. ffmpeg extracts frames, scales them to 80 columns, converts to grayscale
2. Each pixel's brightness is mapped to a character from the chosen set
3. Every ASCII frame is rendered into a PNG bitmap using an embedded font
4. ffmpeg encodes the PNG sequence into H.264 MP4 or an optimized GIF

Every stage runs locally. No upload, no telemetry, no cloud.

## Requirements

- Windows 10+, Linux (glibc 2.31+), or macOS 10.15+
- About 5 MB for the binary itself
- About 80 MB free space for ffmpeg (auto-downloaded on first run)
- Internet access on first run only — afterwards the tool works completely offline

## Build from source

Requires Rust 1.75+.

    git clone https://github.com/datekt/vidpix.git
    cd vidpix
    cargo build --release

The binary lands in target/release/vidpix (or vidpix.exe on Windows).

Run the test suite:

    cargo test

## Project structure

    src/
      core/          video decoding, ASCII conversion, rendering, encoding
      i18n/          localized strings (EN / RU)
      ui/            interactive menu and progress bars
      utils/         path helpers
      args.rs        command-line flag parsing
      cli.rs         top-level flow
      main.rs        entry point

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create a feature branch: git checkout -b feature/amazing-thing
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## License

Licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements

- FFmpeg — the backbone of every stage
- ffmpeg-sidecar — seamless ffmpeg delivery
- image + imageproc — frame rendering
- ab_glyph — font rasterization
- dialoguer + indicatif — terminal UX

Made with Rust by [datekt](https://github.com/datekt)