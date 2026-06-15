<div align="center">

# ⚡ Strom

**High-performance, concurrent audiobook converter for the terminal.**

[![Rust](https://img.shields.io/badge/rust-v1.75+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/aaron-official/strom/actions/workflows/ci.yml/badge.svg)](https://github.com/aaron-official/strom/actions/workflows/ci.yml)

[Features](#-features) • [Installation](#-installation) • [Quick Start](#-quick-start) • [Performance](#-performance)

---

</div>

Strom is a lightning-fast, concurrent Text User Interface (TUI) application built in Rust. It specializes in converting bulky `.m4b` audiobook files into optimized `.mp3` format while leveraging multi-core processing to finish the job in a fraction of the time.

## ✨ Features

- 🖥️ **Interactive TUI:** A polished, keyboard-driven interface built with `ratatui`.
- 🚀 **Turbo-charged Conversion:** Powered by `tokio` to orchestrate multiple FFmpeg processes in parallel.
- 📂 **Smart Output:** Automatically organizes conversions into a dedicated `converted/` folder.
- 🎞️ **Chapter Awareness:** Intelligently extracts embedded chapter metadata for split-mode conversion.
- 📊 **Real-time Progress:** Live progress bars (both overall and per-file) with second-by-second updates.
- 🦀 **Memory Safe:** Built with the safety and speed of Rust.

## 📦 Installation

### Prerequisites

You must have **FFmpeg** installed on your system.
- **macOS:** `brew install ffmpeg`
- **Ubuntu/Debian:** `sudo apt install ffmpeg`
- **Windows:** `choco install ffmpeg`

### Build from Source

```bash
git clone https://github.com/aaron-official/strom.git
cd strom
cargo build --release
```

The binary will be available at `./target/release/strom`.

## 🚀 Quick Start

1. **Launch Strom** in any directory containing `.m4b` files:
   ```bash
   strom
   ```
2. **Navigate:** Use `↑`/`↓` or `j`/`k` to move through the file list.
3. **Select:** Press `Space` to select/deselect files for conversion.
4. **Convert:** Press `Enter` to start the batch.
5. **Confirm:** Select `[Yes]` to begin the high-speed conversion process.

## 🏎️ Performance

Strom is designed for efficiency. Unlike traditional converters that process files sequentially, Strom utilizes a concurrent worker pool:

| File Size | Sequential Time | Strom (8-core) |
| :--- | :--- | :--- |
| 500 MB (Single) | ~10 mins | **~3 mins** |
| 5 GB (Split) | ~2 hours | **~15-20 mins** |

## 🛠️ Tech Stack

- **UI Framework:** [Ratatui](https://github.com/ratatui-org/ratatui)
- **Runtime:** [Tokio](https://tokio.rs/)
- **Backend:** [FFmpeg](https://ffmpeg.org/)
- **Terminal Backend:** [Crossterm](https://github.com/crossterm-rs/crossterm)

## 🗺️ Roadmap

- [ ] Support for `.m4a` and `.aac` inputs.
- [ ] Customizable bitrate and quality settings.
- [ ] Metadata tagging (copying cover art and tags to MP3).
- [ ] Directory-wide recursive scanning.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

---
<div align="center">
Built with 🦀 by [aaron-official](https://github.com/aaron-official/)
</div>
