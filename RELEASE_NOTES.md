# VIDPIX v0.1.0

First public release.

Turn any video into ASCII animation — MP4 or GIF, right in your terminal.
One portable binary, no installers, no setup.

## Downloads

| Platform | File |
|---|---|
| Windows (x64) | `vidpix-windows-x86_64.zip` |
| Linux (x64) | `vidpix-linux-x86_64.tar.gz` |
| macOS (Intel) | `vidpix-macos-x86_64.tar.gz` |
| macOS (Apple Silicon) | `vidpix-macos-arm64.tar.gz` |

## Quick start

1. Download the archive for your OS
2. Extract the binary next to your videos
3. Run `vidpix`
4. Pick a video, a character set (`0 1`, `: ; 0 1`, `*`, all together) and a format (MP4 / GIF)
5. Get `yourvideo_vidpix.mp4` or `.gif` next to the source file

## What's inside

- Interactive console menu with language selection (Russian / English)
- Automatic video detection next to the binary
- Supports `mp4`, `mov`, `avi`, `mkv`, `webm`, `m4v`, `flv`, `wmv`
- Four ASCII character sets
- MP4 (H.264) and GIF (optimized palette) output
- Custom bitmap rendering with an embedded typeface
- Automatic `ffmpeg` download on first run — fully portable afterwards
- Progress bars for every stage
- `vidpix --version` / `vidpix --help`

## Notes

- Windows 10+, Linux (glibc 2.31+), macOS 10.15+
- First run needs internet access to fetch `ffmpeg` (~80 MB)
- This is `0.1.0` — API and CLI may change before `1.0.0`

---

See [CHANGELOG.md](CHANGELOG.md) for the full list of changes.